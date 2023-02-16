use crate::ProjectSettings;
use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use iridium_ecs::World;

use iridium_assets::Assets;

#[derive(WrapperApi)]
struct ProjectApi {
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

/// Connects to the library file created by the project.
pub struct Project {
    /// The project's settings,
    ///
    /// generated from the `project_settings` function.
    pub project_settings: ProjectSettings,

    api: Container<ProjectApi>,
}

impl Project {
    /// Load the project from the library file.
    ///
    /// Their is some weirdness with this being dropped
    /// before data that it's allocated.
    ///
    /// Make sure to drop this **after** anything that it's created on the heap.
    /// This can be done automatically be defining this first,
    /// as local variables are dropped in the reverse order
    /// they are defined in.
    #[must_use]
    pub fn load(path: &str) -> Self {
        let container: Container<ProjectApi> =
            unsafe { Container::load(path) }.expect("Failed to load project");

        Self {
            project_settings: container.project_settings(),
            api: container,
        }
    }

    /// Runs the init system.
    ///
    /// This is where components and systems are registered,
    /// and also system stages are setup until that's part
    /// of the scene file.
    pub fn init_system(&self, world: &mut World, assets: &Assets) {
        self.api.init_system(world, assets);
    }

    /// This loads the assets required for the project.
    /// This either includes the assets compiled into the
    /// binary file, or opens the file when called.
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
