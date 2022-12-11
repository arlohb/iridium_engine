use iridium_assets::Assets;
use iridium_ecs::{Component, Name};

use crate::ui::PanelUi;

/// A widget to edit a component.
pub fn component_widget(ui: &mut egui::Ui, id: impl std::hash::Hash, component: &Component) {
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
    egui::ScrollArea::new([false, true])
        .always_show_scroll(true)
        .auto_shrink([false, false])
        .max_width(f32::INFINITY)
        .show(ui, |ui| {
            for stage in &world.systems.stages {
                for system_name in stage {
                    let system = world
                        .systems
                        .get_system(system_name)
                        .expect("System in stage not found");

                    ui.horizontal(|ui| {
                        ui.label(system_name);
                        let _r = ui.button("/\\");
                        let _r = ui.button("\\/");
                    });

                    let (mut_inputs, immut_inputs) = system.required_components();

                    let type_id_to_name = |type_id: std::any::TypeId| -> &'static str {
                        world
                            .entities
                            .component_info_from_type_id(&type_id)
                            .expect("System input component not found")
                            .type_name
                    };

                    for input in mut_inputs
                        .into_iter()
                        .map(type_id_to_name)
                        .collect::<Vec<_>>()
                    {
                        ui.label(format!("  - &mut {}", input));
                    }

                    for input in immut_inputs
                        .into_iter()
                        .map(type_id_to_name)
                        .collect::<Vec<_>>()
                    {
                        ui.label(format!("  - &{}", input));
                    }

                    ui.separator();
                }
            }
        });
}

/// A widget to edit system states.
pub fn system_states_widget(ui: &mut egui::Ui, entities: &mut iridium_ecs::Entities) {
    egui::ScrollArea::new([false, true])
        .always_show_scroll(true)
        .auto_shrink([false, false])
        .max_width(f32::INFINITY)
        .show(ui, |ui| {
            entities
                // Get the system state.
                .get_entity_components(
                    // Get the id of the system state.
                    entities
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

                    system_states_widget(ui, &mut world.entities);
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
                                    world.entities.add_components(id, [component]);
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
