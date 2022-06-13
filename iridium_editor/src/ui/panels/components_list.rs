use crate::{ui::PanelUi, systems::frame_history_average_delta_time};

pub struct ComponentsList;

impl PanelUi for ComponentsList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut crate::ui::UiState, world: &mut iridium_ecs::World) {
        egui::SidePanel::right("components_list").show(context, |ui| {
            let min_x_logical = ui.max_rect().min.x - ui.spacing().item_spacing.x;
            let min_x_physical = min_x_logical * ui_state.scale_factor;
            let min_x_screen = min_x_physical / ui_state.screen_size.0 as f32;
            ui_state.viewport_rect.max_x = min_x_screen;

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

            if let Some(id) = ui_state.selected_entity {
                let components = world.entities.get_entity_components(id);

                for component in components {
                    ui.label(component.name.to_string());
                }
            }
        });
    }
}
