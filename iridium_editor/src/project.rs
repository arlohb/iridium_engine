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
    pub fn load(path: &str) -> Project {
        let container: Container<ProjectApi> = unsafe { Container::load(path) }.unwrap();

        Project { api: container }
    }

    pub fn init_system(&self, device: &wgpu::Device, world: &mut World, assets: &Assets) {
        (self.api.init_system)(device, world, assets);
    }
}
