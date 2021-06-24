use std::ops::Range;

use genmesh::generators::{Cube, Plane};
use genmesh::{Triangulate, Vertices};

use crate::graphics::Vertex;

use super::DeviceUtilExt;

/// Represents a buffer of vertices and an optional buffer of indices.
///
/// If `index_buffer` is `Some(..)`, then `count` represents the number of indicies.
///
/// Otherwise, `count` represents the number of vertices.
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,

    pub count: u32,
}

impl Mesh {
    /// Builds a mesh from the given vertices.
    pub fn from_vertices<V>(device: &wgpu::Device, vertices: &[V]) -> Mesh
    where
        V: Vertex,
    {
        Mesh {
            vertex_buffer: device.init_vertex_buffer(bytemuck::cast_slice(vertices)),
            index_buffer: None,
            count: vertices.len() as u32,
        }
    }

    /// Builds a mesh from the given vertices and corresponding indices.
    pub fn from_indexed_vertices<V>(device: &wgpu::Device, vertices: &[V], indices: &[u32]) -> Mesh 
    where
        V: Vertex,
    {
        Mesh {
            vertex_buffer: device.init_vertex_buffer(bytemuck::cast_slice(vertices)),
            index_buffer: Some(device.init_index_buffer(indices)),
            count: indices.len() as u32,
        }
    }

    /// Convenience function to create a cube mesh
    pub fn cube<V>(device: &wgpu::Device) -> Mesh
    where
        V: Vertex,
    {
        let vertices: Vec<V> = Cube::new()
            .map(|genmesh::Quad { x, y, z, w }| {
                genmesh::Quad::new(
                    V::with_features(x.pos.into(), x.normal.into(), [0., 0.]),
                    V::with_features(y.pos.into(), y.normal.into(), [1., 0.]),
                    V::with_features(z.pos.into(), z.normal.into(), [1., 1.]),
                    V::with_features(w.pos.into(), w.normal.into(), [0., 1.]),
                )
            })
            .triangulate()
            .vertices()
            .collect();

        Mesh {
            vertex_buffer: device.init_vertex_buffer(bytemuck::cast_slice(&vertices)),
            index_buffer: None,
            count: vertices.len() as u32,
        }
    }

    /// Convenience function to create a plane mesh
    pub fn plane<V>(device: &wgpu::Device) -> Mesh
    where
        V: Vertex,
    {
        let vertices: Vec<V> = Plane::new()
            .map(|genmesh::Quad { x, y, z, w }| {
                genmesh::Quad::new(
                    V::with_features(x.pos.into(), x.normal.into(), [0., 0.]),
                    V::with_features(y.pos.into(), y.normal.into(), [1., 0.]),
                    V::with_features(z.pos.into(), z.normal.into(), [1., 1.]),
                    V::with_features(w.pos.into(), w.normal.into(), [0., 1.]),
                )
            })
            .triangulate()
            .vertices()
            .collect();

        Mesh {
            vertex_buffer: device.init_vertex_buffer(bytemuck::cast_slice(&vertices)),
            index_buffer: None,
            count: vertices.len() as u32,
        }
    }

    /// Draws range `instances` of the mesh using `renderpass`.
    /// If an index buffer is present, it will draw indexed.
    pub fn draw<'a, 'b>(&'b mut self, renderpass: &mut wgpu::RenderPass<'a>, instances: Range<u32>)
    where
        'b: 'a,
    {
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        if let Some(ref ib) = self.index_buffer {
            renderpass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
            renderpass.draw_indexed(0..self.count, 0, instances);
        } else {
            renderpass.draw(0..self.count, instances);
        }
    }
}
