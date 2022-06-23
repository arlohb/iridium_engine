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
mod ui;
mod project;
use project::Project;
mod play_state;

use iridium_core::*;
use iridium_ecs::*;
use iridium_ecs::systems::*;
use iridium_graphics::*;
use iridium_maths::*;
use iridium_map_utils::*;

use inline_spirv::include_spirv;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Iridium Editor")
        .with_maximized(false)
        .build(&event_loop)
        .unwrap();
    
    let mut app = pollster::block_on(App::new(&window));

    let textures = fast_map_arc! {
        "steak" => Texture::from_image_bytes(
            &app.device,
            &app.queue,
            include_bytes!("../assets/FoodSprites/Food/Steak.png"),
            false,
        ),
    };

    let shaders = fast_map_arc! {
        "sprite_vertex" => Shader::new(
            &app.device,
            ShaderType::Vertex,
            include_spirv!("src/vert.hlsl", vert, hlsl, entry="vs_main"),
            vec![
                wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            ],
        ),
        "sprite_fragment" => Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/sprite.hlsl", frag, hlsl, entry="fs_main"),
            vec![
                textures["steak"].texture_binding_type,
                textures["steak"].sampler_binding_type,
            ],
        ),
        "uv_test_fragment" => Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/uv_test.hlsl", frag, hlsl, entry="fs_main"),
            vec![],
        ),
    };

    let materials = fast_map_arc! {
        "steak" => Material::new(
            &app.device,
            app.surface_config.format,
            shaders["sprite_vertex"].clone(),
            shaders["sprite_fragment"].clone(),
        ),
        "uv_test" => Material::new(
            &app.device,
            app.surface_config.format,
            shaders["sprite_vertex"].clone(),
            shaders["uv_test_fragment"].clone(),
        ),
    };

    let meshes = fast_map_arc! {
        "quad" => Mesh {
            vertices: vec![
                Vertex::new(Vec3::new(-1., -1., 0.), [0., 0.]),
                Vertex::new(Vec3::new(-1.,  1., 0.), [0., 1.]),
                Vertex::new(Vec3::new( 1.,  1., 0.), [1., 1.]),
                Vertex::new(Vec3::new( 1., -1., 0.), [1., 0.]),
            ],
            indices: vec![
                0, 3, 2,
                0, 2, 1,
            ],
        },
    };

    let assets = Assets {
        textures,
        shaders,
        materials,
        meshes,
    };

    let mut world = World::new(
        Entities::default(),
        Systems::new(vec![
            SystemsStage::new(vec![
                Box::new(VelocitySystem),
            ]),
            SystemsStage::new(vec![
                // Box::new(PositionLoggerSystem),
                Box::new(FrameHistorySystem),
            ]),
        ]),
    );

    world.entities.register_component::<Renderable2D>();

    let project = Project::load("target/debug/libiridium_example_project.so");

    project.init_system(&app.device, &mut world, &assets);

    let mut last_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !app.input(event) { match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            WindowEvent::Resized(physical_size) => {
                app.resize((physical_size.width, physical_size.height));
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                app.resize((new_inner_size.width, new_inner_size.height));
            },
            _ => {}
        }},
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let delta_time = last_time.elapsed().as_nanos() as f64 / 1_000_000.;
            last_time = std::time::Instant::now();

            if let PlayState::Play = app.ui_state.play_state() {
                world.run_systems(delta_time);
            }

            app.render(&window, &mut world);
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        },
        _ => {}
    });
}
