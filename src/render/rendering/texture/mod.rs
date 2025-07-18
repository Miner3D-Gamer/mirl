use crate::{
    graphics::{get_u32_alpha_of_u32, interpolate_color_rgb_u32, RawImage},
    platform::Buffer,
};
/// Draw a RawImage on screen
pub fn draw_image(
    buffer: &Buffer,
    width: usize,
    height: usize,
    result_width: usize,
    result_height: usize,
    texture_x: isize,
    texture_y: isize,
    texture: &RawImage,
    transparency: bool,
) {
    let start_x = texture_x.max(0);
    let start_y = texture_y.max(0);
    let end_x = (texture_x + result_width as isize).min(width as isize);
    let end_y = (texture_y + result_height as isize).min(height as isize);

    for x in start_x..end_x {
        // Calculate which output pixel we're at (0 to result_width-1)
        let output_x = (x as isize - texture_x) as f32;
        // Map to texture space using pixel centers
        let texture_uv_x =
            (output_x + 0.5) * texture.width as f32 / result_width as f32 - 0.5;
        let clamped_uv_x =
            texture_uv_x.max(0.0).min((texture.width - 1) as f32);

        for y in start_y..end_y {
            // Calculate which output pixel we're at (0 to result_height-1)
            let output_y = (y as isize - texture_y) as f32;
            // Map to texture space using pixel centers
            let texture_uv_y = (output_y + 0.5) * texture.height as f32
                / result_height as f32
                - 0.5;
            let clamped_uv_y =
                texture_uv_y.max(0.0).min((texture.height - 1) as f32);

            let pixel = texture
                .get_pixel((clamped_uv_x as usize, clamped_uv_y as usize));

            if transparency {
                let trans = get_u32_alpha_of_u32(pixel);
                unsafe {
                    let color_place = buffer
                        .pointer
                        .add(y as usize * buffer.width + x as usize);
                    let color = interpolate_color_rgb_u32(
                        *color_place,
                        pixel,
                        trans as f32 / 255.0,
                    );
                    *color_place = color;
                }
            } else {
                unsafe {
                    *buffer
                        .pointer
                        .add(y as usize * buffer.width + x as usize) = pixel;
                }
            }
        }
    }
}
