use iridium_core::Assets;
use iridium_ecs::{create_component, World};
use iridium_graphics::{create_renderable_2d, MaterialInstance};
use iridium_maths::Vec3;

pub mod components;
pub mod systems;

#[no_mangle]
pub fn init_system(device: &wgpu::Device, world: &mut World, assets: &Assets) {
    world.entities.new_entity("Entity 0", vec![
        create_component! { Transform
            position: Vec3::new(-1., -1., 0.),
            scale: Vec3::new(0.2, 0.2, 0.2),
            rotation: 0.3f32,
        },
        create_component! { Velocity
            velocity: Vec3::new(0.0006, -0.0002, 0.),
        },
        create_renderable_2d(
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
    ]);

    world.entities.new_entity("Entity 1", vec![
        create_component! { Transform
            position: Vec3::new(-1., -1., 0.),
            scale: Vec3::new(0.2, 0.2, 0.2),
            rotation: 0.6f32,
        },
        create_component! { Velocity
            velocity: Vec3::new(0.0001, 0.0004, 0.),
        },
        create_renderable_2d(
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
    ]);

    world.entities.new_entity("LeftWall", vec![
        create_component! { Transform
            position: Vec3::new(-1., 0., 0.),
            scale: Vec3::new(0.05, 2., 1.),
            rotation: 0.0f32,
        },
        create_renderable_2d(
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
    ]);

    world.entities.new_entity("RightWall", vec![
        create_component! { Transform
            position: Vec3::new(1., 0., 0.),
            scale: Vec3::new(0.05, 2., 1.),
            rotation: 0.0f32,
        },
        create_renderable_2d(
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
    ]);

    world.entities.new_entity("BottomWall", vec![
        create_component! { Transform
            position: Vec3::new(0., -1., 0.),
            scale: Vec3::new(2., 0.05, 1.),
            rotation: 0.0f32,
        },
        create_renderable_2d(
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
    ]);

    world.entities.new_entity("TopWall", vec![
        create_component! { Transform
            position: Vec3::new(0., 1., 0.),
            scale: Vec3::new(2., 0.05, 1.),
            rotation: 0.0f32,
        },
        create_renderable_2d(
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
    ]);
}
