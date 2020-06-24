pub mod shader;
pub mod vertex_buffer;
pub mod gl_texture;
pub mod texture_generator;
pub mod base_rectangle;

pub use gl_texture::Texture;

use crate::Context;

pub fn enable_blending() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn swap(context: &mut Context) {
    context.window.gl_swap_window();
}

// Draw rectangle
pub fn draw_rect(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, c: (f32, f32, f32)) {
    ctx.color_rect.vao.bind();
    ctx.color_rect.program.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(w, h, 0.0));
    shader::uniform_mat4(&ctx.color_rect.program, "model", model);
    shader::uniform_mat4(&ctx.color_rect.program, "proj", proj);
    shader::uniform_vec3(&ctx.color_rect.program, "color", glam::vec3(c.0, c.1, c.2));
    vertex_buffer::draw_arrays(6);
    ctx.color_rect.vao.unbind();
    ctx.color_rect.program.unbind();
}

// Draw texture at coordinates
pub fn draw_texture(ctx: &mut Context, texture: &Texture, x: f32, y: f32) {
    ctx.sprite_rect.vao.bind();
    ctx.sprite_rect.program.bind();
    texture.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(texture.width as f32, texture.height as f32, 0.0));
    shader::uniform_mat4(&ctx.sprite_rect.program, "model", model);
    shader::uniform_mat4(&ctx.sprite_rect.program, "proj", proj);
    vertex_buffer::draw_arrays(6);
    texture.unbind();
    ctx.sprite_rect.vao.unbind();
    ctx.sprite_rect.program.unbind();
}

pub fn draw_texture_sized(ctx: &mut Context, texture: &Texture, x: f32, y: f32, w: f32, h: f32) {
    ctx.sprite_rect.vao.bind();
    ctx.sprite_rect.program.bind();
    texture.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(w, h, 0.0));
    shader::uniform_mat4(&ctx.sprite_rect.program, "model", model);
    shader::uniform_mat4(&ctx.sprite_rect.program, "proj", proj);
    vertex_buffer::draw_arrays(6);
    texture.unbind();
    ctx.sprite_rect.vao.unbind();
    ctx.sprite_rect.program.unbind();
}

pub fn draw_texture_scaled(ctx: &mut Context, texture: &Texture, x: f32, y: f32, xs: f32, ys: f32) {
    ctx.sprite_rect.vao.bind();
    ctx.sprite_rect.program.bind();
    texture.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(texture.width as f32 * xs, texture.height as f32 * ys, 0.0));
    shader::uniform_mat4(&ctx.sprite_rect.program, "model", model);
    shader::uniform_mat4(&ctx.sprite_rect.program, "proj", proj);
    vertex_buffer::draw_arrays(6);
    texture.unbind();
    ctx.sprite_rect.vao.unbind();
    ctx.sprite_rect.program.unbind();
}