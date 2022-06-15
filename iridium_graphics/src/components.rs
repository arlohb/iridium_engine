use wgpu::util::DeviceExt;

use iridium_ecs::*;

use crate::*;

pub struct Renderable2D {
    pub material: MaterialInstance,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Renderable2D {
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

impl ComponentTrait for Renderable2D {
    fn type_name() -> &'static str { "Renderable2D" }
    fn dyn_type_name(&self) -> &'static str { "Renderable2D" }
}
