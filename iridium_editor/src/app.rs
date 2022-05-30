use std::borrow::Cow;

use inline_spirv::include_spirv;
use winit::{
    window::Window,
    event::*,
};

use crate::object::Object;

pub struct App {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    pub surface_size: winit::dpi::PhysicalSize<u32>,

    objects: Vec<Object>,
}

impl App {
    pub async fn new(window: &Window) -> Self {
        let surface_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        }, None).await.unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: surface_size.width,
            height: surface_size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &surface_config);

        let objects = vec![
            Object::new(
                &device,
                surface_config.format,
                &crate::shaders::vert_shader(&device),
                &crate::shaders::frag_shader(&device),
                &[
                    [-1.0, -1.0, 0.0],
                    [-1.0,  0.0, 0.0],
                    [ 0.0,  0.0, 0.0],
                    [ 0.0, -1.0, 0.0],
                ],
                &[
                    0, 3, 2,
                    0, 2, 1,
                ],
            ),
            Object::new(
                &device,
                surface_config.format,
                &crate::shaders::vert_shader(&device),
                &device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::SpirV(Cow::Borrowed(
                        include_spirv!("src/frag_2.hlsl", frag, hlsl, entry="fs_main")
                    )),
                }),
                &[
                    [-0.4, -1., 0.0],
                    [0.6, -1., 0.0],
                    [0.1, 0., 0.0],
                ],
                &[
                    0, 1, 2,
                ],
            ),
        ];

        Self {
            surface,
            device,
            queue,
            surface_config,
            surface_size,
            objects,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(key),
                    ..
                },
                ..
            } => {
                *key != VirtualKeyCode::Escape
            },
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            for object in &mut self.objects {
                render_pass.set_pipeline(&object.render_pipeline);
                render_pass.set_vertex_buffer(0, object.vertex_buffer.slice(..));
                render_pass.set_index_buffer(object.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..object.index_count, 0, 0..1);
            }
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
