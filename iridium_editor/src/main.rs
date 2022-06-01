mod components;
mod systems;
use systems::*;
mod app;
use app::*;
mod assets;
use assets::*;

use iridium_ecs::*;
use iridium_ecs::systems::*;
use iridium_graphics::*;
use iridium_maths::*;

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

    let mut assets = Assets {
        shaders: vec![
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
                vec![
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                ],
            )),
            Arc::new(Shader::new(
                &app.device,
                ShaderType::Fragment,
                include_spirv!("src/frag_2.hlsl", frag, hlsl, entry="fs_main"),
                vec![],
            )),
        ],
        materials: vec![],
        meshes: vec![
            Arc::new(Mesh {
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
            }),
        ],
    };

    assets.materials = vec![
        Arc::new(Material::new(
            &app.device,
            app.surface_config.format,
            assets.shaders[0].clone(),
            assets.shaders[1].clone(),
        )),
        Arc::new(Material::new(
            &app.device,
            app.surface_config.format,
            assets.shaders[0].clone(),
            assets.shaders[2].clone(),
        )),
    ];

    let mut world = World::new(
        {
            let mut entities = Entities::new(components::component_types());

            let steak_bytes = include_bytes!("../assets/FoodSprites/Food/Steak.png");
            let steak_image = image::load_from_memory(steak_bytes).unwrap();
            let steak_rgba = steak_image.to_rgba8();

            let dimensions = steak_rgba.dimensions();

            let texture_size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            let steak_texture = app.device.create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
                label: None,
            });

            app.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &steak_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &steak_rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                    rows_per_image: std::num::NonZeroU32::new(dimensions.1),
                },
                texture_size,
            );

            let steak_texture_view = steak_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let steak_sampler = app.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            entities.new_entity("Entity 0", create_components! {
                "Transform" => fast_map_any! {
                    "position" => Vec3::new(-1., -1., 0.),
                    "scale" => Vec3::new(0.2, 0.2, 0.2),
                    "rotation" => 0.3f32
                },
                "Velocity" => fast_map_any! {
                    "velocity" => Vec3::new(0.0001, 0.0001, 0.)
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,
                    MaterialInstance::new(
                        &app.device,
                        assets.materials[0].clone(),
                        vec![],
                        vec![],
                        vec![],
                        vec![
                            wgpu::BindingResource::TextureView(&steak_texture_view),
                            wgpu::BindingResource::Sampler(&steak_sampler),
                        ],
                    ),
                    &assets.meshes[0],
                )
            });

            entities.new_entity("Entity 1", create_components! {
                "Transform" => fast_map_any! {
                    "position" => Vec3::new(-1., -1., 0.),
                    "scale" => Vec3::new(0.2, 0.2, 0.2),
                    "rotation" => 0.6f32
                },
                "Velocity" => fast_map_any! {
                    "velocity" => Vec3::new(0.0002, 0.0002, 0.)
                },
                "Renderable2D" => create_renderable_2d(
                    &app.device,MaterialInstance::new(
                        &app.device,
                        assets.materials[1].clone(),
                        vec![],
                        vec![],
                        vec![],
                        vec![],
                    ),
                    &assets.meshes[0],
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
