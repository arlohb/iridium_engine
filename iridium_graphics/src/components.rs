use iridium_map_utils::*;
use wgpu::util::DeviceExt;
use hashbrown::HashMap;

use iridium_ecs::*;

use crate::*;

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Renderable2D" => fast_map! {
            "material" => "MaterialInstance",
            "vertex_buffer" => "wgpu::Buffer",
            "index_buffer" => "wgpu::Buffer",
            "index_count" => "u32"
        }
    }
}

pub fn create_renderable_2d(
    device: &wgpu::Device,
    material_instance: MaterialInstance,
    mesh: &Mesh,
) -> HashMap<String, Box<dyn std::any::Any>> {
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

    fast_map_any! {
        "material" => material_instance,
        "vertex_buffer" => vertex_buffer,
        "index_buffer" => index_buffer,
        "index_count" => index_count
    }
}
