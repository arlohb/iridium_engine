use iridium_ecs::Entities;
use iridium_graphics::Renderer2DSystem;
use winit::{
    window::Window,
    event::*,
};

pub struct App {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_size: winit::dpi::PhysicalSize<u32>,

    egui_winit_state: egui_winit::State,
    egui_context: egui::Context,
    egui_rpass: egui_latest_wgpu_backend::RenderPass,
    ui: egui_demo_lib::DemoWindows,

    renderer_2d_system: Renderer2DSystem,
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

        let egui_winit_state = egui_winit::State::new(4096, window);
        let egui_context = egui::Context::default();
        let egui_rpass = egui_latest_wgpu_backend::RenderPass::new(
            &device,
            surface_config.format,
            1,
        );
        let ui = egui_demo_lib::DemoWindows::default();

        Self {
            surface,
            device,
            queue,
            surface_config,
            surface_size,

            egui_winit_state,
            egui_context,
            egui_rpass,
            ui,

            renderer_2d_system: Renderer2DSystem {},
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
        self.egui_winit_state.on_event(&self.egui_context, event);
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

    pub fn render(&mut self, window: &Window, entities: &Entities) -> Result<(), wgpu::SurfaceError> {
        let scale_factor = 0.8;
        let (screen_rect_physical, _screen_space_rect) = {
            let min_x = 0.;
            let min_y = 0.;
            let max_x = 1.;
            let max_y = 0.75;

            (
                egui::Rect {
                    min: egui::emath::pos2(
                        min_x * self.surface_size.width as f32,
                        min_y * self.surface_size.height as f32,
                    ),
                    max: egui::emath::pos2(
                        max_x * self.surface_size.width as f32,
                        max_y * self.surface_size.height as f32,
                    ),
                },
                egui::Rect {
                    min: egui::emath::pos2(
                        min_x,
                        min_y,
                    ),
                    max: egui::emath::pos2(
                        max_x,
                        max_y,
                    ),
                },
            )
        };

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Modify the input
        let mut input = self.egui_winit_state.take_egui_input(window);
        input.screen_rect = Some(screen_rect_physical);
        input.pixels_per_point = Some(window.scale_factor() as f32 * scale_factor);
        input.events
            .iter_mut()
            .for_each(|event| match event {
                egui::Event::PointerMoved(position) => {
                    // If a button is being held down,
                    // I still want to be able to move controls
                    if !screen_rect_physical.contains(*position)
                    && !self.egui_context.input().pointer.any_down() {
                        *event = egui::Event::PointerGone;
                    }
                },
                egui::Event::PointerButton { pos, .. } => {
                    if !screen_rect_physical.contains(*pos) {
                        *event = egui::Event::PointerGone;
                    }
                },
                _ => (),
            });

        // Begin to draw the UI frame.
        self.egui_context.begin_frame(input);

        // Draw the demo application.
        self.ui.ui(&self.egui_context);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let egui_output = self.egui_context.end_frame();
        let paint_jobs = self.egui_context.tessellate(egui_output.shapes);

        self.egui_winit_state.handle_platform_output(window, &self.egui_context, egui_output.platform_output);

        // Upload all resources for the GPU.
        let screen_descriptor = egui_latest_wgpu_backend::ScreenDescriptor {
            physical_width: screen_rect_physical.width() as u32,
            physical_height: screen_rect_physical.height() as u32,
            scale_factor: window.scale_factor() as f32 * scale_factor,
        };

        self.egui_rpass
            .add_textures(&self.device, &self.queue, &egui_output.textures_delta)
            .unwrap();
        self.egui_rpass.remove_textures(egui_output.textures_delta).unwrap();
        self.egui_rpass.update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);

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

            self.renderer_2d_system.run(entities, 0., &mut render_pass, &self.queue);

            render_pass.set_viewport(
                screen_rect_physical.min.x,
                screen_rect_physical.min.y,
                screen_rect_physical.width(),
                screen_rect_physical.height(),
                0.,
                1.,
            );

            self.egui_rpass.execute_with_renderpass(
                &mut render_pass,
                &paint_jobs,
                &screen_descriptor,
            ).unwrap();
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
