mod frame_history;
pub use frame_history::*;

use iridium_ecs::{Entities, World, create_components};
use iridium_ecs_macros::System;
use iridium_graphics::{create_renderable_2d, MaterialInstance};
use iridium_map_utils::fast_map_any;
use iridium_maths::Vec3;

use crate::{app::App, assets::Assets};

#[derive(System)]
pub struct VelocitySystem {
    activated: bool,
}

impl VelocitySystem {
    fn run(&mut self, entities: &Entities, delta_time: f64) {
        for [mut transform, velocity]
        in entities.query(["Transform", "Velocity"]) {
            // *position.get_mut::<f64>("x") += velocity.get::<f64>("x") * delta_time;
            // *position.get_mut::<f64>("y") += velocity.get::<f64>("y") * delta_time;
            // *position.get_mut::<f64>("z") += velocity.get::<f64>("z") * delta_time;
            *transform.get_mut::<Vec3>("position")  += *velocity.get::<Vec3>("velocity") * delta_time as f32;
            *transform.get_mut::<f32>("rotation") += 0.002 * delta_time as f32;
        }
    }
}

#[derive(System)]
pub struct PositionLoggerSystem {
    activated: bool,
}

impl PositionLoggerSystem {
    fn run(&mut self, entities: &Entities, _delta_time: f64) {
        for [transform]
        in entities.query(["Transform"]) {
            let position = transform.get::<Vec3>("position");
            println!("{} {} {}", position.x, position.y, position.z);
        }
    }
}

pub fn init_system(app: &App, world: &mut World, assets: &Assets) {
    world.entities.new_entity("Entity 0", create_components! {
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

    world.entities.new_entity("Entity 1", create_components! {
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
}
