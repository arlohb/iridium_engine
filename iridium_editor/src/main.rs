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
use iridium_graphics::{
    Camera, CameraGpuData, Material, Mesh, Renderable2D, Renderer2DState, Shader, ShaderInput,
    ShaderType, Texture, Vertex,
};
use iridium_maths::VecN;

use egui_winit::winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use inline_spirv::include_spirv;

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

    assets.add(
        "steak_tex",
        Texture::from_image_bytes(
            &app.device,
            &app.queue,
            include_bytes!("../assets/FoodSprites/Food/Steak.png"),
            false,
        ),
    );

    assets.add(
        "sprite_vertex",
        Shader::new(
            &app.device,
            ShaderType::Vertex,
            include_spirv!("src/vert.hlsl", vert, hlsl, entry = "vs_main"),
            vec![ShaderInput::Transform],
        ),
    );
    assets.add(
        "sprite_fragment",
        Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/sprite.hlsl", frag, hlsl, entry = "fs_main"),
            vec![
                ShaderInput::Texture(
                    assets
                        .get::<Texture>("steak_tex")
                        .expect("asset 'steak_tex' not found"),
                ),
                ShaderInput::Sampler(
                    assets
                        .get::<Texture>("steak_tex")
                        .expect("asset 'steak_tex' not found"),
                ),
            ],
        ),
    );
    assets.add(
        "uv_test_fragment",
        Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/uv_test.hlsl", frag, hlsl, entry = "fs_main"),
            vec![],
        ),
    );

    assets.add(
        "steak_mat",
        Material::new(
            &app.device,
            app.surface_config.format,
            assets
                .get::<Shader>("sprite_vertex")
                .expect("asset 'sprite_vertex' not found"),
            world
                .entities
                .get::<Renderer2DState>()
                .camera_gpu_data
                .as_ref()
                .expect("Camera GPU data not created yet"),
            assets
                .get::<Shader>("sprite_fragment")
                .expect("asset 'sprite_fragment' not found"),
        ),
    );

    assets.add(
        "uv_test",
        Material::new(
            &app.device,
            app.surface_config.format,
            assets
                .get::<Shader>("sprite_vertex")
                .expect("asset 'sprite_vertex' not found"),
            world
                .entities
                .get::<Renderer2DState>()
                .camera_gpu_data
                .as_ref()
                .expect("Camera GPU data not created yet"),
            assets
                .get::<Shader>("uv_test_fragment")
                .expect("asset 'uv_test_fragment' not found"),
        ),
    );

    assets.add(
        "quad",
        Mesh {
            vertices: vec![
                Vertex::new(VecN::new([-1., -1., 0.]), VecN::new([0., 0.])),
                Vertex::new(VecN::new([-1., 1., 0.]), VecN::new([0., 1.])),
                Vertex::new(VecN::new([1., 1., 0.]), VecN::new([1., 1.])),
                Vertex::new(VecN::new([1., -1., 0.]), VecN::new([1., 0.])),
            ],
            indices: vec![0, 3, 2, 0, 2, 1],
        },
    );

    let project = Project::load("target/debug/libiridium_example_project.so");

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
