use crate::*;
use systems::*;

pub struct World {
    pub entities: Entities,
    systems: Systems,
}

impl World {
    pub fn new(mut entities: Entities, systems: Systems) -> World {
        entities.new_entity("SystemState", systems.default_component_states());

        World {
            entities,
            systems,
        }
    }

    pub fn run_systems(&mut self, delta_time: f64) {
        self.systems.run_systems(&self.entities, delta_time);
    }
}
