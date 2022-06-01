use std::sync::Arc;

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl From<ShaderType> for wgpu::ShaderStages {
    fn from(shader_type: ShaderType) -> Self {
        match shader_type {
            ShaderType::Vertex => wgpu::ShaderStages::VERTEX,
            ShaderType::Fragment => wgpu::ShaderStages::FRAGMENT,
        }
    }
}

pub struct Shader {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        shader_type: ShaderType,
        spirv: &[u32],
        inputs: Vec<wgpu::BindingType>,
    ) -> Shader {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &inputs
                .iter()
                .enumerate()
                .map(|(binding, binding_type)| {
                    wgpu::BindGroupLayoutEntry {
                        binding: binding as u32,
                        visibility: shader_type.into(),
                        ty: *binding_type,
                        count: None,
                    }
                }).collect::<Vec<_>>(),
            label: None,
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(spirv)),
        });

        Shader {
            bind_group_layout,
            shader,
        }
    }
}

pub struct ShaderData {
    pub buffers: Vec<Arc<wgpu::Buffer>>,
    pub bind_group: wgpu::BindGroup,
}
