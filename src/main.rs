mod ecs;
use ecs::*;

struct System {
    pub name: &'static str,
    pub activated: bool,
    system: fn(&mut Entities) -> (),
}

impl System {
    pub fn new(name: &'static str, activated: bool, system: fn(&mut Entities) -> ()) -> System {
        System {
            name,
            activated,
            system,
        }
    }

    pub fn run(&self, entities: &mut Entities) {
        (self.system)(entities);
    }
}

struct World {
    entities: Entities,
    systems: Vec<System>,
}

impl World {
    pub fn new(entities: Entities, systems: Vec<System>) -> World {
        World {
            entities,
            systems,
        }
    }

    pub fn run_systems(&mut self) {
        for system in &self.systems {
            if system.activated {
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
            System::new(
                "Move",
                true,
                |entities| {
                    entities.query_2::<Position, Velocity>().iter().for_each(|e| {
                        let pos = e.get_component::<Position>().unwrap();
                        let vel = e.get_component::<Velocity>().unwrap();
                        pos.x += vel.x;
                        pos.y += vel.y;
                    });
                }
            ),
            System::new(
                "Position logger",
                true,
                |entities| {
                    entities.query_1::<Position>().iter().for_each(|e| {
                        println!("{:?}", e.get_component::<Position>().unwrap());
                    });
                }
            ),
        ]
    );

    loop {
        world.run_systems();
    }
}
