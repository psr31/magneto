use std::mem::size_of;

pub trait HasLayout {
    fn layout(shader_offset: u32) -> Vec<wgpu::VertexAttribute>;
}

pub struct RenderPipelineBuilder<'a> {
    label: Option<&'static str>,
    module: Option<&'a wgpu::ShaderModule>,
    module_entry_point: &'static str,
    layout: Option<&'a wgpu::PipelineLayout>,
    buffer_layouts: Vec<(BufferLayoutType, u64)>,
    depth_state: Option<wgpu::DepthStencilState>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new() -> RenderPipelineBuilder<'a> {
        RenderPipelineBuilder {
            label: None,
            module: None,
            module_entry_point: "main",
            layout: None,
            buffer_layouts: Vec::new(),
            depth_state: None,
        }
    }

    pub fn with_label(&mut self, label: &'static str) -> &mut Self {
        self.label = Some(label);
        self
    }

    pub fn with_module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.module = Some(module);
        self
    }

    pub fn with_module_entry_point(&mut self, ep: &'static str) -> &mut Self {
        self.module_entry_point = ep;
        self
    }

    pub fn with_layout(&mut self, layout: &'a wgpu::PipelineLayout) -> &mut Self {
        self.layout = Some(layout);
        self
    }

    pub fn push_vertex_buffer_layout<T: HasLayout>(&mut self) -> &mut Self {
        let shader_offset = self
            .buffer_layouts
            .iter()
            .map(|(t, _)| t.attribute_count())
            .sum();
        self.buffer_layouts.push((
            BufferLayoutType::Vertex(T::layout(shader_offset)),
            size_of::<T>() as u64,
        ));
        self
    }

    pub fn push_instance_buffer_layout<T: HasLayout>(&mut self) -> &mut Self {
        let shader_offset = self
            .buffer_layouts
            .iter()
            .map(|(t, _)| t.attribute_count())
            .sum();
        self.buffer_layouts.push((
            BufferLayoutType::Instance(T::layout(shader_offset)),
            size_of::<T>() as u64,
        ));
        self
    }

    pub fn with_depth_stencil(&mut self, depth_compare: wgpu::CompareFunction) -> &mut Self {
        self.depth_state = Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        self
    }

    pub fn build(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let module = self
            .module
            .expect("Cannot construct render pipeline without shader module.");

        let mut buffers = Vec::new();
        for vb in &self.buffer_layouts {
            match &*vb {
                (BufferLayoutType::Vertex(v), s) => {
                    buffers.push(wgpu::VertexBufferLayout {
                        array_stride: *s,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &v,
                    });
                }
                (BufferLayoutType::Instance(v), s) => {
                    buffers.push(wgpu::VertexBufferLayout {
                        array_stride: *s,
                        step_mode: wgpu::InputStepMode::Instance,
                        attributes: &v,
                    });
                }
            }
        }

        device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label,
                layout: self.layout,
                vertex: wgpu::VertexState {
                    module: module,
                    entry_point: self.module_entry_point,
                    buffers: &buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: module,
                    entry_point: self.module_entry_point,
                    targets: &[wgpu::ColorTargetState {
                        format: format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                depth_stencil: self.depth_state.to_owned(),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            })
    }
}

enum BufferLayoutType {
    Vertex(Vec<wgpu::VertexAttribute>),
    Instance(Vec<wgpu::VertexAttribute>),
}

impl BufferLayoutType {
    fn attribute_count(&self) -> u32 {
        match self {
            BufferLayoutType::Vertex(v) => v.len() as u32,
            BufferLayoutType::Instance(v) => v.len() as u32,
        }
    }
}
