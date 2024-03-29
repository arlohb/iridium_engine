use iridium_assets::Assets;

use crate::{ComponentBox, Entities};
use iridium_reflect::StableTypeId;

/// A system is a function that runs every frame.
pub trait System: 'static + Send + Sync {
    /// The name of the system.
    fn name(&self) -> &'static str;
    /// The type id of the system's state.
    fn state_type_id(&self) -> StableTypeId;
    /// The default state of the system as a `Component`.
    /// This is `None` if the system does not have state.
    fn default_state(&self) -> Option<ComponentBox>;
    /// The components that the system requires.
    ///
    /// This is used to determine which components to query for.
    ///
    /// The first is mutable, the second is immutable.
    fn required_components(&self) -> [Vec<StableTypeId>; 2];
    /// The function that runs every frame.
    fn system(
        &self,
        state: Option<&ComponentBox>,
        entities: &Entities,
        assets: &Assets,
        delta_time: f64,
    );
}
