use iridium_assets::Assets;
use iridium_ecs::Entities;
use iridium_ecs_macros::{system_helper, Component, ComponentStorage, InspectorUi};

use crate::Velocity;

/// The entity is affected by gravity.
#[derive(Component, InspectorUi, ComponentStorage, Default)]
pub struct Weight;

/// The state for the `GravitySystem`.
#[derive(Component, InspectorUi, ComponentStorage)]
pub struct GravityState {
    /// The acceleration down due to gravity.
    #[drag_speed(0.0001)]
    pub acceleration: f32,
}

impl Default for GravityState {
    fn default() -> Self {
        Self { acceleration: 9.81 }
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
