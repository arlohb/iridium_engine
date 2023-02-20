//! Runs the game without the engine.
//!
//! For use in a final executable.

mod app;

pub use app::*;

use iridium_assets::Assets;
use iridium_core::{InputState, LogState, Project};
use iridium_ecs::{systems::Systems, Entities, World};
use iridium_graphics::{Camera, CameraGpuData, Renderable2D, Renderer2DState};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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
    let mut app = pollster::block_on(App::new(&window));

    // Load the project.
    // This needs to be done before `world` and `assets`,
    // for reasons explained in `Project::load`
    let project = Project::load("target/debug/libiridium_example_project.so");

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

    // Load the assets.
    if let Err(e) = project.load_assets(
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
    ) {
        println!("Failed to load assets with error: {e}");
    };

    // Run the init system.
    project.init_system(&mut world, &assets);

    // Open the default scene.
    let default_scene = project.project_settings.default_scene;
    if let Err(e) = world.load(&default_scene, &assets) {
        println!("Failed to load default scene with error: {e:?}");
    }

    // The start time of the last frame.
    let mut last_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        // Handle window events.
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            // If the app didn't handle the event itself.
            if matches!(event, WindowEvent::CloseRequested) {
                // Exit the app.
                *control_flow = ControlFlow::Exit;
            } else {
                app.input(&mut world, event);
            }
        }
        // Redraw the window.
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // Get the time in ms since the last frame.
            let delta_time: f64 = {
                let micros: u128 = last_time.elapsed().as_micros();
                let micros: u32 = u32::try_from(micros).expect(
                    "Delta time micros too big for u32\n\
                    This would mean 71 mins have passed since the last frame :(",
                );
                let micros: f64 = f64::from(micros);
                micros / 1_000.
            };
            // Reset the last time.
            last_time = std::time::Instant::now();

            // Run the systems.
            world
                .systems
                .run_systems(&mut world.entities, delta_time, &assets);

            // Process the input from last frame.
            let input_state = world.entities.get::<InputState>();
            input_state.process_old_inputs();

            // Render the app and game.
            app.render(&mut world, &assets);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
