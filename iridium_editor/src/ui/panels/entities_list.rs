use iridium_ecs::World;

use crate::ui::{UiState, PanelUi};

pub struct EntitiesList;

impl PanelUi for EntitiesList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World) {
        egui::CentralPanel::default().show(context, |ui| {
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
