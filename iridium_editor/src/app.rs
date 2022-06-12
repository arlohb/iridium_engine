use iridium_ecs::World;
use iridium_graphics::Renderer2DSystem;
use winit::{
    window::Window,
    event::*,
};

use crate::ui::*;

pub struct App {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_size: winit::dpi::PhysicalSize<u32>,

    egui_state: EguiState,
    egui_panels: Vec<EguiPanel>,
    ui_state: UiState,

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

        let egui_state = EguiState::new(window, 1.2);
        let egui_panels = vec![
            EguiPanel::new(
                &device,
                surface_config.format,
                crate::ui::panels::EntitiesList,
                ScreenRect::new(
                    0.,
                    0.,
                    0.15,
                    1.,
                )
            ),
            // EguiPanel::new(
            //     &device,
            //     surface_config.format,
            //     egui_demo_lib::DemoWindows::default(),
            //     ScreenRect::new(
            //         0.,
            //         0.,
            //         1.,
            //         0.8,
            //     )
            // ),
        ];
        let ui_state = UiState::new();

        Self {
            surface,
            device,
            queue,
            surface_config,
            surface_size,

            egui_state,
            egui_panels,
            ui_state,

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

    pub fn update(&mut self) {}

    pub fn render(&mut self, window: &Window, world: &mut World) -> Result<(), wgpu::SurfaceError> {
        let screen_rect_physical = self.egui_panels[0].screen_rect.rect_physical(self.surface_size.width, self.surface_size.height);
        let screen_rect_logical = self.egui_panels[0].screen_rect.rect_logical(self.surface_size.width, self.surface_size.height, self.egui_state.scale_factor);

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Modify the input
        let mut input = self.egui_state.winit.take_egui_input(window);
        input.screen_rect = Some(screen_rect_logical);
        input.pixels_per_point = Some(window.scale_factor() as f32 * self.egui_state.scale_factor);
        input.events
            .iter_mut()
            .for_each(|event| match event {
                egui::Event::PointerMoved(position) => {
                    // If a button is being held down,
                    // I still want to be able to move controls
                    if !screen_rect_logical.contains(*position)
                    && !self.egui_state.context.input().pointer.any_down() {
                        *event = egui::Event::PointerGone;
                    }
                },
                egui::Event::PointerButton { pos, .. } => {
                    if !screen_rect_logical.contains(*pos) {
                        *event = egui::Event::PointerGone;
                    }
                },
                _ => (),
            });

        // Begin to draw the UI frame.
        self.egui_state.context.begin_frame(input);

        // Draw the demo application.
        self.egui_panels[0].ui.render(&self.egui_state.context, &mut self.ui_state, world);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let egui_output = self.egui_state.context.end_frame();
        let paint_jobs = self.egui_state.context.tessellate(egui_output.shapes);

        self.egui_state.winit.handle_platform_output(window, &self.egui_state.context, egui_output.platform_output);

        // Upload all resources for the GPU.
        let screen_descriptor = egui_latest_wgpu_backend::ScreenDescriptor {
            physical_width: screen_rect_physical.width() as u32,
            physical_height: screen_rect_physical.height() as u32,
            scale_factor: window.scale_factor() as f32 * self.egui_state.scale_factor,
        };

        self.egui_panels[0].rpass
            .add_textures(&self.device, &self.queue, &egui_output.textures_delta)
            .unwrap();
        self.egui_panels[0].rpass.remove_textures(egui_output.textures_delta).unwrap();
        self.egui_panels[0].rpass.update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descriptor);

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

            render_pass.set_viewport(
                self.surface_size.width as f32 * 0.15,
                self.surface_size.height as f32 * 0.,
                self.surface_size.width as f32 * 0.85,
                self.surface_size.height as f32 * 1.,
                0.,
                1.,
            );

            self.renderer_2d_system.run(&world.entities, 0., &mut render_pass, &self.queue);

            render_pass.set_viewport(
                screen_rect_physical.min.x,
                screen_rect_physical.min.y,
                screen_rect_physical.width(),
                screen_rect_physical.height(),
                0.,
                1.,
            );

            self.egui_panels[0].rpass.execute_with_renderpass(
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
