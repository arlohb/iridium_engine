//! An example project for the iridium game engine.

mod gravity;
pub use gravity::*;
mod velocity;
pub use velocity::*;
mod death;
pub use death::*;
mod flight;
pub use flight::*;
mod flight_rotation;
pub use flight_rotation::*;
mod pipes;
pub use pipes::*;

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
    world.entities.register_component_with_default::<Weight>();
    world
        .entities
        .register_component_with_default::<GravityState>();
    world.entities.register_component_with_default::<Death>();
    world.entities.register_component_with_default::<Flight>();
    world
        .entities
        .register_component_with_default::<PipeState>();
    world.entities.register_component_with_default::<Pipe>();

    world.systems.add_system(VelocitySystem);
    world.systems.add_system(GravitySystem);
    world.systems.add_system(DeathSystem);
    world.systems.add_system(FlightSystem);
    world.systems.add_system(FlightRotationSystem);
    world.systems.add_system(PipeSystem);
    world.systems.add_system(PipeRemovalSystem);
    world.systems.add_system(PipeCollisionSystem);

    world.systems.stages = vec![
        vec![
            "FrameHistorySystem".to_string(),
            "GravitySystem".to_string(),
            "PipeSystem".to_string(),
            "PipeRemovalSystem".to_string(),
            "PipeCollisionSystem".to_string(),
        ],
        vec!["VelocitySystem".to_string()],
        vec!["DeathSystem".to_string()],
        vec!["FlightSystem".to_string()],
        vec!["FlightRotationSystem".to_string()],
    ];
}
