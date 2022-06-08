use iridium_ecs::World;

use crate::ui::PanelUi;

pub struct EntitiesList;

impl PanelUi for EntitiesList {
    fn render(&mut self, context: &egui::Context, world: &mut World) {
        egui::CentralPanel::default().show(context, |ui| {
            for [name] in world.entities.query(["Name"]) {
                ui.label(name.get::<String>("name"));
            }
        });
    }
}
