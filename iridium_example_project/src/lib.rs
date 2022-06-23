use iridium_core::Assets;
use iridium_ecs::{Transform, World, Component, Velocity};
use iridium_graphics::{MaterialInstance, Renderable2D};
use iridium_maths::VecN;

pub mod components;
pub mod systems;

#[no_mangle]
pub fn init_system(device: &wgpu::Device, world: &mut World, assets: &Assets) {
    world.entities.new_entity("Entity 0", vec![
        Component::new(Transform {
            position: VecN::new([-1., -1., 0.]),
            scale: VecN::new([0.2, 0.2, 0.2]),
            rotation: 0.3,
        }),
        Component::new(Velocity {
            velocity: VecN::new([0.0006, -0.0002, 0.]),
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);

    world.entities.new_entity("Entity 1", vec![
        Component::new(Transform {
            position: VecN::new([-1., -1., 0.]),
            scale: VecN::new([0.2, 0.2, 0.2]),
            rotation: 0.6,
        }),
        Component::new(Velocity {
            velocity: VecN::new([0.0001, 0.0004, 0.]),
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);

    world.entities.new_entity("LeftWall", vec![
        Component::new(Transform {
            position: VecN::new([-1., 0., 0.]),
            scale: VecN::new([0.05, 2., 1.]),
            rotation: 0.0f32,
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);

    world.entities.new_entity("RightWall", vec![
        Component::new(Transform {
            position: VecN::new([1., 0., 0.]),
            scale: VecN::new([0.05, 2., 1.]),
            rotation: 0.0f32,
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);

    world.entities.new_entity("BottomWall", vec![
        Component::new(Transform {
            position: VecN::new([0., -1., 0.]),
            scale: VecN::new([2., 0.05, 1.]),
            rotation: 0.0f32,
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);

    world.entities.new_entity("TopWall", vec![
        Component::new(Transform {
            position: VecN::new([0., 1., 0.]),
            scale: VecN::new([2., 0.05, 1.]),
            rotation: 0.0f32,
        }),
        Component::new(Renderable2D::new(
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
        )),
    ]);
}
