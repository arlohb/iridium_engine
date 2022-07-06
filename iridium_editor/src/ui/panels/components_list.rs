use iridium_ecs::Name;

use crate::ui::PanelUi;

pub struct ComponentsList;

impl PanelUi for ComponentsList {
    fn name(&self) -> &'static str { "ComponentsList" }

    fn render(&mut self, context: &egui::Context, ui_state: &mut crate::ui::UiState, world: &mut iridium_ecs::World) {
        egui::SidePanel::right("components_list").show(context, |ui| {
            let min_x_logical = ui.max_rect().min.x - ui.spacing().item_spacing.x;
            let min_x_physical = min_x_logical * ui_state.scale_factor;
            let min_x_screen = min_x_physical / ui_state.screen_size.0 as f32;
            ui_state.viewport_rect.max_x = min_x_screen;

            egui::ScrollArea::new([false, true])
                .always_show_scroll(true)
                .auto_shrink([false, false])
                .max_width(f32::INFINITY)
                .show(ui, |ui| {
                // If something is actually selected.
                if let Some(id) = ui_state.selected_entity {
                    ui.menu_button("Add Component", |ui| {
                        for (type_name, default) in world.entities.component_defaults() {
                            if ui.button(type_name).clicked() {
                                ui.close_menu();
                                let component = default();
                                world.entities.add_components(id, vec![component]);
                            }
                        }
                    });

                    // Get the components of the entity.
                    let mut components = world.entities.get_entity_components(id);

                    // Sort the components by type, putting name first.
                    components.sort_by(|a, b| {
                        if a.is_type::<Name>() {
                            return std::cmp::Ordering::Less;
                        }
                        if b.is_type::<Name>() {
                            return std::cmp::Ordering::Greater;
                        }
                
                        a.type_id().cmp(&b.type_id())
                    });

                    // Get the name of the entity.
                    let name = &mut components[0].get_mut::<Name>().name;

                    // Add some top spacing.
                    ui.add_space(10.);

                    // Text edit for name
                    ui.text_edit_singleline(name);

                    // Separator.
                    ui.separator();
                    ui.add_space(10.);

                    // For each component,
                    for (index, component) in components.into_iter().enumerate() {
                        // Except Name.
                        if component.is_type::<Name>() {
                            continue;
                        }

                        egui::CollapsingHeader::new(component.type_name())
                            .default_open(true)
                            .show(ui, |ui| {
                                egui::Grid::new(index)
                                    .show(ui, |ui| {
                                        component.get_trait_mut().ui(ui);
                                    });
                            });


                        // Separator.
                        ui.separator();
                    }
                }
            });
        });
    }
}
