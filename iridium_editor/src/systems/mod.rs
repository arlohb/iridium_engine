mod frame_history;
pub use frame_history::*;

use iridium_ecs::{systems::System, create_component_type};
use iridium_maths::Vec3;

pub fn velocity_system() -> System {
    System {
        name: "VelocitySystem",
        component_type: create_component_type! { struct VelocitySystem {} },
        system: |entities, delta_time| {
            for [mut transform, mut velocity]
            in entities.query(["Transform", "Velocity"]) {
                *transform.get_mut::<f32>("rotation") += 0.002 * delta_time as f32;

                let position = transform.get_mut::<Vec3>("position");
                let velocity = velocity.get_mut::<Vec3>("velocity");
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
        },
    }
}

// This is a system to test other things,
// so is not always used.
#[allow(dead_code)]
pub fn position_logger_system() -> System {
    System {
        name: "PositionLoggerSystem",
        component_type: create_component_type! { struct PositionLoggerSystem {} },
        system: |entities, _delta_time| {
            for [transform]
            in entities.query(["Transform"]) {
                let position = transform.get::<Vec3>("position");
                println!("{} {} {}", position.x, position.y, position.z);
            }
        },
    }
}
