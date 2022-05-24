mod components;
mod systems;
use systems::*;
mod app;
use app::*;

use iridium_ecs::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    
    let mut app = App::new(&window).await;

    let mut world = World::new(
        {
            let mut entities = Entities::new(components::component_types());

            let new_entity = entities.new_entity("Entity 0");

            entities.add_components(
                new_entity,
                create_components! {
                    "Position" => fast_map_any! {
                        "x" => 0.0,
                        "y" => 0.0,
                        "z" => 0.0
                    },
                    "Velocity" => fast_map_any! {
                        "x" => 1.0,
                        "y" => 1.0,
                        "z" => 1.0
                    }
                },
            );

            entities
        },
        vec![
            Box::new(VelocitySystem::new(true)),
            Box::new(PositionLoggerSystem::new(true)),
            Box::new(DeltaTimeLoggerSystem::new(false)),
        ]
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

            match app.render() {
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
