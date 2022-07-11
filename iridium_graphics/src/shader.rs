use std::sync::Arc;

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
pub struct Shader {
    /// The bind group layout of the inputs.
    pub bind_group_layout: wgpu::BindGroupLayout,
    /// The wgpu shader module.
    pub shader: wgpu::ShaderModule,
}

impl Shader {
    /// Creates a new shader from the spirv bytes.
    #[must_use]
    pub fn new(
        device: &wgpu::Device,
        shader_type: ShaderType,
        spirv: &[u32],
        inputs: &[wgpu::BindingType],
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &inputs
                .iter()
                .enumerate()
                .map(|(binding, binding_type)| wgpu::BindGroupLayoutEntry {
                    binding: binding.try_into().expect("Too many bindings"),
                    visibility: shader_type.into(),
                    ty: *binding_type,
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
            bind_group_layout,
            shader,
        }
    }
}

/// Stores data about a shader instance to be used in a `MaterialInstance`.
pub struct ShaderData {
    /// The buffers to be sent to the shader.
    pub buffers: Vec<Arc<wgpu::Buffer>>,
    /// The bind group to be sent to the shader.
    pub bind_group: wgpu::BindGroup,
}
