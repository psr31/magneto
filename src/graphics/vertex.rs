use bytemuck::{Pod, Zeroable};
use super::HasLayout;

/// `Vertex` defines any data which is to be sent to the GPU in a vertex buffer.
///
/// Must also implement the `HasLayout` trait (for layout of values) 
/// as well as the `Pod` and `Zeroable` traits (for converison of data into bytes).
pub trait Vertex : HasLayout + Pod + Zeroable {
    /// Create `Self` with the provided features. Note that you do not have to use all of this data.
    fn with_features(position: [f32; 3], normal: [f32; 3], texture_coord: [f32; 2]) -> Self; 
}

/// Basic Vertex Representation.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BasicVertex {
    /// Relative position.
    pub position: [f32; 3],
    /// Direction of normal.
    pub normal: [f32; 3],
    /// Texture coordinate.
    pub texture_coord: [f32; 2],
}

impl HasLayout for BasicVertex {
    fn layout(shader_offset: u32) -> Vec<wgpu::VertexAttribute> {
        wgpu::vertex_attr_array![shader_offset=>Float32x3,shader_offset+1=>Float32x3,shader_offset+2=>Float32x2].to_vec()
    }
}

impl Vertex for BasicVertex {
    fn with_features(position: [f32; 3], normal: [f32; 3], texture_coord: [f32; 2]) -> Self {
        BasicVertex {
            position,
            normal,
            texture_coord,
        }
    }
}