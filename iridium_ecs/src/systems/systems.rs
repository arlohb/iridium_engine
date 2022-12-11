use hashbrown::HashMap;
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
}
