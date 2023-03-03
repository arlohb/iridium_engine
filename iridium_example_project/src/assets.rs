use inline_spirv::include_spirv;
use iridium_assets::Assets;
use iridium_graphics::{Material, Mesh, Shader, ShaderInput, ShaderType, Texture, Vertex};
use iridium_maths::VecN;

/// Load a set of assets for a sprite.
///
/// This is specific to this game,
/// so textures are not filtered,
/// it requires `default_vertex` to be loaded,
/// and it assumes every sprite has the same material.
///
/// Will create:
/// - `XXX_tex` - texture
/// - `XXX_frag` - fragment shader
/// - `XXX_mat` - material
fn load_sprite_assets(
    sprite_data: Vec<(&str, &str, bool)>,
    frag_shader_spirv: &[u32],
    camera_gpu_data: &iridium_graphics::CameraGpuData,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface_format: wgpu::TextureFormat,
    assets: &mut Assets,
) -> Result<(), String> {
    for (name, image_path, filtered) in sprite_data {
        assets.add(
            &format!("{name}_tex"),
            Texture::from_image_bytes(
                device,
                queue,
                &std::fs::read(format!("iridium_example_project/assets/{image_path}"))
                    .map_err(|e| e.to_string())?,
                filtered,
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
                assets.get::<Shader>("default_vertex")?,
                camera_gpu_data,
                assets.get::<Shader>(&format!("{name}_frag"))?,
            ),
        );
    }

    Ok(())
}

/// Load default assets.
///
/// Eventually this might be added to the engine.
///
/// # Errors
///
/// If assets were loaded in the wrong order.
/// Shouldn't happen if this function was coded correctly.
pub fn load_default_assets(
    camera_gpu_data: &iridium_graphics::CameraGpuData,
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
    assets: &mut Assets,
) -> Result<(), String> {
    assets.add(
        "default_vertex",
        Shader::new(
            device,
            ShaderType::Vertex,
            include_spirv!("assets/vert.hlsl", vert, hlsl, entry = "vs_main", no_debug),
            vec![ShaderInput::Transform],
        ),
    );

    assets.add(
        "default_mesh",
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
        "default_frag",
        Shader::new(
            device,
            ShaderType::Fragment,
            include_spirv!(
                "assets/uv_test.hlsl",
                frag,
                hlsl,
                entry = "fs_main",
                no_debug
            ),
            vec![],
        ),
    );

    assets.add(
        "default_mat",
        Material::new(
            device,
            surface_format,
            assets.get::<Shader>("default_vertex")?,
            camera_gpu_data,
            assets.get::<Shader>("default_frag")?,
        ),
    );

    Ok(())
}

fn create_sphere(points: i32) -> Mesh {
    Mesh {
        vertices: {
            let radius_points = (0..points).into_iter().map(|i| {
                let theta = 2. * std::f32::consts::PI * (i as f32 / points as f32);

                let x = theta.cos();
                let y = theta.sin();

                Vertex::new(
                    VecN::new([x, y, 0.]),
                    VecN::new([x / 2. + 0.5, y / 2. + 0.5]),
                )
            });

            let mut points = vec![Vertex::new(VecN::zero(), VecN::new([0.5, 0.5]))];
            points.extend(radius_points);
            points
        },
        indices: {
            let mut indices = (0..points)
                .into_iter()
                .flat_map(|i| [0, i as u32 + 1, i as u32 + 2])
                .collect::<Vec<u32>>();

            indices.push(0);
            indices.push(points as u32);
            indices.push(1);

            indices
        },
    }
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
    load_default_assets(camera_gpu_data, device, surface_format, assets)?;

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

    assets.add("ball_mesh", create_sphere(16));

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

    assets.add(
        "bg_tex",
        Texture::from_image_bytes(
            device,
            queue,
            &std::fs::read("iridium_example_project/assets/Background.png")
                .map_err(|e| e.to_string())?,
            true,
        ),
    );

    load_sprite_assets(
        vec![
            ("fish", "FoodSprites/Food/Fish.png", false),
            ("wine", "FoodSprites/Food/Wine.png", false),
            ("bg", "Background.png", true),
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
