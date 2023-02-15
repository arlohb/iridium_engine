use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use iridium_core::ProjectSettings;
use iridium_ecs::World;

use iridium_assets::Assets;

#[derive(WrapperApi)]
pub struct ProjectApi {
    project_settings: fn() -> ProjectSettings,
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
    pub project_settings: ProjectSettings,
    api: Container<ProjectApi>,
}

impl Project {
    pub fn load(path: &str) -> Self {
        let container: Container<ProjectApi> =
            unsafe { Container::load(path) }.expect("Failed to load project");

        Self {
            project_settings: container.project_settings(),
            api: container,
        }
    }

    pub fn init_system(&self, world: &mut World, assets: &Assets) {
        self.api.init_system(world, assets);
    }

    pub fn load_assets(
        &self,
        camera_gpu_data: &iridium_graphics::CameraGpuData,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
        assets: &mut Assets,
    ) {
        self.api
            .load_assets(camera_gpu_data, device, queue, surface_format, assets);
    }
}
