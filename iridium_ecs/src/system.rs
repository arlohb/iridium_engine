use super::*;

pub trait System {
    fn name(&self) -> &'static str;
    fn get_activated(&self) -> bool;
    fn set_activated(&mut self, activated: bool);
    fn run_system(&self, entities: &mut Entities);
}
