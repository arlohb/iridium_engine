use crate::ui::PanelUi;

pub struct ComponentsList;

impl PanelUi for ComponentsList {
    fn render(&mut self, context: &egui::Context, ui_state: &mut crate::ui::UiState, world: &mut iridium_ecs::World) {
        egui::SidePanel::right("components_list").show(context, |ui| {
            let min_x_logical = ui.max_rect().min.x - ui.spacing().item_spacing.x;
            let min_x_physical = min_x_logical * ui_state.scale_factor;
            let min_x_screen = min_x_physical / ui_state.screen_size.0 as f32;
            ui_state.viewport_rect.max_x = min_x_screen;

            // If something is actually selected.
            if let Some(id) = ui_state.selected_entity {
                // Get the components of the entity.
                let mut components = world.entities.get_entity_components(id);

                // Sort the components by type, putting name first.
                components.sort_by(|a, b| {
                    if a.name == "Name" {
                        return std::cmp::Ordering::Less;
                    }
                    if b.name == "Name" {
                        return std::cmp::Ordering::Greater;
                    }
            
                    a.name.cmp(&b.name)
                });

                // Get the name of the entity.
                let name = components[0].get_mut::<String>("name");

                // Add some top spacing.
                ui.add_space(10.);

                // Text edit for name
                ui.text_edit_singleline(name);

                // Separator.
                ui.separator();
                ui.add_space(10.);

                // For each component,
                for component in components {
                    // Except Name.
                    if component.name == "Name" {
                        continue;
                    }

                    egui::CollapsingHeader::new(&component.name)
                        .default_open(true)
                        .show(ui, |ui| {
                            println!("{}", component.name);
                            let component_type = &world.entities.component_types[&component.name];

                            component_type.values.iter().for_each(|(key, value_type)| {
                                ui.horizontal(|ui| {
                                    ui.label(key);
                                    ui.label(value_type);
                                });
                            })
                        });


                    // Separator.
                    ui.separator();
                }
            }
        });
    }
}
