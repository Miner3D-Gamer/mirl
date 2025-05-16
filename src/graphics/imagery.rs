use crate::graphics::u32_to_rgb;

use image::{GenericImage, GenericImageView};

pub fn u32_to_image_rgba(color: u32) -> image::Rgba<u8> {
    let (r, g, b) = u32_to_rgb(color);
    image::Rgba([r, g, b, 255])
}
pub fn image_rgba_to_u32(rgba: image::Rgba<u8>) -> u32 {
    let [r, g, b, _a] = rgba.0; // _a is the alpha channel, which is not used in this case
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}
pub fn rgb_to_image_rgba(r: u8, g: u8, b: u8) -> image::Rgba<u8> {
    image::Rgba([r, g, b, 255])
}
pub fn create_empty_image(width: u32, height: u32) -> image::DynamicImage {
    image::DynamicImage::new_rgba8(width, height)
}

pub fn vec_to_img(
    image: Vec<u32>,
    width: u32,
    height: u32,
) -> image::DynamicImage {
    let mut img = create_empty_image(width, height);
    for y in 0..height {
        for x in 0..width {
            let color = image[(y * width + x) as usize];
            img.put_pixel(x as u32, y as u32, u32_to_image_rgba(color));
        }
    }
    img
}

pub fn draw_texture_into_image(
    image: &mut image::DynamicImage,
    texture_width: u16,
    texture_height: u16,
    texture_x: u16,
    texture_y: u16,
    texture: &image::DynamicImage,
) {
    if texture.width() == 0 || texture.height() == 0 {
        return;
    }

    let scaling_x = (texture_width as f32) / (texture.width() as f32);
    let scaling_y = (texture_height as f32) / (texture.height() as f32);

    for x in texture_x..texture_x + texture_width {
        let texture_uv_x = ((x - texture_x) as f32 / scaling_x) as u32;

        for y in texture_y..texture_y + texture_height {
            let texture_uv_y = ((y - texture_y) as f32 / scaling_y) as u32;

            if texture_uv_x < texture.width() && texture_uv_y < texture.height()
            {
                let pixel = texture.get_pixel(texture_uv_x, texture_uv_y);
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
}
pub fn set_image_size(
    image: image::DynamicImage,
    width: u32,
    height: u32,
) -> image::DynamicImage {
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
use std::path::Path;

use super::RawImage;
pub fn load_image(file_path: &str) -> image::DynamicImage {
    // Convert the string path to a Path
    let path = Path::new(file_path);

    // Open and decode the image, panic if error occurs
    match image::open(path) {
        Ok(image) => image,
        Err(e) => panic!("Failed to load image: {}", e),
    }
}

pub fn pixmap_to_dynamic_image(
    ras: &resvg::tiny_skia::Pixmap,
) -> image::DynamicImage {
    let mut img = image::DynamicImage::new(
        ras.width(),
        ras.height(),
        image::ColorType::Rgba8,
    );
    for x in 0..ras.width() {
        for y in 0..ras.height() {
            let color = ras.pixel(x, y).unwrap();
            let (r, g, b) = crate::graphics::shift_hue_rgb(
                color.red(),
                color.green(),
                color.blue(),
                50.0,
            );
            let pixel = image::Rgba([r, g, b, color.alpha()]);
            img.put_pixel(x, y, pixel);
        }
    }
    img
}
// pub fn dynamic_image_to_pixmap(
//     img: image::DynamicImage,
// ) -> resvg::tiny_skia::Pixmap {
//     let mut pixmap =
//         resvg::tiny_skia::Pixmap::new(img.width(), img.height()).unwrap();
//     for x in 0..img.width() {
//         for y in 0..img.height() {
//             let color = img.get_pixel(x, y);
//             pixmap.pixe(x, y, color);
//         }
//     }
//     pixmap
// }

pub fn raw_image_to_dynamic_image(raw_image: &RawImage) -> image::DynamicImage {
    let mut img =
        create_empty_image(raw_image.width as u32, raw_image.height as u32);
    for y in 0..raw_image.height {
        for x in 0..raw_image.width {
            let color = raw_image.data[y * raw_image.width + x];
            img.put_pixel(x as u32, y as u32, u32_to_image_rgba(color));
        }
    }
    img
}

// Make .into() work
impl From<RawImage> for image::DynamicImage {
    fn from(bush: RawImage) -> Self {
        raw_image_to_dynamic_image(&bush)
    }
}

// Automatically convert RawImage to DynamicImage
// impl Deref for RawImage {
//     type Target = image::DynamicImage;

//     fn deref(&self) -> &Self::Target {
//         raw_image_to_dynamic_image(&self)
//     }
// }
use crate::graphics::pixel::Pixel;

impl From<Pixel> for image::Rgba<u8> {
    fn from(p: Pixel) -> Self {
        rgb_to_image_rgba(p.r, p.g, p.b)
    }
}
