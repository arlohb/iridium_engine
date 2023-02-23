use egui::Color32;

use crate::ui::{PanelUi, ScreenRect};

fn gizmo(context: &egui::Context, position: egui::Pos2, color: Color32) {
    context.debug_painter().add(egui::epaint::CircleShape {
        center: position,
        radius: 10.,
        fill: color,
        stroke: egui::Stroke::NONE,
    });
}

fn gizmo_rect(context: &egui::Context, rect: egui::Rect, color: Color32) {
    gizmo(context, rect.min, color);
    gizmo(context, egui::pos2(rect.min.x, rect.max.y), color);
    gizmo(context, egui::pos2(rect.max.x, rect.min.y), color);
    gizmo(context, rect.max, color);
}

pub struct ViewportPanel;

impl PanelUi for ViewportPanel {
    fn name(&self) -> &'static str {
        "ViewportPanel"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut crate::ui::UiState,
        world: &mut iridium_ecs::World,
        assets: &iridium_assets::Assets,
    ) {
        // egui::CentralPanel::default().show(context, |ui| {
        //     ui.label("You should never see this");
        // });

        let gizmo = |position: egui::Pos2, color: Color32| {
            gizmo(context, position, color);
            position
        };

        let gizmo_rect = |rect: egui::Rect, color: Color32| {
            gizmo_rect(context, rect, color);
            rect
        };

        gizmo(egui::pos2(790., 590.), Color32::RED);

        gizmo_rect(context.screen_rect(), Color32::WHITE);
        gizmo_rect(context.available_rect(), Color32::LIGHT_BLUE);
        gizmo_rect(
            ui_state
                .viewport_rect
                .egui_logical(ui_state.screen_size.0, ui_state.screen_size.1, 1.),
            Color32::BLUE,
        );
        gizmo_rect(
            ui_state.viewport_rect.egui_logical(
                ui_state.screen_size.0,
                ui_state.screen_size.1,
                1.2,
            ),
            Color32::DARK_BLUE,
        );

        let logical = context.available_rect();
        let physical = egui::Rect {
            min: (logical.min.to_vec2() / ui_state.scale_factor).to_pos2(),
            max: (logical.max.to_vec2() / ui_state.scale_factor).to_pos2(),
        };

        gizmo_rect(dbg!(logical), Color32::LIGHT_GREEN);
        gizmo_rect(dbg!(physical), Color32::GREEN);

        dbg!(context.pixels_per_point());

        let width_physical = dbg!(context.screen_rect().width()) / ui_state.scale_factor;
        let height_physical = context.screen_rect().height() / ui_state.scale_factor;
        gizmo(
            dbg!(egui::pos2(width_physical, height_physical)),
            Color32::LIGHT_YELLOW,
        );
        gizmo(
            dbg!(egui::pos2(
                ui_state.screen_size.0 as f32,
                ui_state.screen_size.1 as f32
            )),
            Color32::YELLOW,
        );

        let screen = egui::Rect {
            min: (physical.min.to_vec2() / width_physical).to_pos2(),
            max: (physical.max.to_vec2() / height_physical).to_pos2(),
        };

        ui_state.viewport_rect = ScreenRect {
            min_x: screen.min.x,
            min_y: screen.min.y,
            max_x: screen.max.x,
            max_y: screen.max.y,
        };

        // let available = dbg!(context.available_rect());

        // ui_state.viewport_rect.min_x =
        //     LOGICAL   /     SCALE_FACTOR       /         WIDTH
        //            PHYSICAL                    /         WIDTH
        //                            SCREEN
        // (available.min.x * ui_state.scale_factor) / ui_state.screen_size.0 as f32;
        // ui_state.viewport_rect.min_y =
        // (available.min.y * ui_state.scale_factor) / ui_state.screen_size.1 as f32;
        // ui_state.viewport_rect.max_x =
        // (available.max.x * ui_state.scale_factor) / ui_state.screen_size.0 as f32;
        // ui_state.viewport_rect.max_y =
        // (available.max.y * ui_state.scale_factor) / ui_state.screen_size.1 as f32;
    }
}
