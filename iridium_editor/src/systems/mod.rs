mod frame_history;
pub use frame_history::*;

use iridium_ecs::Entities;
use iridium_ecs_macros::System;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &Entities, delta_time: f64) {
        for [mut position, velocity]
        in entities.query(["Position", "Velocity"]) {
            *position.get_mut::<f64>("x") += velocity.get::<f64>("x") * delta_time;
            *position.get_mut::<f64>("y") += velocity.get::<f64>("y") * delta_time;
            *position.get_mut::<f64>("z") += velocity.get::<f64>("z") * delta_time;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &Entities, _delta_time: f64) {
        for [position]
        in entities.query(["Position"]) {
            println!("{}", position.display(&entities.component_types["Position"]));
        }
    }
}
