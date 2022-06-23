use iridium_ecs_macros::ComponentTrait;
use wgpu::util::DeviceExt;

use crate::*;

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

impl Renderable2D {
    /// Creates a new `Renderable2D` from a `MaterialInstance` and a `Mesh`.
    pub fn new(
        device: &wgpu::Device,
        material_instance: MaterialInstance,
        mesh: &Mesh,
    ) -> Self {
        let vertices_bytes = mesh.vertices
            .iter()
            .flat_map(|v| v.as_bytes())
            .collect::<Vec<u8>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: vertices_bytes.as_slice(),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_bytes = mesh.indices
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
