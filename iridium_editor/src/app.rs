use egui_winit::winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use iridium_assets::Assets;
use iridium_ecs::World;
use iridium_graphics::Renderer2DSystem;

use crate::{
    play_state::PlayState,
    ui::{EguiState, ScreenRect, UiState},
};

/// The main application state.
pub struct App {
    /// The wgpu surface.
    surface: wgpu::Surface,
    /// The wgpu device.
    pub device: wgpu::Device,
    /// The wgpu queue.
    pub queue: wgpu::Queue,
    /// The wgpu surface configuration.
    pub surface_config: wgpu::SurfaceConfiguration,

    /// The egui rendering state.
    egui_state: EguiState,
    /// The state of the UI.
    pub ui_state: UiState,
}

impl App {
    /// Create a new instance of App.
    #[allow(clippy::future_not_send)]
    pub async fn new(window: &Window, event_loop: &EventLoop<()>) -> Self {
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

        // Initialize the UI state.
        let egui_state = EguiState::new(&device, surface_config.format, event_loop);
        let ui_state = UiState::new(ScreenRect::new(1. / 3., 0., 2. / 3., 0.6), screen_size, 1.2);

        Self {
            surface,
            device,
            queue,
            surface_config,

            egui_state,
            ui_state,
        }
    }

    /// Updates the screen size.
    pub fn resize(&mut self, new_size: (u32, u32)) {
        // On windows the window size is 0 when minimized.
        if new_size.0 > 0 && new_size.1 > 0 {
            // Update the screen size.
            self.ui_state.screen_size = new_size;

            // Update the surface configuration.
            self.surface_config.width = new_size.0;
            self.surface_config.height = new_size.1;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    /// Handle the window input.
    ///
    /// Returns true if egui handled the event,
    /// false if the event should be handled outside the app.
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.egui_state
            .winit
            .on_event(&self.egui_state.context, event);
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => *key != VirtualKeyCode::Escape,
            _ => false,
        }
    }

    /// Render everything to the screen.
    pub fn render(&mut self, window: &Window, world: &mut World, assets: &Assets) {
        puffin::profile_function!();

        // Calculate the viewport_rect in logical and physical coordinates.
        let viewport_rect_logical = self.ui_state.viewport_rect.egui_logical(
            self.ui_state.screen_size.0,
            self.ui_state.screen_size.1,
            self.ui_state.scale_factor,
        );
        let viewport_rect_physical = self.ui_state.viewport_rect.egui_logical(
            self.ui_state.screen_size.0,
            self.ui_state.screen_size.1,
            1.,
        );

        let input = self.egui_state.input(
            window,
            viewport_rect_logical,
            self.ui_state.scale_factor,
            &mut self.ui_state,
        );

        self.egui_state
            .draw(window, input, &mut self.ui_state, world, assets);
        self.egui_state.upload_ui(&self.device, &self.queue);

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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
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
                &viewport_rect_physical,
                (
                    self.ui_state.viewport_rect.width() * self.ui_state.screen_size.0 as f32,
                    self.ui_state.viewport_rect.height() * self.ui_state.screen_size.1 as f32,
                ),
                if let PlayState::Play = self.ui_state.play_state() {
                    None
                } else {
                    Some(&mut self.ui_state.camera)
                },
            );

            self.egui_state.render(&mut render_pass, &self.ui_state);
        }

        puffin::profile_scope!("Queue submit");

        // Submit the command encoder.
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present the frame.
        output.present();
    }
}
