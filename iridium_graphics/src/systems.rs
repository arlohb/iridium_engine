use std::cmp::Ordering;

use iridium_assets::Assets;
use iridium_ecs::{
    query,
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    Entities, Name, Transform,
};
use iridium_ecs_macros::{ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use crate::{Camera, CameraGpuData, Renderable2D};

/// The state for `Renderer2DSystem`.
#[derive(ComponentTrait, InspectorUi, Default)]
pub struct Renderer2DState {
    /// The name of the active camera.
    ///
    /// This is used instead of the entity id until I implement entity drag and drop.
    ///
    /// This is the name property of Camera, not the entity name.
    pub active_camera: String,
    /// The camera GPU data.
    #[hidden]
    pub camera_gpu_data: Option<CameraGpuData>,
}

impl ComponentStorage for Renderer2DState {
    fn from_stored(mut stored: StoredComponent, _assets: &iridium_assets::Assets) -> Option<Self> {
        Some(Self {
            active_camera: stored.get("active_camera")?,
            camera_gpu_data: None,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Renderer2DState".to_string(),
            fields: fast_map! {
                "active_camera" => StoredComponentField::new(self.active_camera.to_string(), true),
            },
        }
    }
}

/// Draws entities to the screen.
///
/// This system is much more specialised, so doesn't impl the `System` trait.
pub struct Renderer2DSystem;

impl Renderer2DSystem {
    /// Runs the system.
    #[allow(clippy::too_many_arguments)]
    // This should probably be changed at some point.
    #[allow(clippy::too_many_lines)]
    pub fn run(
        entities: & /* 'entities */ Entities,
        _assets: &Assets,
        device: &wgpu::Device,
        render_pass: & /* 'rpass */ mut wgpu::RenderPass,
        queue: &wgpu::Queue,
        viewport_rect_physical: &egui::Rect,
        size_pixels: (f32, f32),
        editor_camera: Option<&mut Camera>,
    ) {
        puffin::profile_function!();

        let components = {
            puffin::profile_scope!("Setup");

            // Set the viewport to the viewport_rect.
            render_pass.set_viewport(
                viewport_rect_physical.min.x,
                viewport_rect_physical.min.y,
                viewport_rect_physical.width(),
                viewport_rect_physical.height(),
                0.,
                1.,
            );

            let state = entities.get::<Renderer2DState>();

            let camera = {
                puffin::profile_scope!("Camera data update");

                if let Some(camera) = editor_camera {
                    *camera.viewport_size.x_mut() = size_pixels.0;
                    *camera.viewport_size.y_mut() = size_pixels.1;

                    camera
                } else {
                    let mut active_camera = None;

                    for (camera,) in query!(entities, [mut Camera; ]) {
                        *camera.viewport_size.x_mut() = size_pixels.0;
                        *camera.viewport_size.y_mut() = size_pixels.1;

                        if camera.name == state.active_camera {
                            active_camera = Some(camera);
                        }
                    }

                    match active_camera {
                        Some(camera) => camera,
                        None => return,
                    }
                }
            };

            let components = {
                puffin::profile_scope!("Getting components");

                let mut components = {
                    puffin::profile_scope!("Query");

                    query!(entities, [mut Renderable2D; Transform, Name]).collect::<Vec<_>>()
                };

                {
                    puffin::profile_scope!("Sorting");

                    // Sort entities by their z position, then name if z is equal.
                    // The name shouldn't be used to order sprites, it's just to prevent z-fighting.
                    components.sort_by(|(_, a_t, a_name), (_, b_t, b_name)| {
                        // Sort by z-index.
                        let ordering = a_t
                            .position
                            .z()
                            .partial_cmp(&b_t.position.z())
                            .unwrap_or(Ordering::Equal);

                        match ordering {
                            Ordering::Equal => {
                                // Sort by name.
                                a_name.name.cmp(&b_name.name)
                            }
                            _ => ordering,
                        }
                    });
                }

                components
            };

            {
                puffin::profile_scope!("Write camera to GPU");

                let camera_gpu_data = state
                    .camera_gpu_data
                    .get_or_insert_with(|| CameraGpuData::new(device));

                let camera_gpu_data = unsafe {
                    #[allow(clippy::useless_transmute)]
                    std::mem::transmute::<
                        & /* 'entities */ mut CameraGpuData,
                        & /* 'rpass */ mut CameraGpuData,
                    >(camera_gpu_data)
                };

                queue.write_buffer(&camera_gpu_data.buffer, 0, &camera.as_bytes());

                render_pass.set_bind_group(2, &camera_gpu_data.bind_group, &[]);
            }

            components
        };

        puffin::profile_scope!("Rendering");

        for (renderable_2d, transform, _) in components {
            // Check if runtime data is initialised, if not, create it.
            renderable_2d.create_live_data(device);

            // Extend the lifetime of renderable_2d for render_pass.set_pipeline.
            // This is safe because it's only used as the pipeline for the duration of this function.
            #[allow(clippy::useless_transmute)]
            let r2d = unsafe {
                std::mem::transmute::<& /* 'entities */ Renderable2D, & /* 'rpass */ Renderable2D>(
                    renderable_2d,
                )
            };

            let transform_bytes = transform
                .position
                .as_bytes::<16>()
                .into_iter()
                .chain(transform.scale.as_bytes::<12>().into_iter())
                .chain(transform.rotation.to_le_bytes().into_iter())
                .collect::<Vec<u8>>();

            queue.write_buffer(
                &r2d.vertex_shader_buffers
                    .as_ref()
                    .unwrap_or_else(|| unreachable!())[0],
                0,
                &transform_bytes,
            );

            render_pass.set_pipeline(&r2d.material.render_pipeline);
            render_pass.set_bind_group(
                0,
                r2d.vertex_shader_bind_group
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
                &[],
            );
            render_pass.set_bind_group(
                1,
                r2d.fragment_shader_bind_group
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
                &[],
            );
            render_pass.set_vertex_buffer(
                0,
                r2d.vertex_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!())
                    .slice(..),
            );
            render_pass.set_index_buffer(
                r2d.index_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!())
                    .slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(
                0..r2d.index_count.unwrap_or_else(|| unreachable!()),
                0,
                0..1,
            );
        }
    }
}
