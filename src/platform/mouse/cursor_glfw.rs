use super::{cursor_resolution, BaseCursor, Cursor};
use crate::graphics::{pixmap_to_buffer, rasterize_svg};
use crate::{extensions::*, graphics};
//use crate::misc::copyable_list::buffer_to_copy_list;
/// Load a cursor SVG and replace it's placeholders with actual colors
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: U2,
    main_color: u32,
    secondary_color: u32,
    svg_data: String,
) -> Option<Cursor> {
    const EXPECTED_SIZE: f64 = 24.0; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

    let wanted_size = cursor_resolution(size);

    //let path = get_cursor_path(&cursor.file_path.to_string());
    //let svg_data = std::fs::read_to_string(path).unwrap();
    // if svg has one {}, insert main_color, if svg has two {}, insert main_color, secondary_color

    let result_svg = svg_data
        .replace_first_occurrence(
            "{main}",
            &graphics::u32_to_hex_without_alpha(main_color),
        )
        .replace_first_occurrence(
            "{secondary}",
            &graphics::u32_to_hex_without_alpha(secondary_color),
        );

    let image_data = rasterize_svg(
        result_svg.as_bytes(),
        u32::from(wanted_size),
        u32::from(wanted_size),
    )
    .ok()?;

    // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
    let adjusted_hotspot_x = ((f64::from(cursor.hot_spot_x) / EXPECTED_SIZE)
        * f64::from(wanted_size))
    .round() as u32;
    let adjusted_hotspot_y = ((f64::from(cursor.hot_spot_y) / EXPECTED_SIZE)
        * f64::from(wanted_size))
    .round() as u32;
    Some(Cursor::Glfw((
        pixmap_to_buffer(&image_data),
        adjusted_hotspot_x,
        adjusted_hotspot_y,
    )))
}
