use iridium_assets::Assets;
use iridium_ecs::{storage::*, Component, ComponentDefault, ComponentFieldUi};
use iridium_ecs_macros::ComponentTrait;
use iridium_map_utils::fast_map;
use iridium_maths::VecN;
use wgpu::util::DeviceExt;

use crate::*;

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
#[derive(ComponentTrait)]
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
    pub viewport_size: VecN<2>,
}

impl Camera {
    const GPU_BYTES: usize = 32;

    /// Converts the camera to bytes to be sent to the GPU.
    pub fn as_bytes(&self) -> [u8; Self::GPU_BYTES] {
        let mut bytes = [0; Self::GPU_BYTES];

        bytes[0..8].copy_from_slice(&self.position.as_bytes::<8>());
        bytes[8..12].copy_from_slice(&self.min_depth.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.max_depth.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.rotation.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.scale.to_le_bytes());
        bytes[24..28].copy_from_slice(
            &(self.viewport_size.x() as f32 / self.viewport_size.y() as f32).to_le_bytes(),
        );

        bytes
    }
}

impl ComponentDefault for Camera {
    fn create() -> Component {
        Component::new(Camera {
            name: "".to_string(),
            position: VecN::new([0.0, 0.0]),
            min_depth: 0.0,
            max_depth: 1.0,
            rotation: 0.0,
            scale: 1.0,
            viewport_size: VecN::new([1., 1.]),
        })
    }
}

impl ComponentStorage for Camera {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Camera {
            name: stored.get("name")?,
            position: stored.get("position")?.parse().ok()?,
            min_depth: stored.get("min_depth")?.parse().ok()?,
            max_depth: stored.get("max_depth")?.parse().ok()?,
            rotation: stored.get("rotation")?.parse().ok()?,
            scale: stored.get("scale")?.parse().ok()?,
            viewport_size: VecN::new([1., 1.]),
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Camera".to_string(),
            fields: fast_map! {
                "position" => StoredComponentField::NonString(self.position.to_string()),
                "min_depth" => StoredComponentField::NonString(self.min_depth.to_string()),
                "max_depth" => StoredComponentField::NonString(self.max_depth.to_string()),
                "rotation" => StoredComponentField::NonString(self.rotation.to_string()),
                "scale" => StoredComponentField::NonString(self.scale.to_string()),
            },
        }
    }
}

/// Describes how an entity should be drawn to the screen.
#[derive(ComponentTrait)]
pub struct Renderable2D {
    /// The material used.
    #[hidden]
    pub material: MaterialInstance,
    /// The vertex buffer.
    #[hidden]
    pub vertex_buffer: wgpu::Buffer,
    /// The index buffer.
    #[hidden]
    pub index_buffer: wgpu::Buffer,
    /// The number of vertices.
    #[hidden]
    pub index_count: u32,
}

impl ComponentStorage for Renderable2D {
    fn from_stored(_stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        None
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "Renderable2D".to_string(),
            fields: fast_map! {},
        }
    }
}

impl Renderable2D {
    /// Creates a new `Renderable2D` from a `MaterialInstance` and a `Mesh`.
    pub fn new(device: &wgpu::Device, material_instance: MaterialInstance, mesh: &Mesh) -> Self {
        let vertices_bytes = mesh
            .vertices
            .iter()
            .flat_map(|v| v.as_bytes())
            .collect::<Vec<u8>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: vertices_bytes.as_slice(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_bytes = mesh
            .indices
            .iter()
            .flat_map(|v: &u32| v.to_le_bytes())
            .collect::<Vec<u8>>();

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: index_bytes.as_slice(),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = mesh.indices.len() as u32;

        Renderable2D {
            material: material_instance,
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}
