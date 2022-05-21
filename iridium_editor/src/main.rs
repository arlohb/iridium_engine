mod components;
use components::*;
mod systems;
use systems::*;

use iridium_ecs::*;

fn main() {
    let mut world = World::new(
        Entities::new(vec![
            Entity::new(vec![
                Box::new(Position { x: 0., y: 0., z: 0. }),
                Box::new(Velocity { x: 1., y: 1., z: 1. }),
            ]),
        ]),
        vec![
            Box::new(VelocitySystem::new(true)),
            Box::new(PositionLoggerSystem::new(true)),
        ]
    );

    loop {
        world.run_systems();
    }
}
