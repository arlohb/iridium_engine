use crate::*;

pub trait System: 'static + Send + Sync {
    fn name(&self) -> &'static str;
    fn default_state(&self) -> Component;
    fn system(&self, entities: &Entities, delta_time: f64);
}
