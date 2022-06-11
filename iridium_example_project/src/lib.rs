use iridium_core::Assets;
use iridium_ecs::{create_components, Component, World};
use iridium_graphics::{create_renderable_2d, MaterialInstance};
use iridium_map_utils::fast_map_any;
use iridium_maths::Vec3;

pub mod components;
pub mod systems;

#[no_mangle]
pub fn init_system(device: &wgpu::Device, world: &mut World, assets: &Assets) {
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
            device,
            MaterialInstance::new(
                device,
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
            device,
            MaterialInstance::new(
                device,
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
