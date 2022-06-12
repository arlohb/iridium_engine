use crate::{ui::PanelUi, systems::frame_history_average_delta_time};

pub struct ComponentsList;

impl PanelUi for ComponentsList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut crate::ui::UiState, world: &mut iridium_ecs::World) {
        egui::SidePanel::right("components_list").show(context, |ui| {
            ui_state.viewport_rect.max_x = ui.min_rect().min.x / (ui_state.screen_size.0 as f32 / ui_state.scale_factor);

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
