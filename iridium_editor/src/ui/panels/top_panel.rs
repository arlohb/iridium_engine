use crate::{ui::PanelUi, play_state::PlayState};

pub struct TopPanel;

impl PanelUi for TopPanel {
    fn render(&mut self, context: &egui::Context, ui_state: &mut crate::ui::UiState, _world: &mut iridium_ecs::World) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            let max_y_logical = ui.max_rect().max.y + ui.spacing().item_spacing.y;
            let max_y_physical = max_y_logical * ui_state.scale_factor;
            let max_y_screen = max_y_physical / ui_state.screen_size.1 as f32;
            ui_state.viewport_rect.min_y = max_y_screen;

            ui.add_space(1.);
            ui.columns(3, |columns| {
                if let [menus, buttons, stats] = columns {
                    menus.horizontal(|menus| {
                        menus.menu_button("File", |ui| { ui.label("File")});
                        menus.menu_button("Edit", |ui| { ui.label("Edit")});
                        menus.menu_button("View", |ui| { ui.label("View")});
                        menus.menu_button("About", |ui| { ui.label("About")});
                    });

                    egui::Frame::none()
                        .fill(buttons.style().visuals.widgets.inactive.bg_fill)
                        .rounding(3.)
                        .show(buttons, |buttons| {
                            buttons.horizontal(|buttons| {
                                buttons.style_mut().spacing.button_padding = egui::vec2(0., 0.);

                                buttons.add_space(6.);

                                if buttons.add_enabled(
                                    matches!(ui_state.play_state(), PlayState::Stop | PlayState::Pause),
                                    egui::Button::new("▶").frame(false),
                                ).clicked() { ui_state.play(); }
                                if buttons.add_enabled(
                                    matches!(ui_state.play_state(), PlayState::Play),
                                    egui::Button::new("⏸").frame(false),
                                ).clicked() { ui_state.pause(); }
                                if buttons.add_enabled(
                                    matches!(ui_state.play_state(), PlayState::Play | PlayState::Pause),
                                    egui::Button::new("■").frame(false),
                                ).clicked() { ui_state.stop(); }

                                buttons.add_space(1.);
                            });
                        });

                    stats.horizontal(|stats| {
                        stats.label("FPS: ");
                        stats.label("0");
                    });
                }
            });
        });
    }
}
