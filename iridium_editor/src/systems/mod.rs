mod frame_history;
pub use frame_history::*;

use iridium_ecs::{*, systems::System};
use iridium_ecs_macros::ComponentTrait;

#[derive(ComponentTrait)]
pub struct VelocityState {
    pub rotation_speed: f32,
}

pub struct VelocitySystem;

impl System for VelocitySystem {
    fn name(&self) -> &'static str { "VelocitySystem" }

    fn component_type(&self) -> &'static str { "VelocityState" }

    fn system(&self, entities: &Entities, delta_time: f64) {
        for (transform, velocity)
        in query!(entities, [mut Transform, mut Velocity;]) {
            let state = entities.get::<VelocityState>();
            transform.rotation += state.rotation_speed * delta_time as f32;

            let position = &mut transform.position;
            let velocity = &mut velocity.velocity;
            *position += *velocity * delta_time as f32;

            if position.x < -1. {
                position.x = -1.;
                velocity.x = -velocity.x;
            }
            if position.x > 1. {
                position.x = 1.;
                velocity.x = -velocity.x;
            }
            if position.y < -1. {
                position.y = -1.;
                velocity.y = -velocity.y;
            }
            if position.y > 1. {
                position.y = 1.;
                velocity.y = -velocity.y;
            }
        }
    }
}

#[allow(dead_code)]
#[derive(ComponentTrait)]
pub struct PositionLoggerState {}

// This is a system to test other things,
// so is not always used.
#[allow(dead_code)]
pub struct PositionLoggerSystem;

impl System for PositionLoggerSystem {
    fn name(&self) -> &'static str { "PositionLoggerSystem" }

    fn component_type(&self) -> &'static str {
        "PositionLoggerState"
    }

    fn system(&self, entities: &Entities, _delta_time: f64) {
        for (transform, )
        in query!(entities, [; Transform]) {
            let position = transform.position;
            println!("{} {} {}", position.x, position.y, position.z);
        }
    }
}
