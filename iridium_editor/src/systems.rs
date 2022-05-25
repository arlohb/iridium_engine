use iridium_ecs::*;
use iridium_ecs_macros::System;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &Entities, _delta_time: f64) {
        for [mut position, velocity]
        in entities.query(["Position", "Velocity"]) {
            *position.get_mut::<f64>("x") += velocity.get::<f64>("x");
            *position.get_mut::<f64>("y") += velocity.get::<f64>("y");
            *position.get_mut::<f64>("z") += velocity.get::<f64>("z");
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

#[derive(System)]
pub struct DeltaTimeLoggerSystem {
    activated: bool,
}

impl DeltaTimeLoggerSystem {
    fn run(&mut self, _entities: &Entities, delta_time: f64) {
        println!("Delta Time: {:<10} Fps: {:>8.1}", delta_time, 1000. / delta_time);
    }
}
