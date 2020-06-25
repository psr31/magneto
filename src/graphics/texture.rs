/* 
   Plans to add future texture builder, in which all properties 
   (and even image manipulation) can be specified
*/

use crate::graphics::GlTexture;
use crate::graphics::gl_texture::{self, Wrap, Filter};

use std::path::Path;

// A more convenient wrapper around a GlTexture
pub struct Texture {
    pub gl_tex: GlTexture,
    pub wrap: Wrap,
    pub filter: Filter,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    // Loads a texture from the specified path
    pub fn new(path: &Path) -> Option<Texture> {
        let dimage = match image::open(&path) {
            Ok(a) => a,
            Err(_) => return None,
        };

        let image_buffer = dimage.flipv().to_rgba();
        let (width, height) = image_buffer.dimensions();
        
        let tex = GlTexture::new();
        tex.bind();
        gl_texture::assign_image(image_buffer.into_raw(), width, height);
        gl_texture::set_parameters(Wrap::Repeat, Filter::Nearest);
        tex.unbind();

        Some(Texture {
            gl_tex: tex,
            wrap: Wrap::Repeat,
            filter: Filter::Nearest,
            width: width,
            height: height,
        })
    }

    // Loads a texture from memory
    pub fn new_from_memory(data: Vec<u8>) -> Option<Texture> {
        let dimage = match image::load_from_memory(&data) {
            Ok(a) => a,
            Err(_) => return None,
        };

        let image_buffer = dimage.flipv().to_rgba();
        let (width, height) = image_buffer.dimensions();
        
        let tex = GlTexture::new();
        tex.bind();
        gl_texture::assign_image(image_buffer.into_raw(), width, height);
        gl_texture::set_parameters(Wrap::Repeat, Filter::Nearest);
        tex.unbind();

        Some(Texture {
            gl_tex: tex,
            wrap: Wrap::Repeat,
            filter: Filter::Nearest,
            width: width,
            height: height,
        })
    }

    pub fn bind(&self) {
        self.gl_tex.bind();
    }

    pub fn unbind(&self) {
        self.gl_tex.unbind();
    }
}