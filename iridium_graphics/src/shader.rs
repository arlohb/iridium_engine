use std::sync::Arc;

use iridium_assets::{Asset, AssetBox};
use iridium_ecs_macros::HasStableTypeId;
use wgpu::util::DeviceExt;

use crate::Texture;

/// The type of a shader.
///
/// Either `Vertex` or `Fragment`.
///
/// Into can be called on this type to get the corresponding `wgpu::ShaderStages`.
#[derive(Clone, Copy)]
pub enum ShaderType {
    /// A vertex shader.
    Vertex,
    /// A fragment shader.
    Fragment,
}

impl From<ShaderType> for wgpu::ShaderStages {
    fn from(shader_type: ShaderType) -> Self {
        match shader_type {
            ShaderType::Vertex => Self::VERTEX,
            ShaderType::Fragment => Self::FRAGMENT,
        }
    }
}

/// A shader.
#[derive(HasStableTypeId)]
pub struct Shader {
    /// The inputs to the shader.
    pub inputs: Vec<ShaderInput>,
    /// The bind group layout of the inputs.
    pub bind_group_layout: wgpu::BindGroupLayout,
    /// The wgpu shader module.
    pub shader: wgpu::ShaderModule,
}

impl Asset for Shader {}

impl Shader {
    /// Creates a new shader from the spirv bytes.
    #[must_use]
    pub fn new(
        device: &wgpu::Device,
        shader_type: ShaderType,
        spirv: &[u32],
        inputs: Vec<ShaderInput>,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &inputs
                .iter()
                .enumerate()
                .map(|(binding, input)| wgpu::BindGroupLayoutEntry {
                    binding: binding.try_into().expect("Too many bindings"),
                    visibility: shader_type.into(),
                    ty: match input {
                        ShaderInput::Transform => wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        ShaderInput::Texture(texture) => texture.texture_binding_type,
                        ShaderInput::Sampler(texture) => texture.sampler_binding_type,
                    },
                    count: None,
                })
                .collect::<Vec<_>>(),
            label: None,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(spirv)),
        });

        Self {
            inputs,
            bind_group_layout,
            shader,
        }
    }

    /// Creates data needed to render with this shader.
    #[must_use]
    pub fn create_live_data(
        &self,
        device: &wgpu::Device,
    ) -> (Vec<Arc<wgpu::Buffer>>, wgpu::BindGroup) {
        let transform_buffer = if self
            .inputs
            .iter()
            .any(|input| matches!(input, ShaderInput::Transform))
        {
            Some(Arc::new(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    contents: &[0u8; 32],
                },
            )))
        } else {
            None
        };

        let mut buffers: Vec<Arc<wgpu::Buffer>> = vec![];

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &self
                .inputs
                .iter()
                .enumerate()
                .map(|(binding, input)| wgpu::BindGroupEntry {
                    binding: binding.try_into().expect("Too many bindings"),
                    resource: match input {
                        ShaderInput::Transform => transform_buffer
                            .as_ref()
                            .unwrap_or_else(|| unreachable!())
                            .as_entire_binding(),
                        ShaderInput::Texture(texture) => {
                            wgpu::BindingResource::TextureView(&texture.view)
                        }
                        ShaderInput::Sampler(texture) => {
                            wgpu::BindingResource::Sampler(&texture.sampler)
                        }
                    },
                })
                .collect::<Vec<_>>(),
        });

        if let Some(transform_buffer) = transform_buffer {
            buffers.insert(0, transform_buffer);
        }

        (buffers, bind_group)
    }
}

/// An input to a shader.
pub enum ShaderInput {
    /// Transform data about an object.
    Transform,
    /// A texture.
    Texture(AssetBox<Texture>),
    /// A texture sampler.
    Sampler(AssetBox<Texture>),
}
