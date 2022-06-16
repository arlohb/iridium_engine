use crate::{Entities, Component};
use super::*;

pub struct Systems {
    systems: Vec<SystemsStage>,
}

impl Systems {
    pub fn new(systems_stages: Vec<SystemsStage>) -> Systems {
        Systems {
            systems: systems_stages,
        }
    }

    pub fn default_component_states(&self) -> Vec<Component> {
        self.systems.iter()
            .flat_map(|systems_stage| systems_stage.systems
                .iter()
                .map(|system| system.default_state())
            )
            .collect()
    }

    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        for systems_stage in self.systems.iter_mut() {
            systems_stage.run_systems(entities, delta_time);
        }
    }
}
