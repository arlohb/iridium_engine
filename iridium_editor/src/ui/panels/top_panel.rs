use iridium_assets::Assets;

use crate::{play_state::PlayState, ui::PanelUi, FrameHistoryState};

pub struct TopPanel;

impl PanelUi for TopPanel {
    fn name(&self) -> &'static str {
        "TopPanel"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut crate::ui::UiState,
        world: &mut iridium_ecs::World,
        assets: &Assets,
    ) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            ui.add_space(1.);
            ui.columns(3, |columns| {
                if let [menus, buttons, stats] = columns {
                    menus.horizontal(|ui| {
                        ui.menu_button("File", |ui| {
                            if let Some(scene) = &ui_state.open_scene {
                                if ui.button("Save").clicked() {
                                    world.save(scene);
                                    ui.close_menu();
                                }
                            } else {
                                ui.add_enabled(false, egui::Button::new("Save"));
                            }
                        });
                        ui.menu_button("Edit", |ui| ui.label("Edit"));
                        ui.menu_button("View", |ui| ui.label("View"));
                        ui.menu_button("About", |ui| ui.label("About"));
                    });

                    egui::Frame::none()
                        .fill(buttons.style().visuals.widgets.inactive.bg_fill)
                        .rounding(3.)
                        .show(buttons, |ui| {
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.button_padding = egui::vec2(0., 0.);

                                ui.add_space(6.);

                                if ui
                                    .add_enabled(
                                        ui_state.play_state() != PlayState::Play,
                                        egui::Button::new("▶").frame(false),
                                    )
                                    .clicked()
                                {
                                    if ui_state.play_state() == PlayState::Stop {
                                        world.save("temp.json5");
                                    }
                                    ui_state.play();
                                }

                                if ui
                                    .add_enabled(
                                        ui_state.play_state() == PlayState::Play,
                                        egui::Button::new("⏸").frame(false),
                                    )
                                    .clicked()
                                {
                                    ui_state.pause();
                                }

                                if ui
                                    .add_enabled(
                                        ui_state.play_state() != PlayState::Stop,
                                        egui::Button::new("■").frame(false),
                                    )
                                    .clicked()
                                {
                                    world
                                        .load("temp.json5", assets)
                                        .expect("Save file disappeared");
                                    std::fs::remove_file("temp.json5")
                                        .expect("Failed to remove temp file");
                                    ui_state.stop();
                                }

                                ui.add_space(1.);
                            });
                        });

                    stats.horizontal(|ui| {
                        ui.label(format!(
                            "FPS: {:.1}",
                            world.entities.get::<FrameHistoryState>().average_fps()
                        ));
                        ui.add_space(15.);
                        ui.label(format!(
                            "Entities: {}",
                            world.entities.entity_count::<iridium_ecs::Name>()
                        ));
                        ui.add_space(15.);
                        ui.label(format!(
                            "Sprites: {}",
                            world
                                .entities
                                .entity_count::<iridium_graphics::Renderable2D>()
                        ));
                    });
                }
            });
        });
    }
}
