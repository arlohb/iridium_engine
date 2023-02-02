//! # Iridium Editor
//!
//! A game engine for Rust.

#[macro_use]
extern crate dlopen_derive;

mod frame_history;
pub use frame_history::*;
use iridium_core::{InputState, LogState};
use play_state::PlayState;
mod app;
pub use app::*;
mod project;
mod ui;
use project::Project;
mod play_state;

use iridium_assets::Assets;
use iridium_ecs::systems::Systems;
use iridium_ecs::{Entities, World};
use iridium_graphics::{Camera, CameraGpuData, Renderable2D, Renderer2DState};

use egui_winit::winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// This will change in the future.
#[allow(clippy::too_many_lines)]
fn main() {
    // Wgpu uses this logger.
    env_logger::init();

    // Enable the puffin profiler.
    puffin::set_scopes_on(true);

    // Create the event loop.
    let event_loop = EventLoop::new();
    // Create the window.
    let window = WindowBuilder::new()
        .with_title("Iridium Editor")
        .with_maximized(false)
        .build(&event_loop)
        .expect("Failed to create window");

    // Start the app.
    let mut app = pollster::block_on(App::new(&window, &event_loop));

    // Create the world.
    let mut world = World::new(Entities::default(), Systems::new());

    // Create the camera data.
    let camera_gpu_data = CameraGpuData::new(&app.device);

    // Register the default components.
    world.entities.register_component::<Renderable2D>();
    world.entities.register_component::<Renderer2DState>();
    world.entities.register_component::<InputState>();
    world.entities.register_component::<LogState>();
    world.entities.register_component::<FrameHistoryState>();
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

    world.systems.add_system(FrameHistorySystem);

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

    // Open the default scene.
    let default_scene = project.project_settings.default_scene;
    match world.load(&default_scene, &assets) {
        Ok(_) => app.ui_state.open_scene = Some(default_scene),
        Err(e) => println!("Failed to load default scene with error: {e:?}"),
    }

    // The start time of the last frame.
    let mut last_time = std::time::Instant::now();

    // Just while profiling.
    // app.ui_state.play();

    event_loop.run(move |event, _, control_flow| match event {
        // Handle window events.
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            // If the app didn't handle the event itself.
            if !app.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    // Make sure to remove this at some point,
                    // but during dev it's really useful
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        // Exit the app.
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        app.resize((physical_size.width, physical_size.height));
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        app.resize((new_inner_size.width, new_inner_size.height));
                    }
                    _ => {}
                }
            }
        }
        // Redraw the window.
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            // Start a new frame.
            puffin::GlobalProfiler::lock().new_frame();
            puffin::profile_scope!("Frame");

            // Get the time in ms since the last frame.
            let delta_time: f64 = f64::from(
                u32::try_from(last_time.elapsed().as_nanos())
                    .expect("Delta time nanos too big for u32"),
            ) / 1_000_000f64;
            // Reset the last time.
            last_time = std::time::Instant::now();

            // If the game is playing.
            if let PlayState::Play = app.ui_state.play_state() {
                puffin::profile_scope!("Systems");
                // Run the systems.
                world
                    .systems
                    .run_systems(&mut world.entities, delta_time, &assets);
            }

            // Render the app and game.
            app.render(&window, &mut world, &assets);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
