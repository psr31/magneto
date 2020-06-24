pub mod shader;
pub mod vertex_buffer;

use vertex_buffer::VertexArrayObject;
use vertex_buffer::Buffer;

use crate::Context;

const TRIANGLE_BOX: [f32; 12] = [
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    0.0, 0.0,
    0.0, 1.0,
    1.0, 1.0,
];

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
    let mut vao = VertexArrayObject::new();
    let mut vbo = Buffer::new();
    vao.bind();
    vbo.bind();
    vertex_buffer::buffer_data(TRIANGLE_BOX.to_vec());
    vertex_buffer::vertex_attrib_pointer(0, 2, 2, 0);
    vbo.unbind();

    ctx.rect_shader.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    let mut model = glam::Mat4::identity();
    model = model * glam::Mat4::from_translation(glam::vec3(x, y, 0.0));
    model = model * glam::Mat4::from_scale(glam::vec3(w, h, 0.0));
    shader::uniform_mat4(&ctx.rect_shader, "model", model);
    shader::uniform_mat4(&ctx.rect_shader, "proj", proj);
    shader::uniform_vec3(&ctx.rect_shader, "color", glam::vec3(c.0, c.1, c.2));
    vertex_buffer::draw_arrays(6);
    vao.unbind();
    ctx.rect_shader.unbind();
}