use std::convert::TryInto;
use std::fmt::Debug;
use std::path::Path;

use anyhow::Result;
use wgpu::util::DeviceExt;

use crate::graphics::{Display, Mesh, Vertex};

// TODO: Implement material loading
pub struct Material {
    pub diffuse: [f32; 3],
}

pub struct ObjModel {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub fn load_model<P, V>(path: P, dpy: &Display) -> Result<ObjModel>
where
    P: AsRef<Path> + Debug,
    V: Vertex,
{
    let (models, mats) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    )?;

    let mats = mats?;

    let mut meshes = Vec::new();
    let mut materials = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        let vertices: Vec<V> = mesh
            .positions
            .chunks(3)
            .zip(mesh.texcoords.chunks(2))
            .zip(mesh.normals.chunks(3))
            .map(|((pos, tc), norm)| {
                V::with_features(
                    pos.try_into().unwrap(),
                    norm.try_into().unwrap(),
                    tc.try_into().unwrap(),
                )
            })
            .collect();

        let vb = dpy
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });

        let ib = dpy
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsage::INDEX,
            });

        let mesh = Mesh {
            vertex_buffer: vb,
            index_buffer: Some(ib),
            count: mesh.indices.len() as u32,
        };

        meshes.push(mesh);
    }

    for material in mats {
        materials.push(Material {
            diffuse: material.diffuse,
        })
    }

    let model = ObjModel { meshes, materials };

    Ok(model)
}
