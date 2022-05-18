mod ecs;
use ecs::*;

struct World {
    entities: Entities,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new(entities: Entities, systems: Vec<Box<dyn System>>) -> World {
        World {
            entities,
            systems,
        }
    }

    pub fn run_systems(&mut self) {
        for system in &self.systems {
            if system.get_activated() {
                system.run(&mut self.entities);
            }
        }
    }
}

impl std::ops::Index<u128> for World {
    type Output = Entity;

    fn index(&self, id: u128) -> &Self::Output {
        self.entities.get(id).unwrap()
    }
}

fn main() {
    let mut world = World::new(
        Entities::new(vec![
            Entity::new(vec![
                Box::new(Position { x: 0.0, y: 0.0 }),
                Box::new(Velocity { x: 1.0, y: 1.0 }),
            ]),
        ]),
        vec![
            Box::new(VelocitySystem { activated: true }),
            Box::new(PositionLoggerSystem { activated: true }),
        ]
    );

    loop {
        world.run_systems();
    }
}
