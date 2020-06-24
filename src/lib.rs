use sdl2::video::GLProfile;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::Sdl;

pub mod graphics;

pub struct Context {
    pub sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
}

impl Context {
    pub fn new() -> Context {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem.window("Magneto Game", 800, 600)
            .opengl()
            .build()
            .unwrap();
        
        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        Context {
            sdl_context: sdl_context,
            window: window,
            gl_context: ctx,
        }
    }
}