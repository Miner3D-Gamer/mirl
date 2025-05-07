use super::{rgb_to_u32, u32_to_rgba};

// A pixel made to be read FAST
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub color: u32,
}

impl Pixel {
    pub fn new_rgb(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
            color: rgb_to_u32(r, g, b),
        }
    }

    pub fn new_32(color: u32) -> Self {
        let (r, g, b, a) = u32_to_rgba(color);
        Self {
            r,
            g,
            b,
            a,
            color,
        }
    }
}
impl From<Pixel> for u32 {
    fn from(p: Pixel) -> Self {
        p.color
    }
}
