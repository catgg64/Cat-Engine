use wgpu::{BindGroup, BindGroupLayout};
use crate::CatEngine;

pub use wgpu::{TextureDimension, TextureFormat, TextureUsages};

pub struct SurfaceAttributes {
    depth_or_array_layers: u32,
    mip_level_count: u32,
    sample_count: u32,
    dimension: TextureDimension,
    format: TextureFormat,
    usages: TextureUsages,
    label: Option<&'static str>,
    mip_level: u32,
}

impl SurfaceAttributes {
    pub fn default_attributes_2d() -> Self {
        Self {
            depth_or_array_layers: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usages: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("diffuse texture"),
            mip_level: 0,
        }
    }

    pub fn set_depth_or_array_layers(&mut self, v: u32) {
        self.depth_or_array_layers = v;
    }

    pub fn set_mip_level_count(&mut self, v: u32) {
        self.mip_level_count = v;
    }

    pub fn set_sample_count(&mut self, v: u32) {
        self.sample_count = v;
    }

    pub fn set_dimension(&mut self, v: TextureDimension) {
        self.dimension = v;
    }

    pub fn set_format(&mut self, v: TextureFormat) {
        self.format = v;
    }

    pub fn set_usages(&mut self, v: TextureUsages) {
        self.usages = v;
    }

    pub fn set_label(&mut self, v: Option<&'static str>) {
        self.label = v;
    }

    pub fn set_mip_level(&mut self, v: u32) {
        self.mip_level = v
    }

    pub fn get_depth_or_array_layer(&self) -> u32 {
        self.depth_or_array_layers
    }

    pub fn get_mip_level_count(&self) -> u32 {
        self.mip_level_count
    }

    pub fn get_sample_count(&self) -> u32 {
        self.sample_count
    }

    pub fn get_dimension(&self) -> TextureDimension {
        self.dimension
    }

    pub fn get_format(&self) -> TextureFormat {
        self.format
    }
    
    pub fn get_usages(&self) -> TextureUsages {
        self.usages
    }
    
    pub fn get_label(&self) -> Option<&'static str> {
        self.label 
    }

    pub fn get_mip_level(&self) -> u32 {
        self.mip_level
    }
}

pub struct Surface {
    bind_group_layout: BindGroupLayout,
    diffuse_bind_group: BindGroup,
}

impl Surface {
    pub fn new(file: &str, engine: &CatEngine, args: SurfaceAttributes) -> Self {
        let diffuse_image = image::open(file).unwrap().to_rgba8();

        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            depth_or_array_layers: args.get_depth_or_array_layer(),
        };

        let diffuse_texture = engine.device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: args.get_mip_level_count(), // We'll talk about this a little later
                sample_count: args.get_sample_count(),
                dimension: args.get_dimension(),
                // Most images are stored using sRGB, so we need to reflect that here.
                format: args.get_format(),
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: args.get_usages(),
                label: args.get_label(),
                // This is the same as with the SurfaceConfig. It
                // specifies what texture formats can be used to
                // create TextureViews for this texture. The base
                // texture format (Rgba8UnormSrgb in this case) is
                // always supported. Note that using a different
                // texture format is not supported on the WebGL2
                // backend.
                view_formats: &[],
            }
        );
        

        engine.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: args.get_mip_level(),
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &diffuse_image.as_raw(),
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = engine.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            engine.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
    
        let diffuse_bind_group = engine.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        Self{ bind_group_layout: texture_bind_group_layout, diffuse_bind_group }
    }

    pub fn get_bind_group(&self) -> &BindGroup {
        &self.diffuse_bind_group
    }

    pub fn get_bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}
