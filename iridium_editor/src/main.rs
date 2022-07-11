//! # Iridium Editor
//!
//! A game engine for Rust.

#[macro_use]
extern crate dlopen_derive;

mod systems;
use play_state::PlayState;
use systems::*;
mod app;
pub use app::*;
mod project;
mod ui;
use project::Project;
mod play_state;

use iridium_assets::*;
use iridium_ecs::systems::*;
use iridium_ecs::*;
use iridium_graphics::*;
use iridium_maths::*;

use inline_spirv::include_spirv;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    puffin::set_scopes_on(true);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Iridium Editor")
        .with_maximized(false)
        .build(&event_loop)
        .unwrap();

    let mut app = pollster::block_on(App::new(&window));

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
        world.entities.entity_id_from_name("SystemState").unwrap(),
        vec![Component::new(Renderer2DState {
            active_camera: "".to_string(),
            camera_gpu_data: Some(camera_gpu_data),
        })],
    );

    world.entities.new_entity("Camera", vec![Camera::create()]);

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
            vec![wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            }],
        ),
    );
    assets.add(
        "sprite_fragment",
        Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/sprite.hlsl", frag, hlsl, entry = "fs_main"),
            vec![
                assets
                    .get::<Texture>("steak_tex")
                    .unwrap()
                    .texture_binding_type,
                assets
                    .get::<Texture>("steak_tex")
                    .unwrap()
                    .sampler_binding_type,
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
            assets.get::<Shader>("sprite_vertex").unwrap(),
            world
                .entities
                .get::<Renderer2DState>()
                .camera_gpu_data
                .as_ref()
                .unwrap(),
            assets.get::<Shader>("sprite_fragment").unwrap(),
        ),
    );

    assets.add(
        "uv_test",
        Material::new(
            &app.device,
            app.surface_config.format,
            assets.get::<Shader>("sprite_vertex").unwrap(),
            world
                .entities
                .get::<Renderer2DState>()
                .camera_gpu_data
                .as_ref()
                .unwrap(),
            assets.get::<Shader>("uv_test_fragment").unwrap(),
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

    project.init_system(&app.device, &mut world, &assets);

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
            let delta_time = last_time.elapsed().as_nanos() as f64 / 1_000_000.;
            last_time = std::time::Instant::now();

            if let PlayState::Play = app.ui_state.play_state() {
                puffin::profile_scope!("Systems");
                world.run_systems(delta_time);
            }

            app.render(&window, &mut world, &assets);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
