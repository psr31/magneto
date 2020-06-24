// Very low level controls over OpenGL VAO's and VBO's
// Although defined as otherwise, these functions are anything but safe
// Take care

use std::mem;
use std::ffi::c_void;

pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        VertexArrayObject { id: vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct Buffer {
    id: u32,
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        Buffer { id: vbo }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

pub fn draw_arrays(verts: usize) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, verts as i32);
    }
}

pub fn vertex_attrib_pointer(index: usize, size: usize, stride: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(index as u32, size as i32, gl::FLOAT, gl::FALSE, (stride * mem::size_of::<f32>()) as i32, (offset * mem::size_of::<f32>()) as *const c_void);
        gl::EnableVertexAttribArray(index as u32);
    }
}

pub fn buffer_data(data: Vec<f32>) {
    unsafe {
        gl::BufferData(gl::ARRAY_BUFFER, (data.len() *  mem::size_of::<f32>()) as isize, mem::transmute(&data[0]), gl::STATIC_DRAW);
    }
}
