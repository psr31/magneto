use sdl2::video::GLProfile;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::Sdl;

pub mod graphics;
use graphics::base_rectangle::BaseRectangle;

pub struct Context {
    pub sdl_context: Sdl,
    window: Window,
    gl_context: GLContext,
    color_rect: BaseRectangle,
    sprite_rect: BaseRectangle,
    text_rect: BaseRectangle,
    ft_lib: freetype::Library,
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

        // Initialise Freetype
        let ft_lib = freetype::Library::init().unwrap();

        let color_rect = BaseRectangle::new_colored();
        let sprite_rect = BaseRectangle::new_textured();
        let text_rect = BaseRectangle::new_text();

        graphics::enable_blending();

        Context {
            sdl_context: sdl_context,
            window: window,
            gl_context: ctx,
            color_rect: color_rect,
            sprite_rect: sprite_rect,
            text_rect: text_rect,
            ft_lib: ft_lib,
            viewport_width: window_width as f32,
            viewport_height: window_height as f32,
        }
    }
}