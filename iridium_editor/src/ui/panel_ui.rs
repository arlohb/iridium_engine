use iridium_ecs::World;

use super::UiState;

/// Defines a panel that can be rendered to the screen.
pub trait PanelUi {
    /// Renders the panel.
    fn render(&mut self, context: &egui::Context, ui_state: &mut UiState, world: &mut World);
}

impl PanelUi for egui_demo_lib::DemoWindows {
    fn render(&mut self, context: &egui::Context, _: &mut UiState, _: &mut World) {
        self.ui(context);
    }
}
