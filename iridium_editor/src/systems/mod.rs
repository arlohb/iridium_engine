mod frame_history;
pub use frame_history::*;

use iridium_ecs::Entities;
use iridium_ecs_macros::System;
use iridium_maths::Vec3;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &Entities, delta_time: f64) {
        for [mut transform, velocity]
        in entities.query(["Transform", "Velocity"]) {
            *transform.get_mut::<Vec3>("position")  += *velocity.get::<Vec3>("velocity") * delta_time as f32;
            *transform.get_mut::<f32>("rotation") += 0.002 * delta_time as f32;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &Entities, _delta_time: f64) {
        for [transform]
        in entities.query(["Transform"]) {
            let position = transform.get::<Vec3>("position");
            println!("{} {} {}", position.x, position.y, position.z);
        }
    }
}
