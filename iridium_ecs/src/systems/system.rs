use crate::*;

pub trait System: Send + Sync {
    fn name(&self) -> &'static str;
    fn get_activated(&self) -> bool;
    fn set_activated(&mut self, activated: bool);
    fn run_system(&mut self, entities: &Entities, delta_time: f64);
}
