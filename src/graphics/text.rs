use crate::graphics::Texture;
use crate::graphics::shader;
use crate::graphics::vertex_buffer;
use crate::graphics::Color;
use crate::Context;

use std::collections::HashMap;

const CHARS: [char; 95]= [
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
    '~',
    '`',
    '!',
    '@',
    '#',
    '$',
    '%',
    '^',
    '&',
    '*',
    '(',
    ')',
    '-',
    '_',
    '+',
    '=',
    ';',
    '"',
    '<',
    '>',
    ',',
    '?',
    '/',
    '{',
    '}',
    '[',
    ']',
    '\\',
    '\'',
    '|',
];

pub struct Text {
    text: String,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub x: f32,
    pub y: f32,
}

impl Text {
    pub fn new(text: &str, font: &Font, color: Color) -> Text {
        let mut pixel_length = 0.0;

        let mut max_height = 0.0;

        for (i,c) in text.chars().enumerate() {
            let ch = font.characters.get(&c).unwrap();
            pixel_length += (ch.advance >> 6) as f32;

            if ch.size.1 as f32 > max_height {
                max_height = ch.size.1 as f32;
            }
        }

        Text {
            text: text.to_string(),
            width: pixel_length,
            height: max_height,
            color: color,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn set_text(&mut self, text: &str, font: &Font) {
        let mut pixel_length = 0.0;

        let mut max_height = 0.0;

        for (i,c) in text.chars().enumerate() {
            let ch = font.characters.get(&c).unwrap();
            pixel_length += (ch.advance >> 6) as f32;

            if ch.size.1 as f32 > max_height {
                max_height = ch.size.1 as f32;
            }
        }

        self.text = text.to_string();
        self.width = pixel_length;
    }

    pub fn draw(&self, ctx: &mut Context, font: &Font) {
        draw_text(ctx, &self.text, font, self.color, self.x, self.y);
    }
}

/// Container for a character texture and its metrics.
/// Not all characters will have textures (for example, space) and this is represented by None for texture.
pub struct Character {
    texture: Option<Texture>,
    size: (i32, i32),
    bearing: (i32, i32),
    advance: u32,
}

pub struct Font {
    characters: HashMap<char, Character>,
    size: u32,
}

impl Font {
    /// Returns a font upon successful initialisation of the specified font and size
    ///
    /// # Arguments
    /// * `ctx` - Global context object
    /// * `path` - A string slice path leading to the desired truetype font file
    /// * `size` - Size to initialise the font with.
    pub fn new(ctx: &mut Context, path: &str, size: u32) -> Option<Font> {
        let face = ctx.ft_lib.new_face(path, 0).ok()?;

        face.set_pixel_sizes(0, size);

        let mut characters = HashMap::new();

        for c in CHARS.iter() {
            face.load_char(*c as usize, freetype::face::LoadFlag::RENDER).ok()?;

            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let width = bitmap.width();
            let rows = bitmap.rows();
            let top = glyph.bitmap_top();
            let left = glyph.bitmap_left();
            let t = match bitmap.buffer() {
                [] => None,
                _ => Texture::new_from_raw_data(bitmap.buffer().to_vec(), width as u32, rows as u32),
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
            size: size,
        })
    }

    pub fn new_from_memory(ctx: &mut Context, data: Vec<u8>, size: u32) -> Option<Font> {
        let face = ctx.ft_lib.new_memory_face(data, 0).ok()?;

        face.set_pixel_sizes(0, size);

        let mut characters = HashMap::new();

        for c in CHARS.iter() {
            face.load_char(*c as usize, freetype::face::LoadFlag::RENDER).ok()?;

            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            let width = bitmap.width();
            let rows = bitmap.rows();
            let top = glyph.bitmap_top();
            let left = glyph.bitmap_left();
            let t = match bitmap.buffer() {
                [] => None,
                _ => Texture::new_from_raw_data(bitmap.buffer().to_vec(), width as u32, rows as u32),
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
            size: size,
        })
    }
}

pub fn draw_text(ctx: &mut Context, text: &str, font: &Font, color: Color, x: f32, y: f32) -> f32 {
    // Initialise render
    ctx.text_rect.vao.bind();
    ctx.text_rect.program.bind();
    shader::uniform_vec3(&ctx.text_rect.program, "color", glam::vec3(color.r, color.g, color.b));
    shader::uniform_mat4(&ctx.text_rect.program, "proj", ctx.projection_matrix);

    // Draw each character
    let fch = font.characters.get(&(&text.chars().nth(0).unwrap())).unwrap().bearing.0;

    let mut last_x = x - fch as f32;
    for (i, c) in text.chars().enumerate() {
        let ch = font.characters.get(&c).unwrap();

        let nx = last_x + ch.bearing.0 as f32;
        last_x += (ch.advance >> 6) as f32;
        let ny = y - (ch.size.1 as f32 - ch.bearing.1 as f32);

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

    last_x - x
}