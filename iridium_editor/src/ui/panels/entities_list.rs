use iridium_assets::Assets;
use iridium_ecs::{Name, World};

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
                    world.entities.new_entity("New Entity", []);
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

                    for (id, [name]) in {
                        let mut entities = world
                            .entities
                            .query_by_type_id_with_id([&std::any::TypeId::of::<Name>()])
                            .collect::<Vec<_>>();
                        entities.sort_by_key(|(_, [name])| &name.get::<Name>().name);
                        entities
                    } {
                        let name = name.get::<Name>();
                        let name = &name.name;

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

                        if ui
                            .add(egui::Label::new(rich_text).sense(egui::Sense::click()))
                            .clicked()
                        {
                            ui_state.selected_entity = Some(id);
                        }
                    }

                    ui.add_space(30.);
                });
        });
    }
}
