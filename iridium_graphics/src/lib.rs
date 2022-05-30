use wgpu::util::DeviceExt;
use hashbrown::HashMap;

use iridium_ecs::*;

pub fn component_types() -> HashMap<String, ComponentType> {
    create_component_types! {
        "Renderable2D" => fast_map! {
            "render_pipeline" => "wgpu::RenderPipeline",
            "vertex_buffer" => "wgpu::Buffer",
            "index_buffer" => "wgpu::Buffer",
            "index_count" => "u32"
        }
    }
}

pub fn create_renderable_2d(
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    vertices: &[[f32; 3]],
    indices: &[u16],
) -> HashMap<String, Box<dyn std::any::Any>> {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: vertex_shader,
            entry_point: "vs_main",
            buffers: &[
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3]
                }
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: fragment_shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let vertices_bytes = vertices
        .iter()
        .flatten()
        .map(|v: &f32| v.to_le_bytes())
        .flatten()
        .collect::<Vec<u8>>();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: vertices_bytes.as_slice(),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_bytes = indices
        .iter()
        .map(|v: &u16| v.to_le_bytes())
        .flatten()
        .collect::<Vec<u8>>();

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: index_bytes.as_slice(),
        usage: wgpu::BufferUsages::INDEX,
    });

    let index_count = indices.len() as u32;

    fast_map_any! {
        "render_pipeline" => render_pipeline,
        "vertex_buffer" => vertex_buffer,
        "index_buffer" => index_buffer,
        "index_count" => index_count
    }
}

pub struct Renderer2DSystem {}

impl Renderer2DSystem {
    pub fn run(&mut self, entities: &Entities, _delta_time: f64, render_pass: &mut wgpu::RenderPass) {
        for [renderable_2d] in entities.query(["Renderable2D"]) {
            let renderable_2d = unsafe {
                let ptr = &renderable_2d as &Component as *const Component;
                &*ptr
            };
            render_pass.set_pipeline(&renderable_2d.get::<wgpu::RenderPipeline>("render_pipeline"));
            render_pass.set_vertex_buffer(0, renderable_2d.get::<wgpu::Buffer>("vertex_buffer").slice(..));
            render_pass.set_index_buffer(renderable_2d.get::<wgpu::Buffer>("index_buffer").slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..*renderable_2d.get::<u32>("index_count"), 0, 0..1);
        }
    }
}
