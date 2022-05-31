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
    vertices: &[[f32; 3]],
    indices: &[u16],
) -> HashMap<String, Box<dyn std::any::Any>> {
    let vertices_bytes = vertices
        .iter()
        .flatten()
        .flat_map(|v: &f32| v.to_le_bytes())
        .collect::<Vec<u8>>();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: vertices_bytes.as_slice(),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_bytes = indices
        .iter()
        .flat_map(|v: &u16| v.to_le_bytes())
        .collect::<Vec<u8>>();

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: index_bytes.as_slice(),
        usage: wgpu::BufferUsages::INDEX,
    });

    let index_count = indices.len() as u32;

    fast_map_any! {
        "material" => material_instance,
        "vertex_buffer" => vertex_buffer,
        "index_buffer" => index_buffer,
        "index_count" => index_count
    }
}