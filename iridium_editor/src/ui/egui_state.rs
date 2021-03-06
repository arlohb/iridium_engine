use egui::PointerButton;
use iridium_assets::Assets;
use iridium_ecs::World;
use winit::window::Window;

use super::{PanelUi, UiState};

/// Stores data about the UI while it is being rendered.
pub struct FrameData {
    pub paint_jobs: Option<Vec<egui::ClippedPrimitive>>,
    pub screen_descriptor: Option<egui_latest_wgpu_backend::ScreenDescriptor>,
    pub textures_delta: Option<egui::TexturesDelta>,
}

/// The rendering state of the editor UI.
pub struct EguiState {
    /// The egui context.
    pub context: egui::Context,
    /// The egui backend state.
    pub rpass: egui_latest_wgpu_backend::RenderPass,
    /// The egui winit state.
    pub winit: egui_winit::State,
    /// The UI panels.
    pub panels: Vec<Box<dyn PanelUi>>,

    frame_data: FrameData,
    mouse_pos: egui::Pos2,
}

impl EguiState {
    /// Creates a new egui state.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, window: &Window) -> Self {
        // Create the egui context.
        let context = egui::Context::default();

        // Create the egui backend state.
        let rpass = egui_latest_wgpu_backend::RenderPass::new(device, format, 1);

        // Create the winit state.
        let winit = egui_winit::State::new(4096, window);

        // Create the UI panels.
        let panels: Vec<Box<dyn PanelUi>> = vec![
            Box::new(super::panels::TopPanel),
            Box::new(super::panels::EntitiesList::new()),
            Box::new(super::panels::ComponentsList),
            Box::new(super::panels::BottomPanel::new()),
        ];

        Self {
            context,
            rpass,
            winit,
            panels,
            frame_data: FrameData {
                paint_jobs: None,
                screen_descriptor: None,
                textures_delta: None,
            },
            mouse_pos: egui::Pos2::new(0., 0.),
        }
    }

    /// Renders all the panels.
    pub fn render_panels(&mut self, ui_state: &mut UiState, world: &mut World, assets: &Assets) {
        puffin::profile_function!();

        for panel in &mut self.panels {
            puffin::profile_scope!(panel.name());
            panel.render(&self.context, ui_state, world, assets);
        }
    }

    /// Handles the input from winit.
    ///
    /// This modifies the input before the caller sends to egui.
    pub fn input(
        &mut self,
        window: &winit::window::Window,
        viewport_rect_logical: egui::Rect,
        scale_factor: f32,
        ui_state: &mut UiState,
    ) -> egui::RawInput {
        puffin::profile_function!();

        let mut input = self.winit.take_egui_input(window);
        input.pixels_per_point = Some(window.scale_factor() as f32 * scale_factor);
        input.events = input
            .events
            .into_iter()
            .filter_map(|event| match event {
                egui::Event::PointerMoved(position) => {
                    self.mouse_pos = position;

                    if let Some(pan_start) = &mut ui_state.pan_start {
                        let mut movement = position - *pan_start;
                        *pan_start = position;

                        movement = movement * ui_state.camera.scale / (scale_factor * 0.06);

                        movement.x /= ui_state.screen_size.0 as f32;
                        movement.y /= ui_state.screen_size.1 as f32;

                        *ui_state.camera.position.x_mut() -= movement.x;
                        *ui_state.camera.position.y_mut() += movement.y;
                    }

                    // If a button is being held down,
                    // I still want to be able to move controls
                    if viewport_rect_logical.distance_to_pos(position) < 5.
                        && !self.context.input().pointer.any_down()
                    {
                        return Some(egui::Event::PointerGone);
                    }

                    Some(egui::Event::PointerMoved(position))
                }
                egui::Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    modifiers,
                } => {
                    if button == PointerButton::Middle && !pressed {
                        ui_state.pan_start = None;
                    }

                    if viewport_rect_logical.contains(pos) {
                        if button == PointerButton::Middle && pressed {
                            ui_state.pan_start = Some(pos);
                        }

                        return None;
                    }

                    Some(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    })
                }
                egui::Event::Scroll(scroll) => {
                    if ui_state.pan_start.is_some()
                        || viewport_rect_logical.contains(self.mouse_pos)
                    {
                        let scroll_speed = 0.05;

                        if scroll.y > 0. {
                            ui_state.camera.scale /= scroll.y * scroll_speed;
                        } else {
                            ui_state.camera.scale *= scroll.y.abs() * scroll_speed;
                        }

                        ui_state.camera.scale = ui_state.camera.scale.max(0.01);

                        return None;
                    }

                    Some(egui::Event::Scroll(scroll))
                }
                event => Some(event),
            })
            .collect::<Vec<_>>();

        input
    }

    /// Draws the UI.
    pub fn draw(
        &mut self,
        window: &winit::window::Window,
        input: egui::RawInput,
        ui_state: &mut UiState,
        world: &mut World,
        assets: &Assets,
    ) {
        puffin::profile_function!();

        // Begin the UI frame.
        self.context.begin_frame(input);

        // Draw the UI.
        self.render_panels(ui_state, world, assets);

        puffin::profile_scope!("UI draw");

        // End the UI frame.
        let full_output = self.context.end_frame();

        // Give winit the UI output.
        self.winit
            .handle_platform_output(window, &self.context, full_output.platform_output);

        // Get the output of the UI frame.
        let paint_jobs = self.context.tessellate(full_output.shapes);

        // Create the screen descriptor.
        let screen_descriptor = egui_latest_wgpu_backend::ScreenDescriptor {
            physical_width: ui_state.screen_size.0,
            physical_height: ui_state.screen_size.1,
            scale_factor: window.scale_factor() as f32 * ui_state.scale_factor,
        };

        self.frame_data = FrameData {
            paint_jobs: Some(paint_jobs),
            screen_descriptor: Some(screen_descriptor),
            textures_delta: Some(full_output.textures_delta),
        };
    }

    /// Uploads the resources to the GPU.
    pub fn upload_ui(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        puffin::profile_function!();

        self.rpass
            .add_textures(
                device,
                queue,
                self.frame_data
                    .textures_delta
                    .as_ref()
                    .expect("Textures delta has not been created yet"),
            )
            .expect("Failed to add texture to egui");
        self.rpass
            .remove_textures(
                self.frame_data
                    .textures_delta
                    .take()
                    .expect("Textures delta has not been created yet"),
            )
            .expect("Failed to remove texture from egui");
        self.rpass.update_buffers(
            device,
            queue,
            self.frame_data
                .paint_jobs
                .as_ref()
                .expect("Paint jobs have not been created yet"),
            self.frame_data
                .screen_descriptor
                .as_ref()
                .expect("Screen descriptor has not been created yet"),
        );
    }

    /// Render the drawn UI.
    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>, ui_state: &UiState) {
        puffin::profile_function!();

        // Set the viewport to the entire surface.
        render_pass.set_viewport(
            0.,
            0.,
            ui_state.screen_size.0 as f32,
            ui_state.screen_size.1 as f32,
            0.,
            1.,
        );

        // Render the UI.
        self.rpass
            .execute_with_renderpass(
                render_pass,
                self.frame_data
                    .paint_jobs
                    .as_ref()
                    .expect("Paint jobs have not been created yet"),
                self.frame_data
                    .screen_descriptor
                    .as_ref()
                    .expect("Screen descriptor has not been created yet"),
            )
            .expect("Failed to execute egui render pass");
    }
}
