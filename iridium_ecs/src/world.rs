use super::*;

pub struct World {
    entities: Entities,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new(entities: Entities, systems: Vec<Box<dyn System>>) -> World {
        World {
            entities,
            systems,
        }
    }

    pub fn run_systems(&mut self, delta_time: f64) {
        for system in self.systems.iter_mut() {
            if system.get_activated() {
                system.run_system(&mut self.entities, delta_time);
            }
        }
    }
}