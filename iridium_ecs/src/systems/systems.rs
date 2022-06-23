use crate::{Entities, Component};
use super::*;

/// Stores the systems in the world.
pub struct Systems {
    /// The systems in the world.
    systems: Vec<SystemsStage>,
}

impl Systems {
    /// Creates a new systems.
    pub fn new(systems_stages: Vec<SystemsStage>) -> Systems {
        Systems {
            systems: systems_stages,
        }
    }

    /// Return all the default component states for each of the systems.
    pub fn default_component_states(&self) -> Vec<Component> {
        self.systems.iter()
            .flat_map(|systems_stage| systems_stage.systems
                .iter()
                .map(|system| system.default_state())
            )
            .collect()
    }

    /// Executes the systems.
    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        for systems_stage in self.systems.iter_mut() {
            systems_stage.run_systems(entities, delta_time);
        }
    }
}
