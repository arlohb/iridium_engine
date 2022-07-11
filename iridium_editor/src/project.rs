use dlopen::wrapper::{Container, WrapperApi};
use iridium_ecs::World;

use iridium_assets::Assets;

#[derive(WrapperApi)]
pub struct ProjectApi {
    init_system: fn(device: &wgpu::Device, world: &mut World, assets: &Assets) -> (),
}

pub struct Project {
    api: Container<ProjectApi>,
}

impl Project {
    pub fn load(path: &str) -> Self {
        let container: Container<ProjectApi> =
            unsafe { Container::load(path) }.expect("Failed to load project");

        Self { api: container }
    }

    pub fn init_system(&self, device: &wgpu::Device, world: &mut World, assets: &Assets) {
        (self.api.init_system)(device, world, assets);
    }
}
