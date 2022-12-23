use std::any::TypeId;

use hashbrown::{HashMap, HashSet};
use iridium_assets::Assets;
use rayon::prelude::*;

use super::System;
use crate::{Component, Entities};

/// Stores the systems in the world.
#[derive(Default)]
pub struct Systems {
    /// The systems in the world.
    ///
    /// The key is the system name.
    systems: HashMap<String, Box<dyn System>>,
    /// The stages they should run in,
    /// identified by their name.
    pub stages: Vec<Vec<String>>,
}

impl Systems {
    /// Creates a new systems.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all the default component states for each of the systems.
    #[must_use]
    pub fn default_component_states(&self) -> Vec<Component> {
        self.systems
            .iter()
            .map(|(_, system)| system.default_state())
            .collect()
    }

    /// Add a system, this doesn't place it in a stage.
    pub fn add_system<S: System>(&mut self, system: S) {
        self.systems
            .insert(system.name().to_string(), Box::new(system));
    }

    /// Gets a system by name.
    #[must_use]
    pub fn get_system(&self, name: &str) -> Option<&dyn System> {
        self.systems.get(name).map(|system| &**system)
    }

    /// Moves a system up a stage.
    ///
    /// # Errors
    ///
    /// Returns `None` if the system is already at the top,
    /// or if the system doesn't exist.
    pub fn move_system_up(&mut self, name: &str) -> Option<()> {
        // Find the system in the stages.
        let stage_index = self
            .stages
            .iter()
            .position(|stage| stage.contains(&name.to_string()))?;

        // Add the system to the stage above.
        self.stages
            .get_mut(stage_index.checked_sub(1)?)?
            .push(name.to_string());

        // Remove the system from the stage.
        let system_in_stage_index = self.stages[stage_index]
            .iter()
            .position(|system_name| system_name == name)?;
        self.stages[stage_index].remove(system_in_stage_index);

        Some(())
    }

    /// Moves a system down a stage.
    ///
    /// # Errors
    ///
    /// Returns `None` if the system is already at the bottom,
    /// or if the system doesn't exist.
    pub fn move_system_down(&mut self, name: &str) -> Option<()> {
        // Find the system in the stages.
        let stage_index = self
            .stages
            .iter()
            .position(|stage| stage.contains(&name.to_string()))?;

        // Add the system to the stage above.
        self.stages
            .get_mut(stage_index.checked_add(1)?)?
            .push(name.to_string());

        // Remove the system from the stage.
        let system_in_stage_index = self.stages[stage_index]
            .iter()
            .position(|system_name| system_name == name)?;
        self.stages[stage_index].remove(system_in_stage_index);

        Some(())
    }

    /// Executes the systems.
    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64, assets: &Assets) {
        // Run each stage, not in parallel.
        self.stages.iter().for_each(|stage| {
            // Run each system in the stage in parallel.
            stage.par_iter().for_each(|name| {
                // Get the system.
                let system = self.systems.get(name).expect("System in stage not found");

                // Get the type id of the system state.
                let state_type_id = system.state_type_id();
                // Get the system state component.
                let state = entities.get_by_type_id(&state_type_id);

                // Run the system.
                system.system(state, entities, assets, delta_time);
            });
        });
    }

    /// Find errors in a stage.
    /// This will check mutability rules are followed.
    ///
    /// This takes in the inputs of the systems,
    /// so it can be tested separately.
    /// Use `find_errors` instead.
    #[must_use]
    pub fn find_errors_in_stage(inputs: Vec<[Vec<TypeId>; 2]>) -> HashSet<TypeId> {
        puffin::profile_function!();

        // Split the inputs into mutable and immutable.
        let (mut_inputs, immut_inputs): (Vec<_>, Vec<_>) =
            inputs.into_iter().map(|[a, b]| (a, b)).unzip();

        // Flatten the mutable inputs into a Vec.
        let mut_inputs = mut_inputs.into_iter().flatten().collect::<Vec<_>>();
        // Flatten the immutable inputs into a HashSet.
        // This can be a HashSet as duplicates don't matter.
        let immut_inputs = immut_inputs.into_iter().flatten().collect::<HashSet<_>>();

        // Check for duplicate mutable inputs.
        let duplicate_mut_inputs = {
            let mut inputs = HashSet::new();
            let mut duplicates = HashSet::new();

            for input in &mut_inputs {
                if !inputs.insert(input) {
                    duplicates.insert(*input);
                }
            }

            duplicates
        };

        // Check for mutable inputs that are also immutable.
        let mut_inputs_in_immut_inputs = mut_inputs
            .iter()
            .filter(|input| immut_inputs.contains(input))
            .copied()
            .collect::<HashSet<_>>();

        // Merge these two sets.
        let all_errors = duplicate_mut_inputs
            .union(&mut_inputs_in_immut_inputs)
            .copied()
            .collect::<HashSet<_>>();

        all_errors
    }

    /// Find errors in all the stages.
    /// This will check mutability rules are followed.
    #[must_use]
    pub fn find_errors(&self) -> Vec<HashSet<TypeId>> {
        puffin::profile_function!();

        // Get the inputs of each system.
        self.stages
            .iter()
            .map(|stage| {
                stage
                    .iter()
                    .map(|name| {
                        let system = self.systems.get(name).expect("System in stage not found");
                        system.required_components()
                    })
                    .collect::<Vec<_>>()
            })
            // Find the errors in each stage.
            .map(Self::find_errors_in_stage)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn empty() {
        assert_eq!(Systems::find_errors_in_stage(vec![]).len(), 0);
    }

    #[test]
    fn no_errors() {
        assert_eq!(
            Systems::find_errors_in_stage(vec![
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u64>()]],
                [vec![TypeId::of::<u16>()], vec![TypeId::of::<u8>()]],
            ])
            .len(),
            0
        );
    }

    #[test]
    fn multiple_immut() {
        assert_eq!(
            Systems::find_errors_in_stage(vec![
                [vec![TypeId::of::<u64>()], vec![TypeId::of::<u128>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
                [vec![TypeId::of::<u16>()], vec![TypeId::of::<u8>()]],
            ])
            .len(),
            0
        );
    }

    #[test]
    fn multiple_mut() {
        assert_eq!(
            Systems::find_errors_in_stage(vec![
                [vec![TypeId::of::<u64>()], vec![TypeId::of::<u128>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
            ])
            .len(),
            1
        );
    }

    #[test]
    fn mut_and_immut() {
        assert_eq!(
            Systems::find_errors_in_stage(vec![
                [vec![TypeId::of::<u64>()], vec![TypeId::of::<u32>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
                [vec![TypeId::of::<u16>()], vec![TypeId::of::<u8>()]],
            ])
            .len(),
            1
        );
    }

    #[test]
    fn multiple_errosr() {
        assert_eq!(
            Systems::find_errors_in_stage(vec![
                [vec![TypeId::of::<u64>()], vec![TypeId::of::<u16>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
                [vec![TypeId::of::<u32>()], vec![TypeId::of::<u8>()]],
                [vec![TypeId::of::<u16>()], vec![TypeId::of::<u8>()]],
            ])
            .len(),
            2
        );
    }
}
