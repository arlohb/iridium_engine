use rayon::prelude::*;

use crate::Entities;
use super::*;

/// A system stage.
/// 
/// The systems in a stage are executed in parallel.
pub struct SystemsStage {
    /// The systems in this stage.
    pub systems: Vec<Box<dyn System>>,
}

impl SystemsStage {
    /// Creates a new stage with the given systems.
    pub fn new(systems: Vec<Box<dyn System>>) -> SystemsStage {
        SystemsStage {
            systems,
        }
    }

    /// Executes the systems in this stage.
    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        self.systems.par_iter_mut().for_each(|system| {
            system.system(entities, delta_time);
        });
    }
}
