use super::{BaseCursor, Cursor};
use crate::{
    graphics::{pixmap_to_buffer, rasterize_svg},
    misc::EasyUnwrapUnchecked,
    platform::mouse::{CursorResolution, LoadCursorError},
    prelude::Buffer,
};

//use crate::misc::copyable_list::buffer_to_copy_list;
/// Load a cursor SVG and replace it's placeholders with actual Colors
///
/// # Errors
/// When rasterization fails
#[allow(clippy::needless_pass_by_value)]
pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: CursorResolution,
    svg_data: String,
) -> core::result::Result<Cursor, LoadCursorError> {
    let wanted_size: u32 = size.get_size().easy_unwrap_unchecked(); // This will never error because u32 is bigger than u8

    let image_data =
        rasterize_svg(svg_data.as_bytes(), wanted_size, wanted_size).map_err(
            |x| {
                LoadCursorError::InvalidImageData(x.map_or_else(
                    || "Unable to create SVG from data".to_string(),
                    |x| x.to_string(),
                ))
            },
        )?;

    // // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
    // let adjusted_hotspot_x = ((f64::from(cursor.hot_spot_x) / EXPECTED_SIZE)
    //     * f64::from(wanted_size))
    // .round() as u32;
    // let adjusted_hotspot_y = ((f64::from(cursor.hot_spot_y) / EXPECTED_SIZE)
    //     * f64::from(wanted_size))
    // .round() as u32;
    #[cfg(feature = "cursor_show_hotspot")]
    let mut buffer = pixmap_to_buffer(&image_data);
    #[cfg(not(feature = "cursor_show_hotspot"))]
    let buffer = pixmap_to_buffer(&image_data);
    #[cfg(feature = "cursor_show_hotspot")]
    buffer.set_pixel_safe(
        (cursor.hot_spot_x as usize, cursor.hot_spot_y as usize),
        crate::graphics::colors::RED,
    );
    Ok(cursor_from_buffer(
        buffer,
        (u32::from(cursor.hot_spot_x), u32::from(cursor.hot_spot_y)),
    ))
}
#[must_use]
/// Create a Cursor type for glfw
pub const fn cursor_from_buffer(buffer: Buffer, hotspot: (u32, u32)) -> Cursor {
    Cursor::Glfw((buffer, hotspot.0, hotspot.1))
}
