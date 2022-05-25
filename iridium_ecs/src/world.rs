use crate::*;
use systems::*;

pub struct World {
    entities: Entities,
    systems: Systems,
}

impl World {
    pub fn new(entities: Entities, systems: Systems) -> World {
        World {
            entities,
            systems,
        }
    }

    pub fn run_systems(&mut self, delta_time: f64) {
        self.systems.run_systems(&mut self.entities, delta_time);
    }
}
