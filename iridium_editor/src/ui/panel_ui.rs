use iridium_assets::Assets;
use iridium_ecs::World;

use super::UiState;

/// Defines a panel that can be rendered to the screen.
pub trait PanelUi {
    /// The name of the panel.
    ///
    /// Just for profiling purposes.
    fn name(&self) -> &'static str;
    /// Renders the panel.
    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut UiState,
        world: &mut World,
        assets: &Assets,
    );
}

impl PanelUi for egui_demo_lib::DemoWindows {
    fn name(&self) -> &'static str {
        "DemoWindow"
    }

    fn render(&mut self, context: &egui::Context, _: &mut UiState, _: &mut World, _: &Assets) {
        self.ui(context);
    }
}
