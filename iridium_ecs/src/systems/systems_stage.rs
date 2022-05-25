use crate::Entities;
use super::*;

pub struct SystemsStage {
    systems: Vec<Box<dyn System>>,
}

impl SystemsStage {
    pub fn new(systems: Vec<Box<dyn System>>) -> SystemsStage {
        SystemsStage {
            systems,
        }
    }

    pub fn run_systems(&mut self, entities: &mut Entities, delta_time: f64) {
        for system in self.systems.iter_mut() {
            if system.get_activated() {
                system.run_system(entities, delta_time);
            }
        }
    }
}
