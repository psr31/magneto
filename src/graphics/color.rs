use std::convert::From;

// An easy to use color object
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<(f32, f32, f32)> for Color {
    fn from(item: (f32, f32, f32)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: 1.0,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(item: (f32, f32, f32, f32)) -> Self {
        Color {
            r: item.0,
            g: item.1,
            b: item.2,
            a: item.3,
        }
    }
}

impl From<(u32, u32, u32, u32)> for Color {
    fn from(item: (u32, u32, u32, u32)) -> Self {
        Color {
            r: item.0 as f32 / 255.0,
            g: item.1 as f32 / 255.0,
            b: item.2 as f32 / 255.0,
            a: item.3 as f32 / 255.0,
        }
    }
}

impl From<(u32, u32, u32)> for Color {
    fn from(item: (u32, u32, u32)) -> Self {
        Color {
            r: item.0 as f32 / 255.0,
            g: item.1 as f32 / 255.0,
            b: item.2 as f32 / 255.0,
            a: 1.0,
        }
    }
}

