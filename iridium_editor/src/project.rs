use dlopen::wrapper::{Container, WrapperApi};
use iridium_ecs::World;

use iridium_assets::Assets;

#[derive(WrapperApi)]
pub struct ProjectApi {
    default_scene: fn() -> String,
    load_assets: fn(
        camera_gpu_data: &iridium_graphics::CameraGpuData,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
        assets: &mut Assets,
    ) -> (),
    init_system: fn(world: &mut World, assets: &Assets) -> (),
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

    pub fn init_system(&self, world: &mut World, assets: &Assets) {
        (self.api.init_system)(world, assets);
    }

    pub fn load_assets(
        &self,
        camera_gpu_data: &iridium_graphics::CameraGpuData,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
        assets: &mut Assets,
    ) {
        (self.api.load_assets)(camera_gpu_data, device, queue, surface_format, assets);
    }

    pub fn default_scene(&self) -> String {
        (self.api.default_scene)()
    }
}
