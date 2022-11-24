use inline_spirv::include_spirv;
use iridium_assets::Assets;
use iridium_graphics::{Material, Mesh, Shader, ShaderInput, ShaderType, Texture, Vertex};
use iridium_maths::VecN;

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
