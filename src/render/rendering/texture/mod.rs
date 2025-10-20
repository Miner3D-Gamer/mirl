use crate::{
    graphics::{get_alpha_of_u32, interpolate_color_rgb_f64},
    platform::Buffer,
};
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Draw a Buffer on screen
pub fn draw_buffer_on_buffer<
    const SAFE: bool,
    const TRANSPARENCY: bool,
    const TRANSPARENCY_CHECK: bool,
>(
    canvas: &Buffer,
    texture: &Buffer,
    position: (isize, isize),
    result_dimensions: (usize, usize),
) {
    // Calculate the visible rectangle in canvas coordinates
    let canvas_start_x = if SAFE {
        position.0.max(0)
    } else {
        position.0
    };
    let canvas_start_y = if SAFE {
        position.1.max(0)
    } else {
        position.1
    };
    let canvas_end_x = if SAFE {
        (position.0 + result_dimensions.0 as isize).min(canvas.width as isize)
    } else {
        position.0 + result_dimensions.0 as isize
    };
    let canvas_end_y = if SAFE {
        (position.1 + result_dimensions.1 as isize).min(canvas.height as isize)
    } else {
        position.1 + result_dimensions.1 as isize
    };

    // Early exit if no visible area
    if SAFE
        && (canvas_start_x >= canvas_end_x || canvas_start_y >= canvas_end_y)
    {
        return;
    }

    // Pre-calculate scaling factors
    let x_scale = texture.width as f32 / result_dimensions.0 as f32;
    let y_scale = texture.height as f32 / result_dimensions.1 as f32;

    // Iterate only over the visible area
    for canvas_y in canvas_start_y..canvas_end_y {
        // Calculate which output pixel we're at (0 to result_height-1)
        let output_y = (canvas_y - position.1) as f32;
        // Map to texture space using pixel centers
        let texture_uv_y = (output_y + 0.5).mul_add(y_scale, -0.5);
        let clamped_uv_y =
            texture_uv_y.max(0.0).min((texture.height - 1) as f32);
        let texture_y = clamped_uv_y as usize;

        for canvas_x in canvas_start_x..canvas_end_x {
            // Calculate which output pixel we're at (0 to result_width-1)
            let output_x = (canvas_x - position.0) as f32;
            // Map to texture space using pixel centers
            let texture_uv_x = (output_x + 0.5).mul_add(x_scale, -0.5);
            let clamped_uv_x =
                texture_uv_x.max(0.0).min((texture.width - 1) as f32);
            let texture_x = clamped_uv_x as usize;

            let pixel = texture.get_pixel_unsafe((texture_x, texture_y));

            if TRANSPARENCY {
                let trans = get_alpha_of_u32(pixel);
                if TRANSPARENCY_CHECK && trans == 0 {
                    continue;
                }

                let color = interpolate_color_rgb_f64(
                    canvas.get_pixel((canvas_x as usize, canvas_y as usize)),
                    pixel,
                    trans,
                );
                canvas.set_pixel_unsafe(
                    (canvas_x as usize, canvas_y as usize),
                    color,
                );
            } else {
                canvas.set_pixel_unsafe(
                    (canvas_x as usize, canvas_y as usize),
                    pixel,
                );
            }
        }
    }
}

/// Draw a buffer on another buffer 1 to 1 instead of scaling it
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn draw_buffer_on_buffer_1_to_1<
    const SAFE: bool,
    const TRANSPARENCY: bool,
    const TRANSPARENCY_INTERPOLATION: bool,
    const NICHE_TRANSPARENCY_CHECK: bool,
>(
    canvas: &Buffer,
    texture: &Buffer,
    position: (isize, isize),
) {
    // Calculate the visible rectangle in canvas coordinates
    let canvas_start_x = if SAFE {
        position.0.max(0)
    } else {
        position.0
    };
    let canvas_start_y = if SAFE {
        position.1.max(0)
    } else {
        position.1
    };
    let canvas_end_x = if SAFE {
        (position.0 + texture.width as isize).min(canvas.width as isize)
    } else {
        position.0 + texture.width as isize
    };
    let canvas_end_y = if SAFE {
        (position.1 + texture.height as isize).min(canvas.height as isize)
    } else {
        position.1 + texture.height as isize
    };

    // Early exit if no visible area
    if SAFE
        && (canvas_start_x >= canvas_end_x || canvas_start_y >= canvas_end_y)
    {
        return;
    }

    // Calculate texture offsets for the visible area
    let texture_start_x = (canvas_start_x - position.0) as usize;
    let texture_start_y = (canvas_start_y - position.1) as usize;

    // Iterate only over the visible area
    for canvas_y in canvas_start_y..canvas_end_y {
        for canvas_x in canvas_start_x..canvas_end_x {
            let texture_x =
                texture_start_x + (canvas_x - canvas_start_x) as usize;
            let texture_y =
                texture_start_y + (canvas_y - canvas_start_y) as usize;

            let pixel = texture.get_pixel_unsafe((texture_x, texture_y));

            if TRANSPARENCY {
                let trans = get_alpha_of_u32(pixel);
                if NICHE_TRANSPARENCY_CHECK && trans == 0 {
                    continue;
                }

                let color = if TRANSPARENCY_INTERPOLATION {
                    if NICHE_TRANSPARENCY_CHECK && trans == 255 {
                        pixel
                    } else {
                        crate::graphics::interpolate_color_rgb_f32(
                            canvas.get_pixel_unsafe((
                                canvas_x as usize,
                                canvas_y as usize,
                            )),
                            pixel,
                            trans,
                        )
                    }
                } else if trans != 0 {
                    pixel
                } else {
                    canvas.get_pixel_unsafe((
                        canvas_x as usize,
                        canvas_y as usize,
                    ))
                };
                canvas.set_pixel_unsafe(
                    (canvas_x as usize, canvas_y as usize),
                    color,
                );
            } else {
                canvas.set_pixel_unsafe(
                    (canvas_x as usize, canvas_y as usize),
                    pixel,
                );
            }
        }
    }
}
