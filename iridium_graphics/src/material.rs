use std::sync::Arc;

use iridium_assets::Asset;
use wgpu::util::DeviceExt;

use crate::{CameraGpuData, Shader, ShaderData, Vertex};

/// Describes how an entity should be drawn to the screen.
pub struct Material {
    /// The vertex shader.
    pub vertex: Asset<Shader>,
    /// The fragment shader.
    pub fragment: Asset<Shader>,
    /// The render pipeline to use.
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Material {
    /// Creates a new material.
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        vertex: Asset<Shader>,
        camera_gpu_data: &CameraGpuData,
        fragment: Asset<Shader>,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &vertex.bind_group_layout,
                &fragment.bind_group_layout,
                &camera_gpu_data.bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment.shader,
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
            vertex,
            fragment,
            render_pipeline,
        }
    }
}

/// An instance of a `Material`.
pub struct MaterialInstance {
    /// The material ths is an instance of.
    pub material: Asset<Material>,
    /// The shader data for the vertex shader.
    pub vertex_data: ShaderData,
    /// The shader data for the fragment shader.
    pub fragment_data: ShaderData,
}

impl MaterialInstance {
    /// Creates a new material instance.
    #[must_use]
    pub fn new(
        device: &wgpu::Device,
        material: Asset<Material>,
        vertex_buffers: Vec<Arc<wgpu::Buffer>>,
        vertex_resources: Vec<wgpu::BindingResource>,
        fragment_buffer: Vec<Arc<wgpu::Buffer>>,
        fragment_resources: Vec<wgpu::BindingResource>,
    ) -> Self {
        let buffer = Arc::new(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &[0u8; 32],
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        );

        let vertex_buffers = std::iter::once(buffer.clone())
            .chain(vertex_buffers.into_iter())
            .collect();
        let vertex_resources =
            std::iter::once(buffer.as_entire_binding()).chain(vertex_resources.into_iter());

        Self {
            vertex_data: ShaderData {
                buffers: vertex_buffers,
                bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &material.vertex.bind_group_layout,
                    entries: &vertex_resources
                        .enumerate()
                        .map(|(binding, binding_resource)| wgpu::BindGroupEntry {
                            binding: binding.try_into().expect("Too many bindings"),
                            resource: binding_resource,
                        })
                        .collect::<Vec<_>>(),
                    label: None,
                }),
            },
            fragment_data: ShaderData {
                buffers: fragment_buffer,
                bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &material.fragment.bind_group_layout,
                    entries: &fragment_resources
                        .into_iter()
                        .enumerate()
                        .map(|(binding, binding_resource)| wgpu::BindGroupEntry {
                            binding: binding.try_into().expect("Too many bindings"),
                            resource: binding_resource,
                        })
                        .collect::<Vec<_>>(),
                    label: None,
                }),
            },
            material,
        }
    }
}
