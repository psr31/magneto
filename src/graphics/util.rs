use std::path::Path;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

use wgpu::util::DeviceExt;

pub trait DeviceUtilExt {
    fn init_vertex_buffer(&self, data: &[u8]) -> wgpu::Buffer;
    fn init_index_buffer(&self, data: &[u32]) -> wgpu::Buffer;
    fn init_uniform_buffer(&self, data: &[u8]) -> wgpu::Buffer;
    fn shader_from_memory(&self, src: &str, name: Option<&str>) -> wgpu::ShaderModule;
    fn shader_from_file<P: AsRef<Path>>(&self, path: P) -> Result<wgpu::ShaderModule>;
}

impl DeviceUtilExt for wgpu::Device {
    /// Utility function to create a read only vertex buffer with the given data.
    fn init_vertex_buffer(&self, data: &[u8]) -> wgpu::Buffer {
        self
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                usage: wgpu::BufferUsage::VERTEX,
            })
    }
        
    /// Utility function to create a read only index buffer.
    ///
    /// Casts `data` into a slice of bytes to be sent to the gpu.
    fn init_index_buffer(&self, data: &[u32]) -> wgpu::Buffer {
        self
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsage::INDEX,
            })
    }

    /// Utility function to create a writable uniform buffer with the given data.
    fn init_uniform_buffer(&self, data: &[u8]) -> wgpu::Buffer {
        self
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            })
    }

    /// Utility function to create a shader module with the given WGSL source string.
    fn shader_from_memory(&self, src: &str, label: Option<&str>) -> wgpu::ShaderModule {
        self
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label,
                flags: wgpu::ShaderFlags::all(),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            })
    }

    /// Utility function to create a shader module with the given WGSL source file.
    fn shader_from_file<P: AsRef<Path>>(&self, path: P) -> Result<wgpu::ShaderModule> {
        // Get file stem for shader label
        let shader_label = path.as_ref().file_stem().map(|s| s.to_str() ).flatten();
        let mut shader_file = File::open(path.as_ref())?;
        let mut shader_source = String::new();
        shader_file.read_to_string(&mut shader_source)?;

        Ok(self.shader_from_memory(&shader_source, shader_label))
    }
}