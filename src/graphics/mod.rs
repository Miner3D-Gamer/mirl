pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}

pub fn get_sub_vec_of_vec(
    vec: &Vec<u32>,
    width: u32,
    cutout_x: u32,
    cutout_y: u32,
    cutout_width: u32,
    cutout_height: u32,
) -> Vec<u32> {
    let mut sub_vec: Vec<u32> = Vec::new();

    for y in cutout_y..cutout_y + cutout_height {
        for x in cutout_x..cutout_x + cutout_width {
            let index = (y * width + x) as usize;
            sub_vec.push(vec[index]);
        }
    }
    return sub_vec;
}
#[cfg(feature = "imagery")]
pub mod imagery;
#[cfg(feature = "imagery")]
pub use imagery::*;
