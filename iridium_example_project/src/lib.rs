use inline_spirv::include_spirv;
use iridium_assets::Assets;
use iridium_ecs::{Component, Transform, Velocity, World};
use iridium_graphics::{
    Material, Mesh, Renderable2D, Shader, ShaderInput, ShaderType, Texture, Vertex,
};
use iridium_maths::VecN;

use rand::Rng;

pub mod components;
pub mod systems;

#[no_mangle]
pub fn load_assets(
    camera_gpu_data: &iridium_graphics::CameraGpuData,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface_format: wgpu::TextureFormat,
    assets: &mut Assets,
) {
    assets.add(
        "steak_tex",
        Texture::from_image_bytes(
            device,
            queue,
            include_bytes!("../assets/FoodSprites/Food/Steak.png"),
            false,
        ),
    );

    assets.add(
        "sprite_vertex",
        Shader::new(
            device,
            ShaderType::Vertex,
            include_spirv!("src/vert.hlsl", vert, hlsl, entry = "vs_main"),
            vec![ShaderInput::Transform],
        ),
    );
    assets.add(
        "sprite_fragment",
        Shader::new(
            device,
            ShaderType::Fragment,
            include_spirv!("src/sprite.hlsl", frag, hlsl, entry = "fs_main"),
            vec![
                ShaderInput::Texture(
                    assets
                        .get::<Texture>("steak_tex")
                        .expect("asset 'steak_tex' not found"),
                ),
                ShaderInput::Sampler(
                    assets
                        .get::<Texture>("steak_tex")
                        .expect("asset 'steak_tex' not found"),
                ),
            ],
        ),
    );
    assets.add(
        "uv_test_fragment",
        Shader::new(
            device,
            ShaderType::Fragment,
            include_spirv!("src/uv_test.hlsl", frag, hlsl, entry = "fs_main"),
            vec![],
        ),
    );

    assets.add(
        "steak_mat",
        Material::new(
            device,
            surface_format,
            assets
                .get::<Shader>("sprite_vertex")
                .expect("asset 'sprite_vertex' not found"),
            camera_gpu_data,
            assets
                .get::<Shader>("sprite_fragment")
                .expect("asset 'sprite_fragment' not found"),
        ),
    );

    assets.add(
        "uv_test",
        Material::new(
            device,
            surface_format,
            assets
                .get::<Shader>("sprite_vertex")
                .expect("asset 'sprite_vertex' not found"),
            camera_gpu_data,
            assets
                .get::<Shader>("uv_test_fragment")
                .expect("asset 'uv_test_fragment' not found"),
        ),
    );

    assets.add(
        "quad",
        Mesh {
            vertices: vec![
                Vertex::new(VecN::new([-1., -1., 0.]), VecN::new([0., 0.])),
                Vertex::new(VecN::new([-1., 1., 0.]), VecN::new([0., 1.])),
                Vertex::new(VecN::new([1., 1., 0.]), VecN::new([1., 1.])),
                Vertex::new(VecN::new([1., -1., 0.]), VecN::new([1., 0.])),
            ],
            indices: vec![0, 3, 2, 0, 2, 1],
        },
    );
}

#[no_mangle]
pub fn init_system(world: &mut World, assets: &Assets) {
    let mut rng = rand::thread_rng();

    for i in 0..1000 {
        world.entities.new_entity(
            &format!("Steak {i}"),
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
            &format!("Entity {i}"),
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
