use crate::*;

/// A system is a function that runs every frame.
pub trait System: 'static + Send + Sync {
    /// The name of the system.
    fn name(&self) -> &'static str;
    /// The default state of the system as a `Component`.
    fn default_state(&self) -> Component;
    /// The function that runs every frame.
    fn system(&self, entities: &Entities, delta_time: f64);
}
