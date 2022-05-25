use crate::*;

pub trait System {
    fn name(&self) -> &'static str;
    fn get_activated(&self) -> bool;
    fn set_activated(&mut self, activated: bool);
    fn run_system(&mut self, entities: &mut Entities, delta_time: f64);
}
