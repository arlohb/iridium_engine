use inline_spirv::include_spirv;
use iridium_assets::Assets;
use iridium_graphics::{Material, Mesh, Shader, ShaderInput, ShaderType, Texture, Vertex};
use iridium_maths::VecN;

/// Load a set of assets for a sprite.
///
/// This is specific to this game,
/// so textures are not filtered,
/// it requires `sprite_vertex` to be loaded,
/// and it assumes every sprite has the same material.
///
/// Will create:
/// - `XXX_tex` - texture
/// - `XXX_frag` - fragment shader
/// - `XXX_mat` - material
fn load_sprite_assets(
    sprite_data: Vec<(&str, &str)>,
    frag_shader_spirv: &[u32],
    camera_gpu_data: &iridium_graphics::CameraGpuData,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface_format: wgpu::TextureFormat,
    assets: &mut Assets,
) -> Result<(), String> {
    for (name, image_path) in sprite_data {
        assets.add(
            &format!("{name}_tex"),
            Texture::from_image_bytes(
                device,
                queue,
                &std::fs::read(format!("iridium_example_project/assets/{image_path}"))
                    .map_err(|e| e.to_string())?,
                false,
            ),
        );

        assets.add(
            &format!("{name}_frag"),
            Shader::new(
                device,
                ShaderType::Fragment,
                frag_shader_spirv,
                vec![
                    ShaderInput::Texture(assets.get::<Texture>(&format!("{name}_tex"))?),
                    ShaderInput::Sampler(assets.get::<Texture>(&format!("{name}_tex"))?),
                ],
            ),
        );

        assets.add(
            &format!("{name}_mat"),
            Material::new(
                device,
                surface_format,
                assets.get::<Shader>("sprite_vertex")?,
                camera_gpu_data,
                assets.get::<Shader>(&format!("{name}_frag"))?,
            ),
        );
    }

    Ok(())
}

/// Load the assets needed for the game.
///
/// # Errors
///
/// If a file is missing,
/// or logic was incorrect and assets were loaded in the wrong order.
#[no_mangle]
pub fn load_assets(
    camera_gpu_data: &iridium_graphics::CameraGpuData,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface_format: wgpu::TextureFormat,
    assets: &mut Assets,
) -> Result<(), String> {
    assets.add(
        "sprite_vertex",
        Shader::new(
            device,
            ShaderType::Vertex,
            include_spirv!("assets/vert.hlsl", vert, hlsl, entry = "vs_main", no_debug),
            vec![ShaderInput::Transform],
        ),
    );

    assets.add(
        "fish_mesh",
        Mesh {
            vertices: vec![
                Vertex::new(VecN::new([-1., -1., 0.]), VecN::new([1., 0.])),
                Vertex::new(VecN::new([-1., 1., 0.]), VecN::new([1., 1.])),
                Vertex::new(VecN::new([1., 1., 0.]), VecN::new([0., 1.])),
                Vertex::new(VecN::new([1., -1., 0.]), VecN::new([0., 0.])),
            ],
            indices: vec![0, 3, 2, 0, 2, 1],
        },
    );

    assets.add(
        "quad_offset",
        Mesh {
            vertices: vec![
                Vertex::new(VecN::new([-0.5, 0., 0.]), VecN::new([0., 0.])),
                Vertex::new(VecN::new([-0.5, 1., 0.]), VecN::new([0., 1.])),
                Vertex::new(VecN::new([0.5, 1., 0.]), VecN::new([1., 1.])),
                Vertex::new(VecN::new([0.5, 0., 0.]), VecN::new([1., 0.])),
            ],
            indices: vec![0, 3, 2, 0, 2, 1],
        },
    );

    assets.add(
        "wine_mesh",
        Mesh {
            vertices: vec![
                Vertex::new(
                    VecN::new([-0.5 + 5. / 16., 0., 0.]),
                    VecN::new([5. / 16., 0.]),
                ),
                Vertex::new(
                    VecN::new([-0.5 + 5. / 16., 1., 0.]),
                    VecN::new([5. / 16., 1.]),
                ),
                Vertex::new(
                    VecN::new([0.5 - 5. / 16., 1., 0.]),
                    VecN::new([11. / 16., 1.]),
                ),
                Vertex::new(
                    VecN::new([0.5 - 5. / 16., 0., 0.]),
                    VecN::new([11. / 16., 0.]),
                ),
            ],
            indices: vec![0, 3, 2, 0, 2, 1],
        },
    );

    load_sprite_assets(
        vec![
            ("fish", "FoodSprites/Food/Fish.png"),
            ("wine", "FoodSprites/Food/Wine.png"),
        ],
        include_spirv!(
            "assets/sprite.hlsl",
            frag,
            hlsl,
            entry = "fs_main",
            no_debug
        ),
        camera_gpu_data,
        device,
        queue,
        surface_format,
        assets,
    )?;

    Ok(())
}
