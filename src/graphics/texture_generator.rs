use crate::graphics;
use graphics::gl_texture;
use gl_texture::Texture;

pub fn sprite_texture_from_memory(data: Vec<u8>) -> Option<Texture> {
    let dyn_image =  match image::load_from_memory(&data) {
        Ok(a) => a,
        _ => return None,
    };
    let buffer_image = dyn_image.flipv().to_rgba();

    let mut tex = Texture::new();
    tex.bind();
    gl_texture::basic_params();
    let (width, height) = buffer_image.dimensions();
    gl_texture::assign_image(buffer_image.into_raw(), width, height);
    gl_texture::generate_mipmap();
    tex.unbind();
    tex.width = width;
    tex.height = height;
    Some(tex)
}