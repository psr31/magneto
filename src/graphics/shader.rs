// Very low level shader functions
// Needs work to be more versatile

use std::ffi::CString;
use std::mem;

pub struct Program {
    pub id: u32,
}

impl Program {
    pub fn new_from_srcs(vert_src: &str, frag_src: &str) -> Program {
        let mut id = 0;
        unsafe {
            let cs = CString::new(vert_src.as_bytes()).unwrap();
            let v_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(v_shader, 1, &cs.as_ptr(), std::ptr::null());
            gl::CompileShader(v_shader);

            let cs = CString::new(frag_src.as_bytes()).unwrap();
            let f_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(f_shader, 1, &cs.as_ptr(), std::ptr::null());
            gl::CompileShader(f_shader);

            id = gl::CreateProgram();
            gl::AttachShader(id, v_shader);
            gl::AttachShader(id, f_shader);
            gl::LinkProgram(id);
        }
        Program { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

// Requires pr to be bound before calling
pub fn uniform_mat4(pr: &Program, loc: &str, u: glam::Mat4) {
    unsafe {
        let l = gl::GetUniformLocation(pr.id, CString::new(loc).unwrap().as_ptr());
        gl::UniformMatrix4fv(l, 1, gl::FALSE, mem::transmute(&u.to_cols_array()[0]))
    }
}

pub fn uniform_vec3(pr: &Program, loc: &str, u: glam::Vec3) {
    unsafe {
        let l = gl::GetUniformLocation(pr.id, CString::new(loc).unwrap().as_ptr());
        gl::Uniform3f(l, u.x(), u.y(), u.z());
    }
}