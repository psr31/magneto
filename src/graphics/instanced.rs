use bytemuck::{Pod, Zeroable};

use super::{Display, Renderable, HasLayout, Mesh, create_init_vertex_buffer};

/// Represents a Mesh with a set of instances.
pub struct Instanced<T> 
where
    T: HasLayout + Pod + Zeroable,
{
    mesh: Mesh,
    instances: Vec<T>,
    instance_buffer: Option<wgpu::Buffer>,
}

impl<T> Instanced<T>
where
    T: HasLayout + Pod + Zeroable,
{
    pub fn from_mesh(mesh: Mesh) -> Instanced<T> {
        Instanced {
            mesh,
            instances: Vec::new(),
            instance_buffer: None,
        }
    }

    pub fn push_instance(&mut self, inst: T) {
        self.instances.push(inst);
    }

    pub fn extend_instances(&mut self, insts: Vec<T>) {
        self.instances.extend(insts);
    }

    /// Updates the internal instance buffer using the pushed instances.
    ///
    /// You must call this before rendering the instances.
    pub fn create_instance_buffer(&mut self, dpy: &Display) {
        self.instance_buffer = Some(create_init_vertex_buffer(bytemuck::cast_slice(&self.instances), dpy));
    }
}

impl<T> Renderable for Instanced<T>
where
    T: HasLayout + Pod + Zeroable
{
    /// Renders each instance contained with the index buffer.
    ///
    /// # Arguments
    ///
    /// * `renderpass` - The RenderPass to use.
    ///
    /// # Panics
    ///
    /// * A instance buffer has not yet been created using `create_instance_buffer`.
    /// * New instances have been pushed since called `create_instance_buffer`.
    fn render<'a, 'b>(&'b mut self, renderpass: &mut wgpu::RenderPass<'a>)
    where
        'b: 'a
    {
        let buffer = self.instance_buffer.as_ref().expect("Cannot render instanced without first creating an instance buffer.");
        renderpass.set_vertex_buffer(1, buffer.slice(..));
        self.mesh.draw(renderpass, 0..self.instances.len() as u32);
    }
}