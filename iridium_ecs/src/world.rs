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

    pub fn run_systems(&mut self) {
        for system in &self.systems {
            if system.get_activated() {
                system.run_system(&mut self.entities);
            }
        }
    }
}
