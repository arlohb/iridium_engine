use iridium_ecs::World;

use crate::{ui::{UiState, PanelUi}, systems::frame_history_average_delta_time};

pub struct EntitiesList;

impl PanelUi for EntitiesList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World) {
        egui::CentralPanel::default().show(context, |ui| {
            let fps = 1000. / frame_history_average_delta_time(&world.entities.get("FrameHistoryState"));
            ui.label(format!("Fps average: {:.1}", fps));
            ui.separator();

            for (id, [name])
            in world.entities.query_with_id(["Name"]) {
                let name = name.get::<String>("name");

                let label = ui.label(name);
                let label = label.interact(egui::Sense::click());
                if label.clicked() {
                    println!("Clicked id: {id}")
                }
                if ui.button(name).clicked() {
                    ui_state.selected_entity = Some(id);
                    println!("Clicked id: {id}");
                };
            }
        });
    }
}
