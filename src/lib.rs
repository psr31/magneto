use sdl2::video::GLProfile;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::Sdl;

pub mod graphics;
use graphics::shader::Program;
use graphics::vertex_buffer;
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

struct BaseRectangle {
    vao: VertexArrayObject,
    program: Program,
}

impl BaseRectangle {
    fn new() -> BaseRectangle {
        let mut vao = VertexArrayObject::new();
        let mut vbo = Buffer::new();
        vao.bind();
        vbo.bind();
        vertex_buffer::buffer_data(TRIANGLE_BOX.to_vec());
        vertex_buffer::vertex_attrib_pointer(0, 2, 2, 0);
        vbo.unbind();
        vao.unbind();

        let program = Program::new_from_srcs(
            include_str!("../resources/rect_vert.glsl"),
            include_str!("../resources/rect_frag.glsl"),
        );

        BaseRectangle {
            vao: vao,
            program: program,
        }
    }
}

pub struct Context {
    pub sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
    base_rect: BaseRectangle,
    viewport_width: f32,
    viewport_height: f32,
}

impl Context {
    pub fn new(window_title: &str, window_width: u32, window_height: u32) -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem.window(window_title, window_width, window_height)
            .opengl()
            .build()
            .unwrap();
        
        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        let base_rect = BaseRectangle::new();

        Context {
            sdl_context: sdl_context,
            window: window,
            gl_context: ctx,
            base_rect: base_rect,
            viewport_width: window_width as f32,
            viewport_height: window_height as f32,
        }
    }
}