mod components;
mod systems;
use systems::*;
mod app;
use app::*;
mod assets;
use assets::*;
mod ui;

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

#[tokio::main]
async fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    
    let mut app = App::new(&window).await;

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
            include_spirv!("src/frag_1.hlsl", frag, hlsl, entry="fs_main"),
            vec![
                textures["steak"].texture_binding_type,
                textures["steak"].sampler_binding_type,
            ],
        ),
        "uv_test_fragment" => Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/frag_2.hlsl", frag, hlsl, entry="fs_main"),
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
        {
            let mut entities = Entities::new(components::component_types());

            entities.new_entity("Entity 0", create_components! {
                "Transform" => fast_map_any! {
                    "position" => Vec3::new(-1., -1., 0.),
                    "scale" => Vec3::new(0.2, 0.2, 0.2),
                    "rotation" => 0.3f32,
                },
                "Velocity" => fast_map_any! {
                    "velocity" => Vec3::new(0.0001, 0.0001, 0.),
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,
                    MaterialInstance::new(
                        &app.device,
                        assets.materials["steak"].clone(),
                        vec![],
                        vec![],
                        vec![],
                        vec![
                            wgpu::BindingResource::TextureView(&assets.textures["steak"].view),
                            wgpu::BindingResource::Sampler(&assets.textures["steak"].sampler),
                        ],
                    ),
                    &assets.meshes["quad"],
                ),
            });

            entities.new_entity("Entity 1", create_components! {
                "Transform" => fast_map_any! {
                    "position" => Vec3::new(-1., -1., 0.),
                    "scale" => Vec3::new(0.2, 0.2, 0.2),
                    "rotation" => 0.6f32,
                },
                "Velocity" => fast_map_any! {
                    "velocity" => Vec3::new(0.0002, 0.0002, 0.),
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,MaterialInstance::new(
                        &app.device,
                        assets.materials["uv_test"].clone(),
                        vec![],
                        vec![],
                        vec![],
                        vec![],
                    ),
                    &assets.meshes["quad"],
                ),
            });

            entities
        },
        Systems::new(vec![
            SystemsStage::new(vec![
                Box::new(VelocitySystem::new(true)),
            ]),
            SystemsStage::new(vec![
                Box::new(PositionLoggerSystem::new(false)),
                Box::new(FrameHistorySystem::new(false, 500_000, 5000.)),
            ]),
        ]),
    );

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
                app.resize(*physical_size);
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                app.resize(**new_inner_size);
            },
            _ => {}
        }},
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let delta_time = last_time.elapsed().as_nanos() as f64 / 1_000_000.;
            last_time = std::time::Instant::now();

            world.run_systems(delta_time);
            app.update();

            match app.render(&window, &mut world) {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => app.resize(app.surface_size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(err) => panic!("{:?}", err),
            }
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        },
        _ => {}
    });
}
