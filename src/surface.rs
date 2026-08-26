use std::sync::Arc;
use crate::CatEngine;

use image::GenericImageView;
pub use wgpu::{TextureDimension, TextureFormat, TextureUsages, TextureDescriptor, TextureViewDescriptor, SamplerDescriptor, Extent3d, AddressMode, FilterMode, MipmapFilterMode};

pub struct Texture {
    texture: Arc<wgpu::Texture>
}

impl Texture {
    pub fn new(catengine: &CatEngine, desc: &TextureDescriptor) -> Self {
        Self { texture: Arc::new(
            catengine.device.create_texture(desc)
        ) }
    }

    pub fn into_view(&self, desc: &TextureViewDescriptor) -> TextureView {
        TextureView {
            view: Arc::new(
                      self.texture.create_view(desc)
                  )
        }
    }

    pub fn get_inner(&self) -> Arc<wgpu::Texture> {
        self.texture.clone()
    }
}

pub struct TextureView {
    view: Arc<wgpu::TextureView>
}

impl TextureView {
    pub fn new(texture: Texture, desc: &TextureViewDescriptor) -> Self {
        Self { view: Arc::new(
            texture.get_inner().create_view(desc)
        ) }
    }

    pub fn get_inner(&self) -> Arc<wgpu::TextureView> {
        self.view.clone()
    }
}

pub struct Sampler {
    sampler: Arc<wgpu::Sampler>
}

impl Sampler {
    pub fn new(catengine: &CatEngine, desc: &SamplerDescriptor) -> Self {
        Self { sampler: Arc::new(
            catengine.device.create_sampler(desc)
        ) }
    }

    pub fn get_inner(&self) -> Arc<wgpu::Sampler> {
        self.sampler.clone()
    }
}

pub struct SurfaceAttributes {
    pub view: TextureView,
    pub sampler: Sampler
}

impl SurfaceAttributes {
    pub fn simple_image(catengine: &CatEngine, img: &image::DynamicImage, label: Option<&str>) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = Texture::new(catengine, &wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        catengine.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture.get_inner(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = TextureView::new(texture, &wgpu::TextureViewDescriptor::default());
        
        let sampler = Sampler::new(catengine, &wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        SurfaceAttributes { view, sampler }
    }
}

pub struct Surface {
    view: TextureView,
    sampler: Sampler,
}

impl Surface {
    pub fn new(attrs: SurfaceAttributes) -> Self {
        Self{ view: attrs.view, sampler: attrs.sampler }
    }

    pub fn get_texture_view(&self) -> &TextureView {
        &self.view
    }

    pub fn get_sampler(&self) -> &Sampler {
        &self.sampler
    }
}
