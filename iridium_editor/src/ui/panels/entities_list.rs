use iridium_assets::Assets;
use iridium_ecs::{query, EntityCommand, Name, World};

use crate::ui::{PanelUi, UiState};

pub struct EntitiesList {
    name_filter: String,
}

impl EntitiesList {
    pub const fn new() -> Self {
        Self {
            name_filter: String::new(),
        }
    }
}

impl PanelUi for EntitiesList {
    fn name(&self) -> &'static str {
        "EntitiesList"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut UiState,
        world: &mut World,
        _assets: &Assets,
    ) {
        egui::SidePanel::left("entities_list").show(context, |ui| {
            let max_x_logical = ui.max_rect().max.x + ui.spacing().item_spacing.x;
            let max_x_physical = max_x_logical * ui_state.scale_factor;
            let max_x_screen = max_x_physical / ui_state.screen_size.0 as f32;
            ui_state.viewport_rect.min_x = max_x_screen;

            ui.add_space(6.);

            ui.menu_button("Add Entity", |ui| {
                if ui.button("Empty").clicked() {
                    world.entities.new_entity(None, "New Entity", vec![]);
                    ui.close_menu();
                }
            });

            ui.add_space(6.);

            ui.text_edit_singleline(&mut self.name_filter);

            ui.separator();

            egui::ScrollArea::new([false, true])
                .always_show_scroll(true)
                .auto_shrink([false, false])
                .max_width(f32::INFINITY)
                .show(ui, |ui| {
                    ui.add_space(10.);

                    for (id, Name { name }) in {
                        let mut entities = query!(world.entities, [; Name]).collect::<Vec<_>>();
                        entities.sort_by_key(|(_, name)| &name.name);
                        entities
                    } {
                        if name == "SystemState" {
                            continue;
                        }

                        if !name
                            .to_lowercase()
                            .contains(&self.name_filter.to_lowercase())
                        {
                            continue;
                        }

                        let mut rich_text = egui::RichText::new(name);

                        if let Some(selected_id) = ui_state.selected_entity {
                            if selected_id == id {
                                rich_text = rich_text.strong();
                            }
                        }

                        let label = ui.add(egui::Label::new(rich_text).sense(egui::Sense::click()));

                        if label.clicked() {
                            ui_state.selected_entity = Some(id);
                        }

                        let popup = ui.make_persistent_id(format!("Entity {id} popup"));
                        if label.secondary_clicked() {
                            ui.memory_mut(|mem| mem.toggle_popup(popup));
                        }
                        egui::popup::popup_below_widget(ui, popup, &label, |ui| {
                            ui.set_min_width(80.);
                            if ui.button("delete").clicked() {
                                world.entities.send_cmd(EntityCommand::DeleteEntity(id));
                            }
                        });
                    }

                    ui.add_space(30.);
                });
        });
    }
}
