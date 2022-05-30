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
    pub bind_group: wgpu::BindGroup,
    pub shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        shader_type: ShaderType,
        spirv: &[u32],
        inputs: Vec<(wgpu::BindingType, wgpu::BindingResource)>,
    ) -> Shader {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: {
                let binding_index = 0;
                &inputs.iter().map(|(binding_type, _)| wgpu::BindGroupLayoutEntry {
                    binding: binding_index,
                    visibility: shader_type.into(),
                    ty: *binding_type,
                    count: None,
                }).collect::<Vec<_>>()
            },
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: {
                let binding_index = 0;
                &inputs.into_iter().map(|(_, binding_resource)| wgpu::BindGroupEntry {
                    binding: binding_index,
                    resource: binding_resource,
                }).collect::<Vec<_>>()
            },
            label: None,
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(spirv)),
        });

        Shader {
            bind_group_layout,
            bind_group,
            shader,
        }
    }
}
