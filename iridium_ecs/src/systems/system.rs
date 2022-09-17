use std::any::TypeId;

use iridium_assets::Assets;

use crate::{Component, Entities};

/// A system is a function that runs every frame.
pub trait System: 'static + Send + Sync {
    /// The name of the system.
    fn name(&self) -> &'static str;
    /// The type id of the system's state.
    fn state_type_id(&self) -> TypeId;
    /// The default state of the system as a `Component`.
    fn default_state(&self) -> Component;
    /// The function that runs every frame.
    fn system(&self, state: &Component, entities: &Entities, assets: &Assets, delta_time: f64);
}
