use iridium_assets::Assets;
use iridium_core::{InputState, LogState};
use iridium_ecs::World;
use iridium_graphics::Renderer2DSystem;
use iridium_maths::VecN;
use winit::{event::WindowEvent, window::Window};

/// Manages the rendering stuff.
pub struct App {
    /// The wgpu surface.
    pub surface: wgpu::Surface,
    /// The wgpu device.
    pub device: wgpu::Device,
    /// The wgpu queue.
    pub queue: wgpu::Queue,
    /// The wgpu surface configuration
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl App {
    /// Create a new instance of `App`.
    #[allow(clippy::future_not_send)]
    pub async fn new(window: &Window) -> Self {
        // Get the size of the window.
        let screen_size = {
            let size = window.inner_size();
            (size.width, size.height)
        };

        // Usually the backend will be Vulkan,
        // for some reason since wgpu 0.13 now default to GL.
        // GL is a lot slower, so just use anything except that.
        let backends = !wgpu::Backends::GL;

        // Initialize the surface.
        let instance = wgpu::Instance::new(backends);
        let surface = unsafe { instance.create_surface(window) };

        // Initialize the device.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to get adapter");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Failed to get device");

        // Configure the surface.
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: screen_size.0,
            height: screen_size.1,
            // Vsync should be used in the future,
            // but I need to see fps above 60 while debugging performance.
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);

        Self {
            surface,
            device,
            queue,
            surface_config,
        }
    }

    /// Updates the screen size.
    pub fn resize(&mut self, new_size: (u32, u32)) {
        // On windows the window size is 0 when minimized.
        if new_size.0 > 0 && new_size.1 > 0 {
            // Update the surface configuration.
            self.surface_config.width = new_size.0;
            self.surface_config.height = new_size.1;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    /// Handles window input and passes it to the game.
    pub fn input(&mut self, world: &mut World, event: &WindowEvent) {
        let input_state = world.entities.get::<InputState>();
        let log = world.entities.get::<LogState>();

        match event {
            WindowEvent::Resized(physical_size) => {
                self.resize((physical_size.width, physical_size.height));
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize((new_inner_size.width, new_inner_size.height));
            }
            WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(virtual_keycode),
                        ..
                    },
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    input_state.key_pressed((*virtual_keycode).into());
                }
                winit::event::ElementState::Released => {
                    input_state.key_released((*virtual_keycode).into());
                }
            },
            WindowEvent::ModifiersChanged(_) => {
                log.warning("Modifiers in runtime not implemented yet");
            }

            WindowEvent::CursorMoved { position, .. } => {
                input_state.mouse_position = VecN::new([position.x as f32, position.y as f32]);
            }
            WindowEvent::MouseWheel { .. } => {
                log.warning("Mouse wheel in runtime not implemented yet");
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => {
                    input_state.mouse_button_pressed((*button).into());
                }
                winit::event::ElementState::Released => {
                    input_state.mouse_button_released((*button).into());
                }
            },
            _ => {}
        }
    }

    /// Render everything to the screen.
    pub fn render(&mut self, world: &mut World, assets: &Assets) {
        // Get the surface texture to render to.
        let output = self
            .surface
            .get_current_texture()
            .expect("Failed to get output texture");
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create the command encoder to send commands to the GPU.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // Create the render pass.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            // Run the rendering system for the entities in the world.
            Renderer2DSystem::run(
                &world.entities,
                assets,
                &self.device,
                &mut render_pass,
                &self.queue,
                None,
                (
                    self.surface_config.width as f32,
                    self.surface_config.height as f32,
                ),
                None,
            );
        }

        // Submit the command encoder.
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present the frame.
        output.present();
    }
}
