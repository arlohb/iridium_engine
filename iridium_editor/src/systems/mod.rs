mod frame_history;
pub use frame_history::*;

use iridium_ecs::{systems::System, create_component_type};
use iridium_maths::Vec3;

pub fn velocity_system() -> System {
    System {
        name: "VelocitySystem",
        component_type: create_component_type! { struct VelocitySystem {} },
        system: |entities, delta_time| {
            for [mut transform, velocity]
            in entities.query(["Transform", "Velocity"]) {
                *transform.get_mut::<Vec3>("position")  += *velocity.get::<Vec3>("velocity") * delta_time as f32;
                *transform.get_mut::<f32>("rotation") += 0.002 * delta_time as f32;
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
