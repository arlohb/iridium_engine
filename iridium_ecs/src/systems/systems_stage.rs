use rayon::prelude::*;

use super::System;
use crate::Entities;

/// A system stage.
///
/// The systems in a stage are executed in parallel.
pub struct SystemsStage {
    /// The systems in this stage.
    pub systems: Vec<Box<dyn System>>,
}

impl SystemsStage {
    /// Creates a new stage with the given systems.
    #[must_use]
    pub fn new(systems: Vec<Box<dyn System>>) -> Self {
        Self { systems }
    }

    /// Executes the systems in this stage.
    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        self.systems.par_iter_mut().for_each(|system| {
            let state_type_id = system.state_type_id();
            let state = entities.get_by_type_id(&state_type_id);

            system.system(state, entities, delta_time);
        });
    }
}
