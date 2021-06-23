
use wgpu::util::DeviceExt;

use super::Display;

pub fn create_init_vertex_buffer(data: &[u8], dpy: &Display) -> wgpu::Buffer {
    dpy.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: data,
        usage: wgpu::BufferUsage::VERTEX,
    })
}

pub fn create_init_uniform_buffer(data: &[u8], dpy: &Display) -> wgpu::Buffer {
    dpy.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: data,
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    })
}


pub fn create_shader_module(display: &Display, src: &str, name: &'static str) -> wgpu::ShaderModule {
    display.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some(name),
        flags: wgpu::ShaderFlags::all(),
        source: wgpu::ShaderSource::Wgsl(src.into())
    })
}