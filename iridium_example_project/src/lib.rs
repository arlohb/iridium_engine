use iridium_assets::Assets;
use iridium_ecs::{Transform, World, Component, Velocity};
use iridium_graphics::{MaterialInstance, Renderable2D, Texture, Material, Mesh};
use iridium_maths::VecN;

use rand::Rng;

pub mod components;
pub mod systems;

#[no_mangle]
pub fn init_system(device: &wgpu::Device, world: &mut World, assets: &Assets) {
    let mut rng = rand::thread_rng();

    for i in 0..1000 {
        world.entities.new_entity(&format!("Steak {i}"), vec![
            Component::new(Transform {
                position: VecN::new([
                    rng.gen_range(-1f32..1f32),
                    rng.gen_range(-1f32..1f32),
                    rng.gen_range(0f32..1f32),
                ]),
                scale: {
                    let scale = rng.gen_range(0.05f32..0.3f32);

                    VecN::new([
                        scale,
                        scale,
                        1.,
                    ])
                },
                rotation: rng.gen_range(0f32..std::f32::consts::PI),
            }),
            Component::new(Velocity {
                velocity: VecN::new([
                    rng.gen_range(-0.001f32..0.001f32),
                    rng.gen_range(-0.001f32..0.001f32),
                    0.,
                ]),
            }),
            Component::new(Renderable2D::new(
                device,
                MaterialInstance::new(
                    device,
                    assets.get::<Material>("steak_mat").unwrap(),
                    vec![],
                    vec![],
                    vec![],
                    vec![
                        wgpu::BindingResource::TextureView(&assets.get::<Texture>("steak_tex").unwrap().view),
                        wgpu::BindingResource::Sampler(&assets.get::<Texture>("steak_tex").unwrap().sampler),
                    ],
                ),
                &assets.get::<Mesh>("quad").unwrap(),
            )),
        ]);

        world.entities.new_entity(&format!("Entity {i}"), vec![
            Component::new(Transform {
                position: VecN::new([
                    rng.gen_range(-1f32..1f32),
                    rng.gen_range(-1f32..1f32),
                    rng.gen_range(0f32..1f32),
                ]),
                scale: {
                    let scale = rng.gen_range(0.05f32..0.3f32);

                    VecN::new([
                        scale,
                        scale,
                        1.,
                    ])
                },
                rotation: rng.gen_range(0f32..std::f32::consts::PI),
            }),
            Component::new(Velocity {
                velocity: VecN::new([
                    rng.gen_range(-0.001f32..0.001f32),
                    rng.gen_range(-0.001f32..0.001f32),
                    0.,
                ]),
            }),
            Component::new(Renderable2D::new(
                device,
                MaterialInstance::new(
                    device,
                    assets.get::<Material>("uv_test").unwrap(),
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ),
                &assets.get::<Mesh>("quad").unwrap(),
            )),
        ]);
    }

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
                assets.get::<Material>("uv_test").unwrap(),
                vec![],
                vec![],
                vec![],
                vec![],
            ),
            &assets.get::<Mesh>("quad").unwrap(),
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
                assets.get::<Material>("uv_test").unwrap(),
                vec![],
                vec![],
                vec![],
                vec![],
            ),
            &assets.get::<Mesh>("quad").unwrap(),
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
                assets.get::<Material>("uv_test").unwrap(),
                vec![],
                vec![],
                vec![],
                vec![],
            ),
            &assets.get::<Mesh>("quad").unwrap(),
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
                assets.get::<Material>("uv_test").unwrap(),
                vec![],
                vec![],
                vec![],
                vec![],
            ),
            &assets.get::<Mesh>("quad").unwrap(),
        )),
    ]);
}
