use iridium_assets::Assets;

pub mod components;
pub mod systems;

mod assets;
pub use assets::*;
use iridium_ecs::World;

#[no_mangle]
pub fn init_system(world: &mut World, assets: &Assets) {
    world
        .load("iridium_example_project/scenes/scene.json5", assets)
        .unwrap();
}
