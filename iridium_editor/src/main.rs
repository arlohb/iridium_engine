mod components;
use components::*;
mod systems;
use systems::*;

use iridium_ecs::*;

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
