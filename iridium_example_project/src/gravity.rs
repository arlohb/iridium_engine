use iridium_assets::Assets;
use iridium_ecs::{
    storage::{ComponentStorage, StoredComponent, StoredComponentField},
    Entities,
};
use iridium_ecs_macros::{system_helper, ComponentStorage, ComponentTrait, InspectorUi};
use iridium_map_utils::fast_map;

use crate::Velocity;

/// The entity is affected by gravity.
#[derive(ComponentTrait, InspectorUi, ComponentStorage, Default)]
pub struct Weight;

/// The state for the `GravitySystem`.
#[derive(ComponentTrait, InspectorUi)]
pub struct GravityState {
    /// The acceleration down due to gravity.
    pub acceleration: f32,
}

impl Default for GravityState {
    fn default() -> Self {
        Self { acceleration: 9.81 }
    }
}

impl ComponentStorage for GravityState {
    fn from_stored(mut stored: StoredComponent, _assets: &Assets) -> Option<Self> {
        Some(Self {
            acceleration: stored.get("acceleration")?.parse().ok()?,
        })
    }

    fn to_stored(&self) -> StoredComponent {
        StoredComponent {
            type_name: "GravityState".to_string(),
            fields: fast_map! {
                "acceleration" => StoredComponentField::new(self.acceleration.to_string(), false),
            },
        }
    }
}

/// Applies gravity to entities with the `Weight` component.
pub struct GravitySystem;

impl GravitySystem {
    fn system(
        state: &GravityState,
        _entities: &Entities,
        (_, velocity, _): (u128, &mut Velocity, &Weight),
        _assets: &Assets,
        delta_time: f64,
    ) {
        *velocity.velocity.y_mut() -= state.acceleration * delta_time as f32;
    }
}

#[system_helper(GravityState, par_iter, &mut Velocity, &Weight)]
impl System for GravitySystem {}
