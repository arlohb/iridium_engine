use std::any::TypeId;

use iridium_assets::Assets;
use iridium_ecs_macros::impl_system_inputs_for_all;

use crate::{Component, Entities};

/// Something that can be used as input for a system.
pub trait SystemInputs<'a>
where
    Self: Sized,
{
    /// Create an iterator over self from entities.
    fn from_entities(entities: &'a Entities) -> std::vec::IntoIter<Self>;
}

impl SystemInputs<'_> for () {
    fn from_entities(_entities: &Entities) -> std::vec::IntoIter<Self> {
        vec![].into_iter()
    }
}

impl_system_inputs_for_all!();

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
