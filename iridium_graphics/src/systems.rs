use iridium_ecs::{*, storage::*};
use iridium_ecs_macros::ComponentTrait;
use iridium_map_utils::fast_map;

use crate::*;

/// The state for Renderer2DSystem.
#[derive(ComponentTrait)]
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

impl ComponentDefault for Renderer2DState {
    fn create() -> Component {
        Component::new(Renderer2DState {
            active_camera: "".to_string(),
            camera_gpu_data: None,
        })
    }
}

impl ComponentStorage for Renderer2DState {
    fn from_stored(mut stored: StoredComponent, _assets: &iridium_assets::Assets) -> Option<Self> {
        Some(Renderer2DState {
            active_camera: stored.get("active_camera")?,
            camera_gpu_data: None,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Renderer2DState".to_string(),
            fields: fast_map! {
                "active_camera" => StoredComponentField::String(self.active_camera.to_string()),
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
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        let mut components = query!(entities, [; Transform, Renderable2D, Name]).collect::<Vec<_>>();

        // Sort entities by their z position, then name if z is equal.
        // The name shouldn't be used to order sprites, it's just to prevent z-fighting.
        components.sort_by(|(a_t, _, a_name), (b_t, _, b_name)| {
            // Sort by z-index.
            let ordering = a_t.position.z().partial_cmp(&b_t.position.z()).unwrap();

            // If z-index is equal, sort by name.
            if let std::cmp::Ordering::Equal = ordering {
                a_name.name.cmp(&b_name.name)
            } else {
                ordering
            }
        });

        let state = entities.get::<Renderer2DState>();

        let camera = query!(entities, [; Camera])
                .find(|(camera, )| camera.name == state.active_camera)
                .map(|(camera, )| camera);

        let camera = match camera {
            Some(camera) => camera,
            None => return,
        };

        let camera_gpu_data = state.camera_gpu_data.get_or_insert_with(|| {
            CameraGpuData::new(device)
        });

        let camera_gpu_data = unsafe { std::mem::transmute::<&mut CameraGpuData, &mut CameraGpuData>(camera_gpu_data) };

        queue.write_buffer(&camera_gpu_data.buffer, 0, &camera.as_bytes());

        render_pass.set_bind_group(2, &camera_gpu_data.bind_group, &[]);

        for (transform, renderable_2d, _) in components {
            // Extend the lifetime of renderable_2d for render_pass.set_pipeline.
            // This is safe because it's only used as the pipeline for the duration of this function.
            let renderable_2d = unsafe { std::mem::transmute::<&Renderable2D, &Renderable2D>(renderable_2d) };

            let material = &renderable_2d.material;

            let transform_bytes = transform.position.as_bytes::<16>().into_iter()
                .chain(transform.scale.as_bytes::<12>().into_iter())
                .chain(transform.rotation.to_le_bytes().into_iter())
                .collect::<Vec<u8>>();

            queue.write_buffer(
                &material.vertex_data.buffers[0],
                0,
                &transform_bytes,
            );

            render_pass.set_pipeline(&material.material.render_pipeline);
            render_pass.set_bind_group(0, &material.vertex_data.bind_group, &[]);
            render_pass.set_bind_group(1, &material.fragment_data.bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderable_2d.vertex_buffer.slice(..));
            render_pass.set_index_buffer(renderable_2d.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..renderable_2d.index_count, 0, 0..1);
        }
    }
}
