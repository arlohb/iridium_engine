use egui::PointerButton;
use egui_winit::winit::{event_loop::EventLoop, window::Window};
use iridium_assets::Assets;
use iridium_ecs::World;

use super::{PanelUi, UiState};

/// Stores data about the UI while it is being rendered.
pub struct FrameData {
    pub paint_jobs: Option<Vec<egui::ClippedPrimitive>>,
    pub screen_descriptor: Option<egui_wgpu::renderer::ScreenDescriptor>,
    pub textures_delta: Option<egui::TexturesDelta>,
}

/// The rendering state of the editor UI.
pub struct EguiState {
    /// The egui context.
    pub context: egui::Context,
    /// The egui backend state.
    pub renderer: egui_wgpu::Renderer,
    /// The egui winit state.
    pub winit: egui_winit::State,
    /// The UI panels.
    pub panels: Vec<Box<dyn PanelUi>>,

    /// Mouse position in logical pixels.
    pub mouse_pos: egui::Pos2,

    frame_data: FrameData,
}

impl EguiState {
    /// Creates a new egui state.
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        event_loop: &EventLoop<()>,
    ) -> Self {
        // Create the egui context.
        let context = egui::Context::default();

        // Create the egui backend state.
        let rpass = egui_wgpu::Renderer::new(device, format, None, 1);

        // Create the winit state.
        let winit = egui_winit::State::new(event_loop);

        // Create the UI panels.
        let panels: Vec<Box<dyn PanelUi>> = vec![
            Box::new(super::panels::TopPanel),
            Box::new(super::panels::EntitiesList::new()),
            Box::new(super::panels::ComponentsList),
            Box::new(super::panels::BottomPanel::new()),
            Box::new(super::panels::ViewportPanel),
        ];

        Self {
            context,
            renderer: rpass,
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
    #[allow(clippy::too_many_lines)]
    pub fn input(
        &mut self,
        window: &Window,
        viewport_rect_logical: egui::Rect,
        scale_factor: f32,
        ui_state: &mut UiState,
    ) -> (Vec<egui::Event>, egui::RawInput) {
        puffin::profile_function!();

        let mut input = self.winit.take_egui_input(window);
        // input.pixels_per_point = Some(window.scale_factor() as f32 * scale_factor);
        input.pixels_per_point = Some(scale_factor);

        // The events that the game should handle.
        let mut game_events = Vec::new();

        input.events = input
            .events
            .into_iter()
            .filter_map(|event| match event {
                egui::Event::PointerMoved(position) => {
                    self.mouse_pos = (position.to_vec2() * 1.2).to_pos2();

                    if let Some(pan_start) = &mut ui_state.pan_start {
                        let mut movement = position - *pan_start;
                        *pan_start = position;

                        movement = movement * ui_state.camera.scale / (scale_factor * 0.06);

                        movement.x /= ui_state.screen_size.0 as f32;
                        movement.y /= ui_state.screen_size.1 as f32;

                        *ui_state.camera.position.x_mut() -= movement.x;
                        *ui_state.camera.position.y_mut() += movement.y;
                    }

                    // If the UI is doing something with the mouse,
                    // I still want to be able to move controls
                    if viewport_rect_logical.distance_to_pos(position) < 5.
                        && !self.context.wants_pointer_input()
                    {
                        return Some(egui::Event::PointerGone);
                    }

                    Some(egui::Event::PointerMoved(position))
                }
                // False positive when order matters.
                #[allow(clippy::match_same_arms)]
                egui::Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    modifiers,
                } => match (
                    button,
                    pressed,
                    viewport_rect_logical.contains(pos),
                    self.context.wants_pointer_input() | self.context.wants_keyboard_input(),
                ) {
                    // If middle mouse btn is unpressed, and we're panning.
                    (PointerButton::Middle, false, _, _) if ui_state.pan_start.is_some() => {
                        // Stop panning.
                        ui_state.pan_start = None;
                        // Don't send to egui.
                        None
                    }
                    // If middle mouse btn is pressed in the viewport.
                    (PointerButton::Middle, true, true, _) => {
                        // Start panning.
                        ui_state.pan_start = Some(pos);
                        // Don't send to egui.
                        None
                    }
                    // If any mouse btn is pressed/unpressed in the viewport and egui wants it.
                    (_, _, true, true) => Some(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    }),
                    // If any mouse btn is pressed/unpressed in the viewport and egui doesn't want it.
                    (_, _, true, false) => {
                        // Send the input to the game.
                        game_events.push(egui::Event::PointerButton {
                            pos,
                            button,
                            pressed,
                            modifiers,
                        });
                        None
                    }
                    // If any mouse btn is pressed/unpressed outside the viewport.
                    _ => Some(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    }),
                },
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
                egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                    repeat,
                } if !repeat => {
                    if viewport_rect_logical.contains(self.mouse_pos) {
                        game_events.push(egui::Event::Key {
                            key,
                            pressed,
                            modifiers,
                            repeat,
                        });
                        return None;
                    }

                    Some(egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                        repeat,
                    })
                }
                event => Some(event),
            })
            .collect::<Vec<_>>();

        (game_events, input)
    }

    /// Draws the UI.
    pub fn draw(
        &mut self,
        window: &Window,
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
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [ui_state.screen_size.0, ui_state.screen_size.1],
            pixels_per_point: ui_state.scale_factor,
        };

        self.frame_data = FrameData {
            paint_jobs: Some(paint_jobs),
            screen_descriptor: Some(screen_descriptor),
            textures_delta: Some(full_output.textures_delta),
        };
    }

    /// Uploads the resources to the GPU.
    pub fn upload_ui(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        puffin::profile_function!();

        for (id, delta) in &self
            .frame_data
            .textures_delta
            .as_ref()
            .expect("Textures delta not yet created")
            .set
        {
            self.renderer.update_texture(device, queue, *id, delta);
        }

        for id in &self
            .frame_data
            .textures_delta
            .as_ref()
            .expect("Textures delta not yet created")
            .free
        {
            self.renderer.free_texture(id);
        }

        self.renderer.update_buffers(
            device,
            queue,
            encoder,
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
        self.renderer.render(
            render_pass,
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
}
