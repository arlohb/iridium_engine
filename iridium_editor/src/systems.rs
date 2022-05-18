use iridium_ecs::*;
use iridium_ecs_macros::System;
use crate::components::*;

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&self, entities: &mut Entities) {
        for entity in entities.query_2::<Position, Velocity>().iter() {
            let position = entity.get_component::<Position>().unwrap();
            let velocity = entity.get_component::<Velocity>().unwrap();

            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&self, entities: &mut Entities) {
        for entity in entities.query_1::<Position>().iter() {
            println!("{:?}", entity.get_component::<Position>().unwrap());
        }
    }
}
