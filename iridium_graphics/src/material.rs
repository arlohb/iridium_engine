use iridium_assets::{Asset, AssetBox};
use iridium_ecs_macros::HasStableTypeId;

use crate::{CameraGpuData, Shader, Vertex};

/// Describes how an entity should be drawn to the screen.
#[derive(HasStableTypeId)]
pub struct Material {
    /// The vertex shader.
    pub vertex_shader: AssetBox<Shader>,
    /// The fragment shader.
    pub fragment_shader: AssetBox<Shader>,
    /// The render pipeline to use.
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Asset for Material {}

impl Material {
    /// Creates a new material.
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        vertex_shader: AssetBox<Shader>,
        camera_gpu_data: &CameraGpuData,
        fragment_shader: AssetBox<Shader>,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &vertex_shader.bind_group_layout,
                &fragment_shader.bind_group_layout,
                &camera_gpu_data.bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader.shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader.shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        Self {
            vertex_shader,
            fragment_shader,
            render_pipeline,
        }
    }
}
