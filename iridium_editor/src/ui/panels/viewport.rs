use crate::ui::{PanelUi, ScreenRect};

pub struct ViewportPanel;

impl PanelUi for ViewportPanel {
    fn name(&self) -> &'static str {
        "ViewportPanel"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut crate::ui::UiState,
        _world: &mut iridium_ecs::World,
        _assets: &iridium_assets::Assets,
    ) {
        let physical_rect = context.available_rect();

        let screen_space_rect = egui::Rect {
            min: egui::pos2(
                physical_rect.min.x / context.screen_rect().width(),
                physical_rect.min.y / context.screen_rect().height(),
            ),
            max: egui::pos2(
                physical_rect.max.x / context.screen_rect().width(),
                physical_rect.max.y / context.screen_rect().height(),
            ),
        };

        ui_state.viewport_rect = ScreenRect {
            min_x: screen_space_rect.min.x,
            min_y: screen_space_rect.min.y,
            max_x: screen_space_rect.max.x,
            max_y: screen_space_rect.max.y,
        };
    }
}
