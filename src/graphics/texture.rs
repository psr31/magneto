use image::EncodableLayout;
use anyhow::Result;
use super::Display;

pub struct Image {
    pub data: Vec<u8>,
    pub size: wgpu::Extent3d,
    pub layout: wgpu::ImageDataLayout,
}

impl Image {
    pub fn load_from_memory(src: &[u8]) -> Result<Image> {
        let image = image::load_from_memory(src)?.to_rgba8();
        
        Ok(Image {
            data: image.as_bytes().to_vec(),
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(image.width() * 4),
                rows_per_image: std::num::NonZeroU32::new(image.height()),
            },
        })
    }
}

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new_from_bytes(dpy: &Display, src: &[u8], label: Option<&'static str>) -> Result<Texture> {
        let image = Image::load_from_memory(src)?;
        let texture = dpy.device.create_texture(&wgpu::TextureDescriptor {
            label: label,
            size: image.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        dpy.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image.data,
            image.layout,
            image.size
        );
        
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = dpy.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Texture {
            texture,
            view,
            sampler
        })
    }

    // TODO: Make sampler usable
    pub fn new_depth_texture(dpy: &Display) -> Texture {
        let texture = dpy.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: dpy.sc_desc.width,
                height: dpy.sc_desc.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::RENDER_ATTACHMENT,
        });

        let sampler = dpy.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        Texture {
            texture,
            sampler,
            view
        }
    }
}
