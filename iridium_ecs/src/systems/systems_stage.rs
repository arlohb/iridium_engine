use rayon::prelude::*;

use crate::Entities;
use super::*;

pub struct SystemsStage {
    pub systems: Vec<Box<dyn System>>,
}

impl SystemsStage {
    pub fn new(systems: Vec<Box<dyn System>>) -> SystemsStage {
        SystemsStage {
            systems,
        }
    }

    pub fn run_systems(&mut self, entities: &Entities, delta_time: f64) {
        self.systems.par_iter_mut().for_each(|system| {
            system.system(entities, delta_time);
        });
    }
}
