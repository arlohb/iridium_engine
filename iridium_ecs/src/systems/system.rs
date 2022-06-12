use crate::*;

pub type SystemFn = fn(entities: &Entities, delta_time: f64);

pub struct System {
    pub name: &'static str,
    pub component_type: ComponentType,
    pub system: SystemFn,
}
