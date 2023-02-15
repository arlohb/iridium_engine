//! Runs the game without the engine.
//!
//! For use in a final executable.

mod app;
pub use app::*;
mod project;
pub use project::*;

use iridium_assets::Assets;
use iridium_core::{InputState, LogState};
use iridium_ecs::{systems::Systems, Entities, World};
use iridium_graphics::{Camera, CameraGpuData, Renderable2D, Renderer2DState};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    // Create the event loop.
    let event_loop = EventLoop::new();
    // Create the window.
    let window = WindowBuilder::new()
        .with_title("Iridium Editor")
        .with_maximized(false)
        .build(&event_loop)
        .expect("Failed to create window");

    // Start the app.
    let app = pollster::block_on(App::new(&window));

    // Create the world.
    let mut world = World::new(Entities::default(), Systems::new());

    // Create the camera data.
    let camera_gpu_data = CameraGpuData::new(&app.device);

    // Register the default components.
    world.entities.register_component::<Renderable2D>();
    world.entities.register_component::<Renderer2DState>();
    world.entities.register_component::<InputState>();
    world.entities.register_component::<LogState>();
    world.entities.register_component_with_default::<Camera>();
    world.entities.add_components(
        world
            .entities
            .entity_id_from_name("SystemState")
            .expect("SystemState entity not found"),
        vec![
            Renderer2DState {
                active_camera: String::new(),
                camera_gpu_data: Some(camera_gpu_data),
            }
            .into(),
            InputState::default().into(),
        ],
    );

    // Create the camera.
    world
        .entities
        .new_entity(None, "Camera", vec![Camera::default().into()]);

    // Create the assets.
    let mut assets = Assets::new();

    // Load the project.
    let project = Project::load("target/debug/libiridium_example_project.so");

    // Load the assets.
    project.load_assets(
        world
            .entities
            .get::<Renderer2DState>()
            .camera_gpu_data
            .as_ref()
            .expect("CameraGpuData not found"),
        &app.device,
        &app.queue,
        app.surface_config.format,
        &mut assets,
    );

    // Run the init system.
    project.init_system(&mut world, &assets);
}
