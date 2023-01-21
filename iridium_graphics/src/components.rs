use std::sync::Arc;

use iridium_assets::{Asset, Assets};
use iridium_ecs::storage::{ComponentStorage, StoredComponent, StoredComponentField};
use iridium_ecs_macros::{Component, ComponentStorage, InspectorUi};
use iridium_map_utils::fast_map;
use iridium_maths::VecN;
use wgpu::util::DeviceExt;

use crate::{Material, Mesh, Vertex};

/// Stores data about the camera to be used for the GPU.
pub struct CameraGpuData {
    /// The bind group layout.
    pub bind_group_layout: wgpu::BindGroupLayout,
    /// The bind group.
    pub bind_group: wgpu::BindGroup,
    /// The buffer.
    pub buffer: wgpu::Buffer,
}

impl CameraGpuData {
    /// Creates a new camera GPU data.
    #[must_use]
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: Camera::GPU_BYTES as u64,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            bind_group_layout,
            bind_group,
            buffer,
        }
    }
}

/// Describes the camera used to render the scene.
///
/// This is just a simple orthographic camera.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct Camera {
    /// The name.
    pub name: String,
    /// The position.
    #[drag_speed(0.05)]
    pub position: VecN<2>,
    /// The minimum depth to render.
    pub min_depth: f32,
    /// The maximum depth to render.
    pub max_depth: f32,
    /// The rotation.
    ///
    /// In radians.
    #[drag_speed(0.05)]
    pub rotation: f32,
    /// The scale.
    #[drag_speed(0.05)]
    pub scale: f32,
    /// The screen size.
    #[hidden]
    #[temporary(VecN::new([1., 1.]))]
    pub viewport_size: VecN<2>,
}

impl Camera {
    const GPU_BYTES: usize = 32;

    /// Converts the camera to bytes to be sent to the GPU.
    #[must_use]
    pub fn as_bytes(&self) -> [u8; Self::GPU_BYTES] {
        let mut bytes = [0; Self::GPU_BYTES];

        bytes[0..8].copy_from_slice(&self.position.as_bytes::<8>());
        bytes[8..12].copy_from_slice(&self.min_depth.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.max_depth.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.rotation.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.scale.to_le_bytes());
        bytes[24..28]
            .copy_from_slice(&(self.viewport_size.x() / self.viewport_size.y()).to_le_bytes());

        bytes
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: VecN::new([0.0, 0.0]),
            min_depth: 0.0,
            max_depth: 1.0,
            rotation: 0.0,
            scale: 1.0,
            viewport_size: VecN::new([1., 1.]),
        }
    }
}

/// Describes how an entity should be drawn to the screen.
#[derive(Component, InspectorUi)]
pub struct Renderable2D {
    /// The mesh used.
    ///
    /// Most of the time this will just be a quad.
    pub mesh: Asset<Mesh>,

    /// The material used.
    #[hidden]
    pub material: Asset<Material>,

    /// The buffers used by the vertex shader.
    #[hidden]
    pub vertex_shader_buffers: Option<Vec<Arc<wgpu::Buffer>>>,
    /// The bind group of the vertex shader.
    #[hidden]
    pub vertex_shader_bind_group: Option<wgpu::BindGroup>,
    /// The buffers used by the fragment shader.
    #[hidden]
    pub fragment_shader_buffers: Option<Vec<Arc<wgpu::Buffer>>>,
    /// The bind group of the fragment shader.
    #[hidden]
    pub fragment_shader_bind_group: Option<wgpu::BindGroup>,

    /// The vertex buffer.
    #[hidden]
    pub vertex_buffer: Option<wgpu::Buffer>,
    /// The index buffer.
    #[hidden]
    pub index_buffer: Option<wgpu::Buffer>,
    /// The number of vertices.
    #[hidden]
    pub index_count: Option<u32>,
}

impl ComponentStorage for Renderable2D {
    fn from_stored(mut stored: StoredComponent, assets: &Assets) -> Option<Self> {
        Some(Self {
            mesh: assets.get(&stored.get("mesh")?)?,

            material: assets.get(&stored.get("material")?)?,

            vertex_shader_buffers: None,
            vertex_shader_bind_group: None,
            fragment_shader_buffers: None,
            fragment_shader_bind_group: None,

            vertex_buffer: None,
            index_buffer: None,
            index_count: None,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Renderable2D".to_string(),
            fields: fast_map! {
                "mesh" => StoredComponentField::new(self.mesh.id.clone(), true),
                "material" => StoredComponentField::new(self.material.id.clone(), true),
            },
        }
    }
}

impl Renderable2D {
    /// Creates a new renderable.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(mesh: Asset<Mesh>, material: Asset<Material>) -> Self {
        Self {
            mesh,
            material,
            vertex_shader_buffers: None,
            vertex_shader_bind_group: None,
            fragment_shader_buffers: None,
            fragment_shader_bind_group: None,
            vertex_buffer: None,
            index_buffer: None,
            index_count: None,
        }
    }

    /// Creates the live data needed at runtime and in editor that isn't stored.
    pub fn create_live_data(&mut self, device: &wgpu::Device) {
        // The live data is all or nothing,
        // so if vertex_buffer is none, all the other fields should be none.
        if self.vertex_buffer.is_some() {
            return;
        }

        {
            let (buffers, bind_group) = self.material.vertex_shader.create_live_data(device);
            self.vertex_shader_buffers = Some(buffers);
            self.vertex_shader_bind_group = Some(bind_group);
        }

        {
            let (buffers, bind_group) = self.material.fragment_shader.create_live_data(device);
            self.fragment_shader_buffers = Some(buffers);
            self.fragment_shader_bind_group = Some(bind_group);
        }

        let vertices_bytes = self
            .mesh
            .vertices
            .iter()
            .flat_map(Vertex::as_bytes)
            .collect::<Vec<u8>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: vertices_bytes.as_slice(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_bytes = self
            .mesh
            .indices
            .iter()
            .flat_map(|v: &u32| v.to_le_bytes())
            .collect::<Vec<u8>>();

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: index_bytes.as_slice(),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = self
            .mesh
            .indices
            .len()
            .try_into()
            .expect("Index count too large");

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.index_count = Some(index_count);
    }
}
