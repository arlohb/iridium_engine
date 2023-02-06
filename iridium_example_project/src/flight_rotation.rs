use iridium_assets::Assets;
use iridium_ecs::Transform;
use iridium_ecs_macros::system_helper;

use crate::{Flight, Velocity};

/// The system that applies flight rotation.
pub struct FlightRotationSystem;

impl FlightRotationSystem {
    fn system(
        _state: (),
        _entities: &iridium_ecs::Entities,
        (_, transform, velocity, _): (u128, &mut Transform, &Velocity, &Flight),
        _assets: &Assets,
        delta_time: f64,
    ) -> Result<(), String> {
        let half_pi = std::f32::consts::PI / 2.;

        transform.rotation -= velocity.velocity.y() * delta_time as f32 * 4.;

        let default = 2.52;
        transform.rotation = transform
            .rotation
            .clamp(default - half_pi, default + half_pi);

        Ok(())
    }
}

#[system_helper((), par_iter, &mut Transform, &Velocity, &Flight)]
impl System for FlightRotationSystem {}
