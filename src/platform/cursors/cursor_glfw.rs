use super::{cursor_resolution, BaseCursor, Cursor};
use crate::extensions::*;
use crate::graphics::{pixmap_to_raw_image, rasterize_svg, u32_to_hex};
/// Load a cursor SVG and replace it's placeholders with actual colors
pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: U2,
    main_color: u32,
    secondary_color: u32,
    svg_data: String,
) -> Cursor {
    let expected_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

    let wanted_size = cursor_resolution(size);

    //let path = get_cursor_path(&cursor.file_path.to_string());
    //let svg_data = std::fs::read_to_string(path).unwrap();
    // if svg has one {}, insert main_color, if svg has two {}, insert main_color, secondary_color

    let result_svg = svg_data
        .replace_first_occurrence("{}", &u32_to_hex(main_color))
        .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

    let image_data = rasterize_svg(
        &result_svg.as_bytes(),
        wanted_size as u32,
        wanted_size as u32,
    );

    // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
    let adjusted_hotspot_x = ((cursor.hot_spot_x as f64 / expected_size as f64)
        * wanted_size as f64)
        .round() as u32;
    let adjusted_hotspot_y = ((cursor.hot_spot_y as f64 / expected_size as f64)
        * wanted_size as f64)
        .round() as u32;

    return Cursor::Glfw(Some((
        pixmap_to_raw_image(&image_data),
        adjusted_hotspot_x,
        adjusted_hotspot_y,
    )));
}
