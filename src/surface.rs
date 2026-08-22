use wgpu::{Origin3d, Sampler, TextureAspect, TextureView};
use crate::CatEngine;

pub use wgpu::{TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, SamplerDescriptor};

enum WindowWidthHeightAttr {
    Dimension,
    Specific(u32, u32),
    Config,
}

pub enum MultiplierValue {
    Numer(u32),
    MultiplierByWidth(u32),
    MultiplierByHeight(u32),
}

pub struct SurfaceAttributes {
    width_height_attr: WindowWidthHeightAttr,
    depth_or_array_layers: u32,
    mip_level_count: u32,
    sample_count: u32,
    dimension: TextureDimension,
    format: TextureFormat,
    usages: TextureUsages,
    label: Option<&'static str>,
    mip_level: u32,
    origin: Origin3d,
    aspect: TextureAspect,
    offset: u64,
    bytes_per_row: Option<MultiplierValue>,
    rows_per_image: MultiplierValue,
    texture_view_descriptor: TextureViewDescriptor<'static>,
    sampler_descriptor: SamplerDescriptor<'static>,
}

impl SurfaceAttributes {
    pub fn default_attributes_2d() -> Self {
        Self {
            width_height_attr: WindowWidthHeightAttr::Dimension,
            depth_or_array_layers: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usages: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("diffuse texture"),
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: TextureAspect::All,
            offset: 0,
            bytes_per_row: Some(MultiplierValue::MultiplierByWidth(4)),
            rows_per_image: MultiplierValue::MultiplierByHeight(1),
            texture_view_descriptor: TextureViewDescriptor::default(),
            sampler_descriptor: wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                ..Default::default()
            }
        }
    }
    
    pub fn default_attributes_depth() -> Self {
        Self {
            width_height_attr: WindowWidthHeightAttr::Config,
            depth_or_array_layers: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usages: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            label: Some("diffuse texture"),
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: TextureAspect::All,
            offset: 0,
            bytes_per_row: Some(MultiplierValue::MultiplierByWidth(4)),
            rows_per_image: MultiplierValue::MultiplierByHeight(1),
            texture_view_descriptor: TextureViewDescriptor::default(),
            sampler_descriptor: wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual), // 5.
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()

            }
        }
    }
    
    pub fn set_width_height_to_specific(&mut self, width: u32, height: u32) { self.width_height_attr = WindowWidthHeightAttr::Specific(width, height); }
    pub fn set_width_height_to_config(&mut self) { self.width_height_attr = WindowWidthHeightAttr::Config; }
    pub fn set_width_height_to_dimension(&mut self) { self.width_height_attr = WindowWidthHeightAttr::Dimension; }
    pub fn set_depth_or_array_layers(&mut self, depth_or_array_layers: u32) { self.depth_or_array_layers = depth_or_array_layers; }
    pub fn set_mip_level_count(&mut self, mip_level_count: u32) { self.mip_level_count = mip_level_count; }
    pub fn set_sample_count(&mut self, sample_count: u32) { self.sample_count = sample_count; }
    pub fn set_dimension(&mut self, dimension: TextureDimension) { self.dimension = dimension; }
    pub fn set_format(&mut self, format: TextureFormat) { self.format = format; }
    pub fn set_usages(&mut self, usages: TextureUsages) { self.usages = usages; }
    pub fn set_label(&mut self, label: Option<&'static str>) { self.label = label; }
    pub fn set_mip_level(&mut self, mip_level: u32) { self.mip_level = mip_level; }
    pub fn set_origin(&mut self, origin: Origin3d) { self.origin = origin; }
    pub fn set_aspect(&mut self, aspect: TextureAspect) { self.aspect = aspect; }
    pub fn set_offset(&mut self, offset: u64) { self.offset = offset; }
    pub fn set_bytes_per_row(&mut self, bytes_per_row: Option<MultiplierValue>) { self.bytes_per_row = bytes_per_row; }
    pub fn set_rows_per_image(&mut self, rows_per_image: MultiplierValue) { self.rows_per_image = rows_per_image; }
    pub fn set_texture_view_descriptor(&mut self, texture_view_descriptor: TextureViewDescriptor<'static>) { self.texture_view_descriptor = texture_view_descriptor; }
}

pub struct Surface {
    view: TextureView,
    sampler: Sampler,
}

impl Surface {
    pub fn new(file: &str, catengine: &CatEngine, args: SurfaceAttributes) -> Self {
        let diffuse_image = image::open(file).unwrap().to_rgba8();

        let dimensions = diffuse_image.dimensions();


        let texture_size = match args.width_height_attr {
            WindowWidthHeightAttr::Dimension => {
                wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: args.depth_or_array_layers,
                }
            }
            WindowWidthHeightAttr::Specific(width, height) => {
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: args.depth_or_array_layers,
                }
            }
            WindowWidthHeightAttr::Config => {
                 wgpu::Extent3d {
                    width: catengine.config.width.max(1),
                    height: catengine.config.height.max(1),
                    depth_or_array_layers: args.depth_or_array_layers,
                }
            }
        };

        let diffuse_texture = catengine.device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: args.mip_level_count,
                sample_count: args.sample_count,
                dimension: args.dimension,
                format: args.format,
                usage: args.usages,
                label: args.label,
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
        

        catengine.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: args.mip_level,
                origin: args.origin,
                aspect: args.aspect,
            },
            // The actual pixel data
            &diffuse_image.as_raw(),
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: args.offset,
                bytes_per_row: match args.bytes_per_row {
                    None => {None},
                    Some(v) => match v {
                        MultiplierValue::Numer(v) => Some(v),
                        MultiplierValue::MultiplierByHeight(v) => { Some(v * dimensions.1) }
                        MultiplierValue::MultiplierByWidth(v) => { Some(v * dimensions.0) }
                    }
                },
                rows_per_image: match args.rows_per_image {
                        MultiplierValue::Numer(v) => Some(v),
                        MultiplierValue::MultiplierByHeight(v) => { Some(v * dimensions.1) }
                        MultiplierValue::MultiplierByWidth(v) => { Some(v * dimensions.0) }
                    }
            },
            
            texture_size,
        );

        let diffuse_texture_view = diffuse_texture.create_view(&args.texture_view_descriptor);
        let diffuse_sampler = catengine.device.create_sampler(&args.sampler_descriptor);

        Self{ view: diffuse_texture_view, sampler: diffuse_sampler }
    }

    pub fn get_view(&self) -> &TextureView {
        &self.view
    }

    pub fn get_sampler(&self) -> &Sampler {
        &self.sampler
    }
}
