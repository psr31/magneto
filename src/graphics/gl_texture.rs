use std::mem;

pub enum Format {
    Rgba,
    Red,
}


impl Format{
    pub fn as_gl_id(&self) -> i32 {
        match self {
            Format::Rgba => gl::RGBA as i32,
            Format::Red => gl::RED as i32,
        }
    }
}

pub enum Wrap {
    Repeat,
    ClampEdge,
}

impl Wrap{
    pub fn as_gl_id(&self) -> i32 {
        match self {
            Wrap::Repeat => gl::REPEAT as i32,
            Wrap::ClampEdge => gl::CLAMP_TO_EDGE as i32,
        }
    }
}

pub enum Filter {
    Nearest,
    Linear,
}

impl Filter {
    pub fn as_gl_id(&self) -> i32 {
        match self {
            Filter::Nearest => gl::NEAREST as i32,
            Filter::Linear => gl::LINEAR as i32,
        }
    }
}

pub struct GlTexture {
    id: u32,
}


impl GlTexture {
    pub fn new() -> GlTexture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        GlTexture { id: id }
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

pub fn assign_image(data: Vec<u8>, w: u32, h: u32, format: Format) {
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, format.as_gl_id(), w as i32, h as i32, 0, format.as_gl_id() as u32, gl::UNSIGNED_BYTE, mem::transmute(&data[0]));
    }
}

pub fn set_parameters(wrap: Wrap, filter: Filter) {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap.as_gl_id());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap.as_gl_id());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter.as_gl_id());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter.as_gl_id());
    }
}
