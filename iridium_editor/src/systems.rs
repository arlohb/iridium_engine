use iridium_ecs::*;
use iridium_ecs_macros::System;
use crate::components::*;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for entity in entities.query_2::<Position, Velocity>().iter() {
            let position = entity.get_component::<Position>().unwrap();
            let velocity = entity.get_component::<Velocity>().unwrap();

            position.x += velocity.x;
            position.y += velocity.y;
            position.z += velocity.z;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &mut Entities, _delta_time: f64) {
        for entity in entities.query_1::<Position>().iter() {
            println!("{:?}", entity.get_component::<Position>().unwrap());
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
