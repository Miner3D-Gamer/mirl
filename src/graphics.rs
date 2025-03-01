use crate::math::range;

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}
pub fn u32_to_rgba(color: u32) -> image::Rgba<u8> {
    let (r, g, b) = u32_to_rgb(color);
    image::Rgba([r, g, b, 255])
}

pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}
pub fn rgba_to_u32(rgba: image::Rgba<u8>) -> u32 {
    let [r, g, b, _a] = rgba.0; // _a is the alpha channel, which is not used in this case
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}
pub fn rgb_to_rgba(r: u8, g: u8, b: u8) -> image::Rgba<u8> {
    image::Rgba([r, g, b, 255])
}
pub fn create_empty_image(width: u32, height: u32) -> DynamicImage {
    image::DynamicImage::new_rgba8(width, height)
}
use image::GenericImage;

pub fn vec_to_img(image: Vec<u32>, width: u32, height: u32) -> DynamicImage {
    let mut img = create_empty_image(width, height);
    for y in 0..height {
        for x in 0..width {
            let color = image[(y * width + x) as usize];
            img.put_pixel(x as u32, y as u32, u32_to_rgba(color));
        }
    }
    img
}

pub fn draw_texture_into_image(
    image: &mut DynamicImage,
    texture_width: u16,
    texture_height: u16,
    texture_x: u16,
    texture_y: u16,
    texture: &DynamicImage,
) {
    if texture.width() == 0 || texture.height() == 0 {
        return;
    }

    let scaling_x = (texture_width as f32) / (texture.width() as f32);
    let scaling_y = (texture_height as f32) / (texture.height() as f32);

    for x in range(texture_x, texture_x + texture_width) {
        let texture_uv_x = ((x - texture_x) as f32 / scaling_x) as u32;

        for y in range(texture_y, texture_y + texture_height) {
            let texture_uv_y = ((y - texture_y) as f32 / scaling_y) as u32;

            if texture_uv_x < texture.width() && texture_uv_y < texture.height() {
                let pixel = texture.get_pixel(texture_uv_x, texture_uv_y);
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
}
pub fn set_image_size(image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    //not resize, added pixels by new size should be empty
    let mut img = create_empty_image(width, height);
    draw_texture_into_image(
        &mut img,
        image.width() as u16,
        image.height() as u16,
        0,
        0,
        &image,
    );
    return img;
}
use image::{self, DynamicImage, GenericImageView};
use std::path::Path;
pub fn load_image(file_path: &str) -> image::DynamicImage {
    // Convert the string path to a Path
    let path = Path::new(file_path);

    // Open and decode the image, panic if error occurs
    match image::open(path) {
        Ok(image) => image,
        Err(e) => panic!("Failed to load image: {}", e),
    }
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
