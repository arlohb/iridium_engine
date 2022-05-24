use iridium_ecs::*;
use iridium_ecs_macros::System;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for [position, velocity]
        in entities.query(["Position", "Velocity"]) {
            let x = *velocity.lock().unwrap().get::<f64>("x").unwrap();
            let y = *velocity.lock().unwrap().get::<f64>("y").unwrap();
            let z = *velocity.lock().unwrap().get::<f64>("z").unwrap();

            *position.lock().unwrap().get_mut::<f64>("x").unwrap() += x;
            *position.lock().unwrap().get_mut::<f64>("y").unwrap() += y;
            *position.lock().unwrap().get_mut::<f64>("z").unwrap() += z;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for [position]
        in entities.query(["Position"]) {
            println!("{}", position.lock().unwrap().display(&entities.component_types["Position"]));
        }
    }
}

#[derive(System)]
pub struct DeltaTimeLoggerSystem {
    activated: bool,
}

impl DeltaTimeLoggerSystem {
    fn run(&mut self, _entities: &mut Entities, delta_time: f64) {
        println!("Delta Time: {:<10} Fps: {:<8.1}", delta_time, 1000. / delta_time);
    }
}
