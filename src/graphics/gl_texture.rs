// Texture functionality and design needs a lot of work

use std::mem;

pub struct Texture {
    id: u32,
    pub width: u32,
    pub height: u32,
}


impl Texture {
    pub fn new() -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        Texture { id: id, width: 0, height: 0 }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

pub fn generate_mipmap() {
    unsafe {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}

pub fn assign_image(data: Vec<u8>, w: u32, h: u32) {
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(&data[0]));
    }
}

pub fn basic_params() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    }
}

pub fn text_params() {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }
}
