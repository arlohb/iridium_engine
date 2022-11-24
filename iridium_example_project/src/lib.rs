use iridium_assets::Assets;
use iridium_ecs::{Component, Transform, Velocity, World};
use iridium_graphics::Renderable2D;
use iridium_maths::VecN;

use rand::Rng;

pub mod components;
pub mod systems;

mod assets;
pub use assets::*;

#[no_mangle]
pub fn init_system(world: &mut World, assets: &Assets) {
    let mut rng = rand::thread_rng();

    for i in 0..1000 {
        world.entities.new_entity(
            &format!("Steak {:03}", i),
            [
                Component::new(Transform {
                    position: VecN::new([
                        rng.gen_range(-1f32..1f32),
                        rng.gen_range(-1f32..1f32),
                        rng.gen_range(0f32..1f32),
                    ]),
                    scale: {
                        let scale = rng.gen_range(0.05f32..0.3f32);

                        VecN::new([scale, scale, 1.])
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
                    assets.get("quad").expect("Asset quad not found"),
                    assets.get("steak_mat").expect("Asset steak_mat not found"),
                )),
            ],
        );

        world.entities.new_entity(
            &format!("Entity {:03}", i),
            [
                Component::new(Transform {
                    position: VecN::new([
                        rng.gen_range(-1f32..1f32),
                        rng.gen_range(-1f32..1f32),
                        rng.gen_range(0f32..1f32),
                    ]),
                    scale: {
                        let scale = rng.gen_range(0.05f32..0.3f32);

                        VecN::new([scale, scale, 1.])
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
                    assets.get("quad").unwrap(),
                    assets.get("uv_test").unwrap(),
                )),
            ],
        );
    }

    world.entities.new_entity(
        "LeftWall",
        [
            Component::new(Transform {
                position: VecN::new([-1., 0., 0.]),
                scale: VecN::new([0.05, 2., 1.]),
                rotation: 0.0f32,
            }),
            Component::new(Renderable2D::new(
                assets.get("quad").unwrap(),
                assets.get("uv_test").unwrap(),
            )),
        ],
    );

    world.entities.new_entity(
        "RightWall",
        [
            Component::new(Transform {
                position: VecN::new([1., 0., 0.]),
                scale: VecN::new([0.05, 2., 1.]),
                rotation: 0.0f32,
            }),
            Component::new(Renderable2D::new(
                assets.get("quad").unwrap(),
                assets.get("uv_test").unwrap(),
            )),
        ],
    );

    world.entities.new_entity(
        "BottomWall",
        [
            Component::new(Transform {
                position: VecN::new([0., -1., 0.]),
                scale: VecN::new([2., 0.05, 1.]),
                rotation: 0.0f32,
            }),
            Component::new(Renderable2D::new(
                assets.get("quad").unwrap(),
                assets.get("uv_test").unwrap(),
            )),
        ],
    );

    world.entities.new_entity(
        "TopWall",
        [
            Component::new(Transform {
                position: VecN::new([0., 1., 0.]),
                scale: VecN::new([2., 0.05, 1.]),
                rotation: 0.0f32,
            }),
            Component::new(Renderable2D::new(
                assets.get("quad").unwrap(),
                assets.get("uv_test").unwrap(),
            )),
        ],
    );
}
