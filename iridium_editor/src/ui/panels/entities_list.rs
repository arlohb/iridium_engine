use iridium_ecs::World;

use crate::{ui::{UiState, PanelUi}, systems::frame_history_average_delta_time};

pub struct EntitiesList;

impl PanelUi for EntitiesList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World) {
        egui::SidePanel::left("entities_list").show(context, |ui| {
            let max_x_logical = ui.max_rect().max.x + ui.spacing().item_spacing.x;
            let max_x_physical = max_x_logical * ui_state.scale_factor;
            let max_x_screen = max_x_physical / ui_state.screen_size.0 as f32;
            ui_state.viewport_rect.min_x = max_x_screen;

            let fps = 1000. / frame_history_average_delta_time(&world.entities.get("FrameHistoryState"));
            ui.label(format!("Fps average: {:.1}", fps));
            ui.separator();

            ui.menu_button("Add Entity", |ui| {
                if ui.button("Empty").clicked() {
                    world.entities.new_entity("New Entity", vec![]);
                    ui.close_menu()
                }
            });

            ui.separator();
            ui.add_space(10.);

            for (id, [name])
            in world.entities.query_with_id(["Name"]) {
                let name = name.get::<String>("name");

                let mut rich_text = egui::RichText::new(name);

                if let Some(selected_id) = ui_state.selected_entity {
                    if selected_id == id {
                        rich_text = rich_text.strong();
                    }
                }

                if ui.add(egui::Label::new(rich_text).sense(egui::Sense::click())).clicked() {
                    println!("Clicked id: {id}");
                    ui_state.selected_entity = Some(id);
                }
            }
        });
    }
}
