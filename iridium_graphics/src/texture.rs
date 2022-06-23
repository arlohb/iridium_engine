/// Stores wgpu texture and sampler data.
/// 
/// While wgpu separates textures and samplers, it makes sense for them to be one in iridium.
/// 
/// Also stores the binding types for ease of use.
pub struct Texture {
    /// The binding type of the texture.
    pub texture_binding_type: wgpu::BindingType,
    /// The binding type of the sampler.
    pub sampler_binding_type: wgpu::BindingType,
    /// The texture.
    pub texture: wgpu::Texture,
    /// The texture view.
    pub view: wgpu::TextureView,
    /// The sampler.
    pub sampler: wgpu::Sampler,
}

impl Texture {
    /// Creates a new texture from the rgba bytes.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_rgba: &[u8],
        dimensions: (u32, u32),
        filtered: bool,
    ) -> Texture {
        let texture_binding_type = wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: filtered },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        };

        let sampler_binding_type = wgpu::BindingType::Sampler(
            if filtered { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering }
        );

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            label: None,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: if filtered { wgpu::FilterMode::Linear } else { wgpu::FilterMode::Nearest },
            min_filter: if filtered { wgpu::FilterMode::Linear } else { wgpu::FilterMode::Nearest },
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Texture { texture_binding_type, sampler_binding_type, texture, view, sampler }
    }

    /// Creates a new texture from the raw bytes of the image file.
    /// 
    /// While this function can in theory read images of any file type, for now only the png feature of the `image` crate is enabled.
    /// 
    /// In the future I hope to allow any image format supported by the `image` crate, but this would increase compile times by quite a bit.
    pub fn from_image_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_bytes: &[u8],
        filtered: bool,
    ) -> Texture {
        let image_rgba = image::load_from_memory(image_bytes).unwrap().to_rgba8();
        let dimensions = image_rgba.dimensions();

        Texture::new(device, queue, &image_rgba, dimensions, filtered)
    }
}
