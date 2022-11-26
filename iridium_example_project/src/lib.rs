use iridium_assets::Assets;

mod gravity;
pub use gravity::*;
mod velocity;
pub use velocity::*;

mod assets;
pub use assets::*;
use iridium_ecs::World;

#[no_mangle]
pub fn default_scene() -> String {
    "iridium_example_project/scenes/scene.json5".to_string()
}

#[no_mangle]
pub fn init_system(world: &mut World, assets: &Assets) {
    // To silence warnings.
    let _ = assets;

    world.entities.register_component_with_default::<Velocity>();
    world
        .entities
        .register_component_with_default::<VelocityState>();
    world.entities.register_component_with_default::<Weight>();
    world
        .entities
        .register_component_with_default::<GravityState>();
}
