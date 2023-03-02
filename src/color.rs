pub mod consts {
    use super::{color, RGBA};

    pub const RED: RGBA = color(255, 0, 0, 255);
    pub const GREEN: RGBA = color(0, 255, 0, 255);
    pub const BLUE: RGBA = color(0, 0, 255, 255);
    pub const BLACK: RGBA = color(0, 0, 0, 255);
    pub const WHITE: RGBA = color(255, 255, 255, 255);
}

pub type RGBA = u32;
// 0xRRGGBBAA

pub const fn color(r: u8, g: u8, b: u8, a: u8) -> RGBA {
    (r as u32) << (8 * 3) | (g as u32) << (8 * 2) | (b as u32) << 8 | (a as u32)
}
pub const fn split_color(c: RGBA) -> (u8, u8, u8, u8) {
    ((c >> 24) as u8, (c >> 16) as u8, (c >> 8) as u8, (c) as u8)
}
pub fn alpha_blend(c1: RGBA, c2: RGBA) -> RGBA {
    let (r1, g1, b1, a1) = split_color(c1);
    let (r1, g1, b1, a1) = (r1 as u32, g1 as u32, b1 as u32, a1 as u32);
    let (r2, g2, b2, a2) = split_color(c2);
    let (r2, g2, b2, a2) = (r2 as u32, g2 as u32, b2 as u32, a2 as u32);

    // lerp(a, b, t) = a + (b - a) * t

    let color_max = 255;

    let a3 = a1 + (a2 * (color_max - a1)) / color_max;
    let r3 = r1 + (a3 * (r2 - r1)) / color_max;
    let g3 = g1 + (a3 * (g2 - g1)) / color_max;
    let b3 = b1 + (a3 * (b2 - b1)) / color_max;

    color(r3 as u8, g3 as u8, b3 as u8, a3 as u8)
}
