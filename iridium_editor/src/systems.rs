use iridium_ecs::*;
use crate::components::*;

pub struct VelocitySystem {
    pub activated: bool,
}

impl System for VelocitySystem {
    fn name(&self) -> &'static str { "VelocitySystem" }

    fn get_activated(&self) -> bool { self.activated }
    fn set_activated(&mut self, activated: bool) { self.activated = activated; }

    fn run(&self, entities: &mut Entities) {
        for entity in entities.query_2::<Position, Velocity>().iter() {
            let position = entity.get_component::<Position>().unwrap();
            let velocity = entity.get_component::<Velocity>().unwrap();

            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

pub struct PositionLoggerSystem {
    pub activated: bool,
}

impl System for PositionLoggerSystem {
    fn name(&self) -> &'static str { "PositionLoggerSystem" }

    fn get_activated(&self) -> bool { self.activated }
    fn set_activated(&mut self, activated: bool) { self.activated = activated; }

    fn run(&self, entities: &mut Entities) {
        for entity in entities.query_1::<Position>().iter() {
            println!("{:?}", entity.get_component::<Position>().unwrap());
        }
    }
}
