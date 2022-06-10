use iridium_ecs::World;

use super::UiState;

pub trait PanelUi {
    fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World);
}

impl PanelUi for egui_demo_lib::DemoWindows {
    fn render(&mut self, context: &egui::Context, _: &mut UiState, _: &mut World) {
        self.ui(context);
    }
}
