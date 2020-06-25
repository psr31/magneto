use crate::graphics::shader::Program;
use crate::graphics::vertex_buffer;
use vertex_buffer::VertexArrayObject;
use vertex_buffer::Buffer;

const TRIANGLE_BOX: [f32; 12] = [
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    0.0, 0.0,
    0.0, 1.0,
    1.0, 1.0,
];

const TRIANGLE_BOX_TEX: [f32; 24] = [
    0.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
    0.0, 1.0, 0.0, 0.0,
    1.0, 1.0, 1.0, 0.0,
];

const TRIANGLE_BOX_TEXT: [f32; 24] = [
    0.0, 0.0, 0.0, 0.0,
    1.0, 0.0, 1.0, 0.0,
    1.0, 1.0, 1.0, 1.0,
    0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
];

pub struct BaseRectangle {
    pub vao: VertexArrayObject,
    pub program: Program,
}

impl BaseRectangle {
    pub fn new_colored() -> BaseRectangle {
        let vao = VertexArrayObject::new();
        let vbo = Buffer::new();
        vao.bind();
        vbo.bind();
        vertex_buffer::buffer_data(TRIANGLE_BOX.to_vec());
        vertex_buffer::vertex_attrib_pointer(0, 2, 2, 0);
        vbo.unbind();
        vao.unbind();

        let program = Program::new_from_srcs(
            include_str!("../../resources/rect_vert.glsl"),
            include_str!("../../resources/rect_frag.glsl"),
        );

        BaseRectangle {
            vao: vao,
            program: program,
        }
    }

    pub fn new_text() -> BaseRectangle {
        let vao = VertexArrayObject::new();
        let vbo = Buffer::new();
        vao.bind();
        vbo.bind();
        vertex_buffer::buffer_data(TRIANGLE_BOX_TEXT.to_vec());
        vertex_buffer::vertex_attrib_pointer(0, 2, 4, 0);
        vertex_buffer::vertex_attrib_pointer(1, 2, 4, 2);
        vbo.unbind();
        vao.unbind();

        let program = Program::new_from_srcs(
            include_str!("../../resources/text_vert.glsl"),
            include_str!("../../resources/text_frag.glsl"),
        );

        BaseRectangle {
            vao: vao,
            program: program,
        }
    }

    pub fn new_textured() -> BaseRectangle {
        let vao = VertexArrayObject::new();
        let vbo = Buffer::new();
        vao.bind();
        vbo.bind();
        vertex_buffer::buffer_data(TRIANGLE_BOX_TEX.to_vec());
        vertex_buffer::vertex_attrib_pointer(0, 2, 4, 0);
        vertex_buffer::vertex_attrib_pointer(1, 2, 4, 2);
        vbo.unbind();
        vao.unbind();

        let program = Program::new_from_srcs(
            include_str!("../../resources/sprite_vert.glsl"),
            include_str!("../../resources/sprite_frag.glsl"),
        );

        BaseRectangle {
            vao: vao,
            program: program,
        }
    }
}
