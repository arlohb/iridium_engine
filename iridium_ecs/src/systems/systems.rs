use crate::Entities;
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

    pub fn run_systems(&mut self, entities: &mut Entities, delta_time: f64) {
        for systems_stage in self.systems.iter_mut() {
            systems_stage.run_systems(entities, delta_time);
        }
    }
}
