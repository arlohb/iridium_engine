use crate::*;

pub trait System: 'static + Send + Sync {
    fn name(&self) -> &'static str;
    fn component_type(&self) -> ComponentType;
    fn system(&self, entities: &Entities, delta_time: f64);
}
