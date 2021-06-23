use super::{Display, Texture};

pub struct BindGroupBuilder<'a> {
    layout: &'a wgpu::BindGroupLayout,
    resources: Vec<wgpu::BindingResource<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new(layout: &'a wgpu::BindGroupLayout) -> BindGroupBuilder<'a> {
        BindGroupBuilder {
            layout,
            resources: Vec::new(),
        }
    }

    // Pushes both a texture and a texture sampler onto the resources (in that order)
    pub fn with_texture(mut self, texture: &'a Texture) -> Self {
        self.resources
            .push(wgpu::BindingResource::TextureView(&texture.view));
        self.resources
            .push(wgpu::BindingResource::Sampler(&texture.sampler));
        self
    }

    // Pushes a uniform buffer onto the resources
    pub fn with_uniform_buffer(mut self, buffer: &'a wgpu::Buffer) -> Self {
        self.resources.push(buffer.as_entire_binding());
        self
    }

    // Builds the BindGroup
    pub fn build(self, dpy: &Display) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = self
            .resources
            .into_iter()
            .enumerate()
            .map(|(i, r)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: r,
            })
            .collect();

        dpy.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: self.layout,
            entries: &entries,
        })
    }
}

pub struct BglBuilder {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BglBuilder {
    pub fn new() -> BglBuilder {
        BglBuilder {
            entries: Vec::new(),
        }
    }

    fn next_index(&self) -> u32 {
        self.entries.len() as u32
    }

    pub fn with_vertex_uniforms(mut self) -> BglBuilder {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.next_index(),
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        self
    }

    pub fn with_texture(mut self) -> BglBuilder {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.next_index(),
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });
        self
    }

    pub fn with_sampler(mut self) -> BglBuilder {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler {
                filtering: true,
                comparison: false,
            },
            count: None,
        });
        self
    }

    pub fn build(self, dpy: &Display) -> wgpu::BindGroupLayout {
        dpy.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &self.entries,
            })
    }
}
