use iridium_assets::Assets;
use iridium_ecs::{Entities, Transform};
use iridium_ecs_macros::{
    system_helper, Component, ComponentStorage, HasStableTypeId, InspectorUi,
};

/// The velocity of an entity.
#[derive(Component, InspectorUi, ComponentStorage, HasStableTypeId)]
pub struct Velocity {
    #[drag_speed(0.0001)]
    /// The velocity.
    pub velocity: iridium_maths::VecN<3>,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            velocity: iridium_maths::VecN::new([0.0, 0.0, 0.0]),
        }
    }
}

/// Applies velocity to entities with the `Velocity` component.
pub struct VelocitySystem;

impl VelocitySystem {
    fn system(
        _state: (),
        _entities: &Entities,
        (_, transform, velocity): (u128, &mut Transform, &mut Velocity),
        _assets: &Assets,
        delta_time: f64,
    ) -> Result<(), String> {
        transform.position += velocity.velocity * delta_time as f32;

        Ok(())
    }
}

#[system_helper((), par_iter, &mut Transform, &mut Velocity)]
impl System for VelocitySystem {}
