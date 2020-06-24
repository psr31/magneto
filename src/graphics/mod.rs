pub mod shader;
pub mod vertex_buffer;

use vertex_buffer::VertexArrayObject;
use vertex_buffer::Buffer;

use crate::Context;

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn swap(context: &mut Context) {
    context.window.gl_swap_window();
}

// Very unoptimised rectangle renderer
pub fn draw_rect(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, c: (f32, f32, f32)) {
    ctx.base_rect.vao.bind();
    ctx.base_rect.program.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(w, h, 0.0));
    shader::uniform_mat4(&ctx.base_rect.program, "model", model);
    shader::uniform_mat4(&ctx.base_rect.program, "proj", proj);
    shader::uniform_vec3(&ctx.base_rect.program, "color", glam::vec3(c.0, c.1, c.2));
    vertex_buffer::draw_arrays(6);
    ctx.base_rect.vao.unbind();
    ctx.base_rect.program.unbind();
}