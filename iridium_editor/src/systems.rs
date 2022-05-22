use iridium_ecs::*;
use iridium_ecs_macros::System;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for entity in entities.query(vec!["Position", "Velocity"]).iter_mut() {
            let velocity = entity.get_component("Velocity").unwrap();
            let x = *velocity.get::<f64>("x").unwrap();
            let y = *velocity.get::<f64>("y").unwrap();
            let z = *velocity.get::<f64>("z").unwrap();
            let position = entity.get_component_mut("Position").unwrap();

            *position.get_mut::<f64>("x").unwrap() += x;
            *position.get_mut::<f64>("y").unwrap() += y;
            *position.get_mut::<f64>("z").unwrap() += z;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for entity in entities.query(vec!["Position"]).iter() {
            println!("{:?}", entity.get_component("Position").unwrap());
        }
    }
}

#[derive(System)]
pub struct DeltaTimeLoggerSystem {
    activated: bool,
}

impl DeltaTimeLoggerSystem {
    fn run(&mut self, _entities: &mut Entities, delta_time: f64) {
        println!("Delta Time: {}", delta_time);
    }
}
