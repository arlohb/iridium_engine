//! # Iridium Editor
//!
//! A game engine for Rust.

#![warn(
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

#[macro_use]
extern crate dlopen_derive;

mod systems;
use play_state::PlayState;
use systems::{FrameHistoryState, FrameHistorySystem, VelocityState, VelocitySystem};
mod app;
pub use app::*;
mod project;
mod ui;
use project::Project;
mod play_state;

use iridium_assets::Assets;
use iridium_ecs::systems::{Systems, SystemsStage};
use iridium_ecs::{Component, ComponentDefault, Entities, World};
use iridium_graphics::{Camera, CameraGpuData, Renderable2D, Renderer2DState};

use egui_winit::winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// This will change in the future.
#[allow(clippy::too_many_lines)]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    puffin::set_scopes_on(true);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Iridium Editor")
        .with_maximized(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let mut app = pollster::block_on(App::new(&window, &event_loop));

    let mut world = World::new(
        Entities::default(),
        Systems::new(vec![
            SystemsStage::new(vec![Box::new(VelocitySystem)]),
            SystemsStage::new(vec![
                // Box::new(PositionLoggerSystem),
                Box::new(FrameHistorySystem),
            ]),
        ]),
    );

    let camera_gpu_data = CameraGpuData::new(&app.device);

    world.entities.register_component::<Renderable2D>();
    world.entities.register_component::<Renderer2DState>();
    world.entities.register_component::<FrameHistoryState>();
    world.entities.register_component::<VelocityState>();
    world.entities.register_component_with_default::<Camera>();
    world.entities.add_components(
        world
            .entities
            .entity_id_from_name("SystemState")
            .expect("SystemState entity not found"),
        [Component::new(Renderer2DState {
            active_camera: String::new(),
            camera_gpu_data: Some(camera_gpu_data),
        })],
    );

    world.entities.new_entity("Camera", [Camera::create()]);

    let mut assets = Assets::new();

    let project = Project::load("target/debug/libiridium_example_project.so");

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
    project.init_system(&mut world, &assets);

    let mut last_time = std::time::Instant::now();

    // Just while profiling.
    // app.ui_state.play();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !app.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
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
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            puffin::GlobalProfiler::lock().new_frame();
            puffin::profile_scope!("Frame");

            let delta_time: f64 = f64::from(
                u32::try_from(last_time.elapsed().as_nanos())
                    .expect("Delta time nanos too big for u32"),
            ) / 1_000_000f64;
            last_time = std::time::Instant::now();

            if let PlayState::Play = app.ui_state.play_state() {
                puffin::profile_scope!("Systems");
                world.run_systems(delta_time, &assets);
            }

            app.render(&window, &mut world, &assets);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
