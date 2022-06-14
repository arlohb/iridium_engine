use hashbrown::HashMap;

use crate::{Entities, ComponentType};
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

    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        for systems_stage in self.systems.iter_mut() {
            systems_stage.run_systems(entities, delta_time);
        }
    }

    pub fn component_types(&self) -> HashMap<String, ComponentType> {
        let mut component_types = HashMap::new();
        for systems_stage in &self.systems {
            for system in &systems_stage.systems {
                component_types.insert(system.component_type().name, system.component_type());
            }
        }
        component_types
    }
}
