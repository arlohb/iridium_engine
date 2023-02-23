//! An example project for the iridium game engine.

#![warn(clippy::expect_used)]

mod velocity;
pub use velocity::*;
mod death;
pub use death::*;
mod collision;
pub use collision::*;
mod movement;
pub use movement::*;

mod assets;
pub use assets::*;

use iridium_assets::Assets;
use iridium_core::ProjectSettings;
use iridium_ecs::World;

/// Returns the project settings.
#[no_mangle]
pub fn project_settings() -> ProjectSettings {
    ProjectSettings {
        default_scene: "iridium_example_project/scenes/scene.json5".to_string(),
    }
}

/// An init system.
#[no_mangle]
pub fn init_system(world: &mut World, assets: &Assets) {
    // To silence warnings.
    let _ = assets;

    world.entities.register_component_with_default::<Velocity>();
    world.entities.register_component_with_default::<Wall>();
    world.entities.register_component_with_default::<Death>();
    world.entities.register_component_with_default::<Movement>();

    world.systems.add_system(VelocitySystem);
    world.systems.add_system(DeathSystem);
    world.systems.add_system(MovementSystem);
    world.systems.add_system(CollisionSystem);

    world.systems.stages = vec![
        vec!["VelocitySystem".to_string()],
        vec!["MovementSystem".to_string()],
        vec!["CollisionSystem".to_string()],
    ];
}
