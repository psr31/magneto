use std::ops::Range;

pub trait Renderable {
    fn render<'a, 'b>(&'b mut self, rp: &mut wgpu::RenderPass<'a>)
    where
        'b: 'a;
}

pub trait InstanceRender {
    fn render_instanced<'a, 'b>(&'b mut self, rp: &mut wgpu::RenderPass<'a>, instances: Range<u32>)
    where
        'b: 'a;
}
