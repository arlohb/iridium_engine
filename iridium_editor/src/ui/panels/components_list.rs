use std::any::TypeId;

use iridium_assets::Assets;
use iridium_ecs::{ComponentBox, Name};
use std::collections::HashSet;

use crate::ui::PanelUi;

/// A widget to edit a component.
pub fn component_widget(ui: &mut egui::Ui, id: impl std::hash::Hash, component: &ComponentBox) {
    egui::CollapsingHeader::new(component.type_name())
        .default_open(true)
        .show(ui, |ui| {
            egui::Grid::new(id).show(ui, |ui| {
                component.get_trait_mut().ui(ui);
            });
        });
}

/// A widget to view / reorder systems in stages.
pub fn system_stages_widget(ui: &mut egui::Ui, world: &mut iridium_ecs::World) {
    puffin::profile_function!();

    egui::ScrollArea::new([false, true])
        .always_show_scroll(true)
        .auto_shrink([false, false])
        .max_width(f32::INFINITY)
        .show(ui, |ui| {
            world
                .systems
                .stages
                .clone()
                .into_iter()
                .enumerate()
                .zip(world.systems.find_errors().into_iter())
                .for_each(|((index, mut stage), errors)| {
                    egui::CollapsingHeader::new(format!("Stage {index}"))
                        .default_open(true)
                        .show(ui, |ui| {
                            // Sort systems by name.
                            // Just for consistency,
                            stage.sort();

                            let (mut_inputs, immut_inputs): (Vec<_>, Vec<_>) = stage
                                .clone()
                                .into_iter()
                                // Get the inputs of each system.
                                .map(|system_name| {
                                    world
                                        .systems
                                        .get_system(&system_name)
                                        .expect("System in stage not found")
                                        .required_components()
                                })
                                .map(|[a, b]| (a, b))
                                .unzip();

                            stage
                                .into_iter()
                                .zip(mut_inputs)
                                .zip(immut_inputs)
                                .for_each(|((system_name, mut_inputs), immut_inputs)| {
                                    ui.horizontal(|ui| {
                                        // The system name.
                                        ui.label(&system_name);

                                        // Place the buttons on the right.
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Min),
                                            |ui| {
                                                if ui.button("/\\").clicked() {
                                                    world.systems.move_system_up(&system_name);
                                                }
                                                if ui.button("\\/").clicked() {
                                                    world.systems.move_system_down(&system_name);
                                                }
                                            },
                                        );
                                    });

                                    let zip_with_names =
                                        |type_id: TypeId| -> (TypeId, &'static str) {
                                            (
                                                type_id,
                                                world
                                                    .entities
                                                    .component_info_from_type_id(&type_id)
                                                    .expect("System input component not found")
                                                    .type_name,
                                            )
                                        };

                                    for (input_id, input_name) in
                                        mut_inputs.into_iter().map(zip_with_names)
                                    {
                                        ui.colored_label(
                                            if errors.contains(&input_id) {
                                                ui.visuals().warn_fg_color
                                            } else {
                                                ui.visuals().text_color()
                                            },
                                            format!("  - &mut {input_name}"),
                                        );
                                    }

                                    for (input_id, input_name) in
                                        immut_inputs.into_iter().map(zip_with_names)
                                    {
                                        ui.colored_label(
                                            if errors.contains(&input_id) {
                                                ui.visuals().warn_fg_color
                                            } else {
                                                ui.visuals().text_color()
                                            },
                                            format!("  - &{input_name}"),
                                        );
                                    }

                                    ui.separator();
                                });
                        });
                });

            if ui.button("Add stage").clicked() {
                world.systems.stages.push(vec![]);
            }
        });
}

/// A widget to edit system states.
pub fn system_states_widget(ui: &mut egui::Ui, world: &mut iridium_ecs::World) {
    egui::ScrollArea::new([false, true])
        .always_show_scroll(true)
        .auto_shrink([false, false])
        .max_width(f32::INFINITY)
        .show(ui, |ui| {
            world
                .entities
                // Get the system state.
                .get_entity_components(
                    // Get the id of the system state.
                    world
                        .entities
                        .entity_id_from_name("SystemState")
                        .expect("SystemState not found"),
                )
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .for_each(|(index, component)| {
                    // Don't show Name.
                    if component.type_name() == "Name" {
                        return;
                    }

                    // Show the component.
                    component_widget(ui, index, component);
                    ui.separator();
                });

            if ui.button("Add system state").clicked() {
                // Get the id of the system state.
                let system_state_id = world
                    .entities
                    .entity_id_from_name("SystemState")
                    .expect("SystemState not found");

                // Get the default states of every system.
                let states = world.systems.default_component_states();

                // Get the type ids of the states already in the world.
                let already_added: HashSet<TypeId> = world
                    .entities
                    .get_entity_component_types(system_state_id)
                    .expect("SystemState not found")
                    .into_iter()
                    .collect();

                // Add the states that aren't already in the world.
                world.entities.add_components(
                    system_state_id,
                    states
                        .into_iter()
                        .filter(|state| !already_added.contains(&state.type_id()))
                        .collect(),
                );
            }
        });
}

pub struct ComponentsList;

impl PanelUi for ComponentsList {
    fn name(&self) -> &'static str {
        "ComponentsList"
    }

    fn render(
        &mut self,
        context: &egui::Context,
        ui_state: &mut crate::ui::UiState,
        world: &mut iridium_ecs::World,
        _assets: &Assets,
    ) {
        egui::SidePanel::right("components_list").show(context, |ui| {
            egui::TopBottomPanel::bottom("system_stages")
                .resizable(true)
                .frame(egui::Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(egui::Separator::default().spacing(0.));

                    system_stages_widget(ui, world);
                });

            egui::TopBottomPanel::bottom("system_state")
                .resizable(true)
                .frame(egui::Frame::none())
                .show_inside(ui, |ui| {
                    ui.add(egui::Separator::default().spacing(0.));

                    system_states_widget(ui, world);
                });

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
                        let mut components =
                            if let Some(components) = world.entities.get_entity_components(id) {
                                components
                            } else {
                                return;
                            };

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

                            // Render the component.
                            component_widget(ui, index, component);

                            // Separator.
                            ui.separator();
                        }
                    }
                });
        });
    }
}
