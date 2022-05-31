use std::sync::Arc;

use crate::*;

pub struct Material {
    pub vertex: Arc<Shader>,
    pub fragment: Arc<Shader>,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Material {
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        vertex: Arc<Shader>,
        fragment: Arc<Shader>,
    ) -> Material {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&vertex.bind_group_layout],
            push_constant_ranges: &[],
        });
    
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.shader,
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
                module: &fragment.shader,
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

        Material {
            vertex,
            fragment,
            render_pipeline,
        }
    }
}

pub struct MaterialInstance {
    pub material: Arc<Material>,
    pub vertex_bind_group: wgpu::BindGroup,
    pub fragment_bind_group: wgpu::BindGroup,
}

impl MaterialInstance {
    pub fn new(
        device: &wgpu::Device,
        material: Arc<Material>,
        vertex_resources: Vec<wgpu::BindingResource>,
        fragment_resources: Vec<wgpu::BindingResource>,
    ) -> MaterialInstance {
        MaterialInstance {
            material: material.clone(),
            vertex_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &material.vertex.bind_group_layout,
                entries: {
                    let binding_index = 0;
                    &vertex_resources.into_iter().map(|binding_resource| wgpu::BindGroupEntry {
                        binding: binding_index,
                        resource: binding_resource,
                    }).collect::<Vec<_>>()
                },
                label: None,
            }),
            fragment_bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &material.fragment.bind_group_layout,
                entries: {
                    let binding_index = 0;
                    &fragment_resources.into_iter().map(|binding_resource| wgpu::BindGroupEntry {
                        binding: binding_index,
                        resource: binding_resource,
                    }).collect::<Vec<_>>()
                },
                label: None,
            }),
        }
    }
}
