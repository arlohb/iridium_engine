use iridium_assets::Assets;

use super::SystemsStage;
use crate::{Component, Entities};

/// Stores the systems in the world.
pub struct Systems {
    /// The systems in the world.
    systems: Vec<SystemsStage>,
}

impl Systems {
    /// Creates a new systems.
    #[must_use]
    pub fn new(systems_stages: Vec<SystemsStage>) -> Self {
        Self {
            systems: systems_stages,
        }
    }

    /// Return all the default component states for each of the systems.
    #[must_use]
    pub fn default_component_states(&self) -> Vec<Component> {
        self.systems
            .iter()
            .flat_map(|systems_stage| {
                systems_stage
                    .systems
                    .iter()
                    .map(|system| system.default_state())
            })
            .collect()
    }

    /// Executes the systems.
    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64, assets: &Assets) {
        for systems_stage in &mut self.systems {
            systems_stage.run_systems(entities, delta_time, assets);
        }
    }
}
