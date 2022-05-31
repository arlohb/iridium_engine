mod components;
mod systems;
use systems::*;
mod app;
use app::*;


use iridium_ecs::*;
use iridium_ecs::systems::*;
use iridium_graphics::*;
use wgpu::util::DeviceExt;

use std::sync::Arc;
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

    let shaders: Vec<Arc<Shader>> = vec![
        Arc::new(Shader::new(
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
        )),
        Arc::new(Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/frag_1.hlsl", frag, hlsl, entry="fs_main"),
            vec![],
        )),
        Arc::new(Shader::new(
            &app.device,
            ShaderType::Fragment,
            include_spirv!("src/frag_2.hlsl", frag, hlsl, entry="fs_main"),
            vec![],
        )),
    ];

    let mut world = World::new(
        {
            let mut entities = Entities::new(components::component_types());

            entities.new_entity("Entity 0", create_components! {
                "Position" => fast_map_any! {
                    "x" => 0.0,
                    "y" => 0.0,
                    "z" => 0.0
                },
                "Velocity" => fast_map_any! {
                    "x" => 0.0001,
                    "y" => 0.0001,
                    "z" => 0.0001
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,
                    {
                        let buffer = Arc::new(app.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: &[0.4f32.to_le_bytes(), 0.4f32.to_le_bytes()].into_iter().flatten().collect::<Vec<u8>>(),
                            usage: wgpu::BufferUsages::UNIFORM
                                | wgpu::BufferUsages::COPY_DST,
                        }));

                        MaterialInstance::new(
                            &app.device,
                            Arc::new(Material::new(
                                &app.device,
                                app.surface_config.format,
                                shaders[0].clone(),
                                shaders[1].clone(),
                            )),
                            vec![buffer.clone()],
                            vec![buffer.as_entire_binding()],
                            vec![],
                            vec![],
                        )
                    },
                    &[
                        [-1.0, -1.0, 0.0],
                        [-1.0,  0.0, 0.0],
                        [ 0.0,  0.0, 0.0],
                        [ 0.0, -1.0, 0.0],
                    ],
                    &[
                        0, 3, 2,
                        0, 2, 1,
                    ],
                )
            });

            entities.new_entity("Entity 1", create_components! {
                "Position" => fast_map_any! {
                    "x" => 0.0,
                    "y" => 0.0,
                    "z" => 0.0
                },
                "Velocity" => fast_map_any! {
                    "x" => 0.0002,
                    "y" => 0.0002,
                    "z" => 0.0002
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,
                    {
                        let buffer = Arc::new(app.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: &[0u8; 8],
                            usage: wgpu::BufferUsages::UNIFORM
                                | wgpu::BufferUsages::COPY_DST,
                        }));

                        MaterialInstance::new(
                            &app.device,
                            Arc::new(Material::new(
                                &app.device,
                                app.surface_config.format,
                                shaders[0].clone(),
                                shaders[2].clone(),
                            )),
                            vec![buffer.clone()],
                            vec![buffer.as_entire_binding()],
                            vec![],
                            vec![],
                        )
                    },
                    &[
                        [-0.5, -0.5, 0.0],
                        [-0.5,  0.5, 0.0],
                        [ 0.5,  0.5, 0.0],
                        [ 0.5, -0.5, 0.0],
                    ],
                    &[
                        0, 3, 2,
                        0, 2, 1,
                    ],
                )
            });

            entities
        },
        Systems::new(vec![
            SystemsStage::new(vec![
                Box::new(VelocitySystem::new(true)),
            ]),
            SystemsStage::new(vec![
                Box::new(PositionLoggerSystem::new(false)),
                Box::new(FrameHistorySystem::new(true, 500_000, 5000.)),
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

            match app.render(&world.entities) {
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
