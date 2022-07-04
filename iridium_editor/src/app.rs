use iridium_ecs::World;
use iridium_graphics::Renderer2DSystem;
use winit::{
    window::Window,
    event::*,
};

use crate::ui::*;

/// The main application state.
pub struct App {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,

    egui_state: EguiState,
    pub ui_state: UiState,

    renderer_2d_system: Renderer2DSystem,
}

impl App {
    /// Create a new instance of App.
    pub async fn new(window: &Window) -> Self {
        // Get the size of the window.
        let screen_size = {
            let size = window.inner_size();
            (size.width, size.height)
        };

        // Initialize the surface.
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        // Initialize the device.
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

        // Configure the surface.
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: screen_size.0,
            height: screen_size.1,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &surface_config);

        // Initialize the UI state.
        let egui_state = EguiState::new(
            &device,
            surface_config.format,
            window,
        );
        let ui_state = UiState::new(
            ScreenRect::new(1. / 3., 0., 2. / 3., 0.6),
            screen_size,
            1.2,
        );

        // Create the renderer 2D system.
        let renderer_2d_system = Renderer2DSystem {};

        Self {
            surface,
            device,
            queue,
            surface_config,

            egui_state,
            ui_state,

            renderer_2d_system,
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

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.egui_state.winit.on_event(&self.egui_state.context, event);
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

    /// Render everything to the screen.
    pub fn render(&mut self, window: &Window, world: &mut World) {
        // Calculate the viewport_rect in logical and physical coordinates.
        let viewport_rect_logical = self.ui_state.viewport_rect.egui_logical(self.ui_state.screen_size.0, self.ui_state.screen_size.1, self.ui_state.scale_factor);
        let viewport_rect_physical = self.ui_state.viewport_rect.egui_logical(self.ui_state.screen_size.0, self.ui_state.screen_size.1, 1.);

        // Modify the input.
        let mut input = self.egui_state.winit.take_egui_input(window);
        input.pixels_per_point = Some(window.scale_factor() as f32 * self.ui_state.scale_factor);
        input.events
            .iter_mut()
            .for_each(|event| match event {
                egui::Event::PointerMoved(position) => {
                    // If a button is being held down,
                    // I still want to be able to move controls
                    if viewport_rect_logical.distance_to_pos(*position) < 5.
                    && !self.egui_state.context.input().pointer.any_down() {
                        *event = egui::Event::PointerGone;
                    }
                },
                egui::Event::PointerButton { pos, .. } => {
                    if viewport_rect_logical.contains(*pos) {
                        *event = egui::Event::PointerGone;
                    }
                },
                _ => (),
            });

        // Begin the UI frame.
        self.egui_state.context.begin_frame(input);

        // Draw the UI.
        self.egui_state.render_panels(&mut self.ui_state, world);

        // End the UI frame.
        let egui_output = self.egui_state.context.end_frame();

        // Give winit the UI output.
        self.egui_state.winit.handle_platform_output(window, &self.egui_state.context, egui_output.platform_output);

        // Get the output of the UI frame.
        let paint_jobs = self.egui_state.context.tessellate(egui_output.shapes);

        // Create the screen descriptor.
        let screen_descriptor = egui_latest_wgpu_backend::ScreenDescriptor {
            physical_width: self.ui_state.screen_size.0,
            physical_height: self.ui_state.screen_size.1,
            scale_factor: window.scale_factor() as f32 * self.ui_state.scale_factor,
        };

        // Upload the resources to the GPU.
        self.egui_state.rpass.add_textures(&self.device, &self.queue, &egui_output.textures_delta).unwrap();
        self.egui_state.rpass.remove_textures(egui_output.textures_delta).unwrap();
        self.egui_state.rpass.update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);

        // Get the surface texture to render to.
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create the command encoder to send commands to the GPU.
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // Create the render pass.
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

            // Set the viewport to the viewport_rect.
            render_pass.set_viewport(
                viewport_rect_physical.min.x,
                viewport_rect_physical.min.y,
                viewport_rect_physical.width(),
                viewport_rect_physical.height(),
                0.,
                1.,
            );

            // Run the rendering system for the entities in the world.
            self.renderer_2d_system.run(&world.entities, 0., &mut render_pass, &self.queue);

            // Set the viewport to the entire surface.
            render_pass.set_viewport(
                0.,
                0.,
                self.ui_state.screen_size.0 as f32,
                self.ui_state.screen_size.1 as f32,
                0.,
                1.,
            );

            // Render the UI.
            self.egui_state.rpass.execute_with_renderpass(
                &mut render_pass,
                &paint_jobs,
                &screen_descriptor,
            ).unwrap();
        }

        // Submit the command encoder.
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present the frame.
        output.present();
    }
}
