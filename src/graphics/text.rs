use crate::graphics::Texture;
use crate::graphics::shader;
use crate::graphics::vertex_buffer;
use crate::Context;

use std::collections::HashMap;

const CHARS: [char; 65]= [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    ' ',
    ':',
    '.',
];

pub struct Character {
    texture: Option<Texture>,
    size: (u32, u32),
    bearing: (u32, u32),
    advance: u32,
}

pub struct Font {
    characters: HashMap<char, Character>,
}

impl Font {
    pub fn new(ctx: &mut Context, path: &str, size: u32) -> Option<Font> {
        let face = ctx.ft_lib.new_face(path, 0).ok()?;

        face.set_pixel_sizes(0, size);

        let mut characters = HashMap::new();

        for c in CHARS.iter() {
            face.load_char(*c as usize, freetype::face::LoadFlag::RENDER).ok()?;

            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let width = bitmap.width() as u32;
            let rows = bitmap.rows() as u32;
            let top = glyph.bitmap_top() as u32;
            let left = glyph.bitmap_left() as u32;
            let t = match bitmap.buffer() {
                [] => None,
                _ => Texture::new_from_raw_data(bitmap.buffer().to_vec(), width, rows),
            };

            let co = Character {
                texture: t,
                size: (width, rows),
                bearing: (left, top),
                advance: glyph.advance().x as u32,
            };

            characters.insert(*c, co);
        }

        Some(Font {
            characters: characters,
        })
    }
}

pub fn render_text(ctx: &mut Context, text: &str, font: &Font, x: f32, y: f32) {
    // Initialise render
    ctx.text_rect.vao.bind();
    ctx.text_rect.program.bind();
    let proj = glam::Mat4::orthographic_rh_gl(0.0, ctx.viewport_width, ctx.viewport_height, 0.0, -1.0, 0.0);
    shader::uniform_vec3(&ctx.text_rect.program, "color", glam::vec3(0.0, 0.0, 0.0));
    shader::uniform_mat4(&ctx.text_rect.program, "proj", proj);

    // Draw each character
    let mut last_x = x;
    for (i, c) in text.chars().enumerate() {
        let ch = font.characters.get(&c).unwrap();

        let nx = last_x + ch.bearing.0 as f32;
        last_x += (ch.advance >> 6) as f32;
        let ny = (y - ch.bearing.1 as f32) + 48.0;

        if !ch.texture.is_some() {
            continue;
        }

        let texture = ch.texture.as_ref().unwrap();

        let mut model = glam::Mat4::identity();
        model = model * glam::Mat4::from_translation(glam::vec3(nx, ny, 0.0));
        model = model * glam::Mat4::from_scale(glam::vec3(ch.size.0 as f32, ch.size.1 as f32, 0.0));
        shader::uniform_mat4(&ctx.text_rect.program, "model", model);
        texture.bind();
        vertex_buffer::draw_arrays(6);
        texture.unbind();
    }

    ctx.text_rect.program.unbind();
    ctx.text_rect.vao.unbind();
}