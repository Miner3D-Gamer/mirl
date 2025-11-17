#[cfg(feature = "std")]
/// A steepness of 15 and offset of 0.8 makes a nice looking icon (Rough estimates based on trail and error)
pub fn fade_out_buffer(buffer: &mut Buffer, steepness: f32, offset: f32) {
    let cx = buffer.width as f32 / 2.0;
    let cy = buffer.height as f32 / 2.0;
    let max_dist = cx.hypot(cy);

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = dx.hypot(dy) / max_dist;
            let fade = 1.0 - dist;
            let fade =
                1.0 - crate::math::smooth_0_to_1(fade, steepness, offset);
            unsafe {
                let color = *buffer.pointer().add(y * buffer.width + x);
                *buffer.mut_pointer().add(y * buffer.width + x) = color
                    .with_alpha(
                        crate::math::interpolate(0_f32, 255_f32, fade) as u32
                    );
            };
        }
    }
}

#[cfg(feature = "std")]
use std::hash::Hasher;

#[cfg(feature = "std")]
use crate::graphics::ColorManipulation;
#[cfg(feature = "std")]
use crate::Buffer;
#[cfg(feature = "std")]
/// Combine 2 strings
pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    result
}
#[cfg(feature = "std")]
/// Hash a value
pub fn hash_value<T: std::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}

/// Convert the corner type from [`mirl::math::collision::rectangle::Rectangle::get_edge_position`](crate::math::collision::rectangle::Rectangle::get_edge_position) into the appropriate cursor style
#[must_use]
pub const fn corner_type_to_cursor_style(
    corner: u8,
) -> Option<crate::platform::CursorStyle> {
    match corner {
        0 | 4 => Some(crate::platform::CursorStyle::ResizeNWSE),
        1 | 5 => Some(crate::platform::CursorStyle::ResizeVertically),
        2 | 6 => Some(crate::platform::CursorStyle::ResizeNESW),
        3 | 7 => Some(crate::platform::CursorStyle::ResizeHorizontally),
        _ => None,
    }
}
/// # A resizing helper function
///
/// Using the corner type from [`mirl::math::collision::rectangle::Rectangle::get_edge_position`](crate::math::collision::rectangle::Rectangle::get_edge_position) converts the given delta into a change of x, y, width, and height of a rectangle
#[must_use]
#[cfg(feature = "std")]
pub const fn corner_type_and_delta_to_metric_change<
    T: [const] std::ops::Neg<Output = T> + num_traits::ConstZero + Copy,
>(
    corner: u8,
    mouse_pos_delta: (T, T),
) -> (T, T, T, T) {
    match corner {
        0 => (
            mouse_pos_delta.0,
            mouse_pos_delta.1,
            -mouse_pos_delta.0,
            -mouse_pos_delta.1,
        ),
        1 => (T::ZERO, mouse_pos_delta.1, T::ZERO, -mouse_pos_delta.1),
        2 => {
            (T::ZERO, mouse_pos_delta.1, mouse_pos_delta.0, -mouse_pos_delta.1)
        }
        3 => (T::ZERO, T::ZERO, mouse_pos_delta.0, T::ZERO),
        4 => (T::ZERO, T::ZERO, mouse_pos_delta.0, mouse_pos_delta.1),
        5 => (T::ZERO, T::ZERO, T::ZERO, mouse_pos_delta.1),
        6 => {
            (mouse_pos_delta.0, T::ZERO, -mouse_pos_delta.0, mouse_pos_delta.1)
        }
        7 => (mouse_pos_delta.0, T::ZERO, -mouse_pos_delta.0, T::ZERO),
        _ => (T::ZERO, T::ZERO, T::ZERO, T::ZERO),
    }
}

/// A windows only section for misc function
#[cfg(target_os = "windows")]
#[cfg(feature = "system")]
#[cfg(feature = "std")]
pub mod windows {
    // use windows::Win32::System::Diagnostics::Debug::GetThreadContext;
    // use windows::Win32::System::Memory::{
    //     VirtualQuery, MEMORY_BASIC_INFORMATION,
    // };

    use windows::Win32::System::Memory::{
        VirtualQuery, MEMORY_BASIC_INFORMATION,
    };
    #[allow(trivial_casts)]
    #[allow(clippy::ref_as_ptr)]
    /// Check the stack use
    ///
    /// # Errors
    ///
    pub fn get_actual_stack_info() {
        unsafe {
            let current_sp = (&0 as *const i32).cast::<std::ffi::c_void>(); // &0 as *const i32 as *const std::ffi::c_void
            let mut mbi = MEMORY_BASIC_INFORMATION::default();

            // Query the memory region containing current stack pointer
            VirtualQuery(
                Some(current_sp),
                &raw mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            let region_base = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;
            let current_addr = current_sp as usize;

            println!("Memory region base:   0x{region_base:x}");
            println!("Memory region size:   {} MB", region_size / 1024 / 1024);
            println!("Current SP:           0x{current_addr:x}");
            println!(
                "Offset in region:     {} KB",
                (current_addr - region_base) / 1024
            );

            // Stack grows downward, so distance from top of region
            let used_from_top = (region_base + region_size) - current_addr;
            println!("Used from region top: {} KB", used_from_top / 1024);
        }
    }
}

#[macro_export]
/// Converts unsigned types to their signed versions
macro_rules! unsigned_to_signed {
    (u8) => {
        i8
    };
    (u16) => {
        i16
    };
    (u32) => {
        i32
    };
    (u64) => {
        i64
    };
    (u128) => {
        i128
    };
    (usize) => {
        isize
    };
}
#[macro_export]
/// Converts signed types to their unsigned versions
macro_rules! signed_to_unsigned {
    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
}
#[macro_export]
/// Switches between unsigned and signed versions
macro_rules! switch_signing {
    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
    (u8) => {
        i8
    };
    (u16) => {
        i16
    };
    (u32) => {
        i32
    };
    (u64) => {
        i64
    };
    (u128) => {
        i128
    };
    (usize) => {
        isize
    };
}
#[macro_export]
/// Gives the next bigger value type excluding usize/isize
macro_rules! upgrade_type {
    (i8) => {
        i16
    };
    (i16) => {
        i32
    };
    (i32) => {
        i64
    };
    (i64) => {
        i128
    };
    (u8) => {
        u16
    };
    (u16) => {
        u32
    };
    (u32) => {
        u64
    };
    (u64) => {
        u128
    };
    (f32) => {
        f64
    };
    (f64) => {
        f128
    };
}
#[cfg(feature = "std")]
#[allow(clippy::cast_precision_loss)]
/// A helper function to be used inside a `execute_at` render function
pub fn invert_color_below(
    buffer: &mut crate::platform::Buffer,
    xy: (usize, usize),
    color: u32,
) {
    let old = buffer.get_pixel(xy);
    let inverted = crate::graphics::invert_color(old);
    let new = crate::graphics::interpolate_color_rgb_u32_f32(
        inverted,
        old,
        crate::graphics::get_alpha_of_u32(color) as f32 / 255.0,
    );
    crate::render::draw_pixel_safe(buffer, xy, new);
}

#[cfg(feature = "std")]
#[allow(clippy::cast_precision_loss)]
/// A helper function to be used inside a `execute_at` render function
pub fn invert_color_if_same(
    buffer: &mut crate::platform::Buffer,
    xy: (usize, usize),
    color: u32,
) {
    let old = buffer.get_pixel(xy);
    if old == color {
        let inverted = crate::graphics::invert_color(old);

        crate::render::draw_pixel_safe(buffer, xy, inverted);
    }
    // let new = crate::graphics::interpolate_color_rgb_u32_f32(
    //     inverted,
    //     old,
    //     crate::graphics::get_u32_red_of_u32(color) as f32 / 255.0,
    // );
    crate::render::draw_pixel_safe(buffer, xy, color);
}
/// Lists but copyable
#[cfg(feature = "std")]
pub mod copyable_list;

#[allow(clippy::cast_possible_truncation)] // Formats need to be consistent
/// Convert a list of strings into a list of u8
#[must_use]
#[cfg(feature = "std")]
pub fn strings_to_bytes(strings: &Vec<String>) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&(strings.len() as u32).to_le_bytes());

    for s in strings {
        let string_bytes = s.as_bytes();
        bytes.extend_from_slice(&(string_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(string_bytes);
    }

    bytes
}

#[allow(clippy::cast_possible_truncation)] // Formats need to be consistent
#[cfg(feature = "std")]
/// Convert a list of u8 into a list of Strings
#[must_use]
pub fn bytes_to_strings(bytes: &[u8]) -> Vec<String> {
    let mut cursor = 0;
    let mut strings = Vec::new();

    if bytes.len() < 4 {
        return strings;
    }
    let num_strings =
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    cursor += 4;

    for _ in 0..num_strings {
        if cursor + 4 > bytes.len() {
            break;
        }

        let len = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]) as usize;
        cursor += 4;

        if cursor + len > bytes.len() {
            break;
        }

        let string_bytes = &bytes[cursor..cursor + len];
        if let Ok(s) = String::from_utf8(string_bytes.to_vec()) {
            strings.push(s);
        }
        cursor += len;
    }

    strings
}

// /// Turn 2d coordinates into a 1d index
// #[must_use]
// pub fn xy_to_idx<T: mirl::math::NumberWithMonotoneOps>(
//     x: T,
//     y: T,
//     width: T,
// ) -> T {
//     y * width + x
// }
// /// Turn a 1d index into a 2d coordinate
// #[must_use]
// pub fn idx_to_xy<T: mirl::math::NumberWithMonotoneOps + Copy>(
//     idx: T,
//     width: T,
// ) -> (T, T) {
//     let y = idx / width;
//     let x = idx % width;
//     (x, y)
// }
// /// Generate a minefield with a safe area of 3x3
// #[must_use]
// #[cfg(feature = "debug-window")]
// pub fn generate_minefield(
//     width: usize,
//     height: usize,
//     safe_x: usize,
//     safe_y: usize,
//     mines: usize,
// ) -> Vec<bool> {
//     let total_tiles = width * height;
//     let mut id_board: Vec<usize> = 0.repeat_value(total_tiles);
//     let mut board = Vec::with_capacity(total_tiles);
//     let safe_range = 1;

//     for x in -safe_range..safe_range {
//         for y in -safe_range..safe_range {
//             id_board[xy_to_idx(
//                 safe_x.saturated_add_sign(x),
//                 safe_y.saturated_add_sign(y),
//                 width,
//             )] = usize::MAX;
//         }
//     }
//     let offset = id_board.iter().filter(|x| **x == usize::MAX).count();
//     #[allow(clippy::needless_range_loop)] // Clippy, no.
//     for i in 0..total_tiles {
//         if id_board[i] == 0 {
//             id_board[i] = i + offset;
//         } else if id_board[i] == usize::MAX {
//             id_board[i] = 0;
//         }
//     }
//     let mut remaining_mines = mines as f64;
//     for i in id_board {
//         let chance = remaining_mines / (total_tiles - i) as f64;
//         if chance > rand::random() {
//             remaining_mines -= 1.0;
//             board.push(true);
//         } else {
//             board.push(false);
//         }
//     }

//     board
// }

// fn count_mines_around_place(
//     position: (usize, usize),
//     board: &[bool],
//     width: usize,
//     height: usize,
// ) -> usize {
//     let mut mines = 0;
//     for x in -1..1 {
//         for y in -1..1 {
//             let new_pos = position.try_tuple_into().add((x, y));
//             if new_pos.0 > 0
//                 && new_pos.1 > 0
//                 && new_pos.0 < width as isize
//                 && new_pos.1 < height as isize
//                 && board
//                     [xy_to_idx(new_pos.0 as usize, new_pos.1 as usize, width)]
//             {
//                 mines += 1;
//             }
//         }
//     }
//     mines
// }

// /// Puts all numbers into place
// #[must_use]
// pub fn get_numbers_from_minefield(
//     board: &[bool],
//     width: usize,
//     height: usize,
// ) -> Vec<usize> {
//     let mut new_board = Vec::new();
//     for i in 0..board.len() {
//         new_board.push(count_mines_around_place(
//             idx_to_xy(i, width),
//             board,
//             width,
//             height,
//         ));
//     }
//     new_board
// }
// /// Convert state boards to a final visual representation
// #[must_use]
// pub fn get_visual_board(
//     minefield: &[bool],
//     numbers: &[usize],
//     revealed: &[bool],
//     flagged: &[bool],
// ) -> Vec<MinesweeperTile> {
//     let mut result = Vec::new();
//     for i in 0..minefield.len() {
//         if flagged[i] {
//             result.push(MinesweeperTile::Flag);
//         } else if !revealed[i] {
//             result.push(MinesweeperTile::UnRevealed);
//         } else if minefield[i] {
//             result.push(MinesweeperTile::Mine);
//         } else {
//             result.push(match numbers[i] {
//                 0 => MinesweeperTile::Empty,
//                 1 => MinesweeperTile::One,
//                 2 => MinesweeperTile::Two,
//                 3 => MinesweeperTile::Three,
//                 4 => MinesweeperTile::Four,
//                 5 => MinesweeperTile::Five,
//                 6 => MinesweeperTile::Six,
//                 7 => MinesweeperTile::Seven,
//                 8 => MinesweeperTile::Eight,
//                 9 => MinesweeperTile::Nine,
//                 _ => MinesweeperTile::Flag,
//             });
//         }
//     }

//     result
// }
// /// LOGIC
// pub fn handle_minesweeper_logic(
//     flag_mode: bool,
//     click_pos: (usize, usize),
//     mask: &mut [bool],
//     flagged: &mut [bool],
//     minefield: &[bool],
//     numbers: &[usize],
//     width: usize,
// ) -> (Vec<MinesweeperTile>, bool) {
//     let idx = xy_to_idx(click_pos.0, click_pos.1, width);
//     let mut lost = false;
//     if flag_mode {
//         if !mask[idx] {
//             flagged[idx] = !flagged[idx];
//         }
//     } else if !flagged[idx] {
//         mask[idx] = true;
//         if minefield[idx] {
//             lost = true;
//         }
//     }

//     (get_visual_board(minefield, numbers, mask, flagged), lost)
// }

// #[allow(missing_docs)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum MinesweeperTile {
//     UnRevealed,
//     Empty,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
//     Flag,
//     Mine,
// }
/// Horizontal Arrow + Control behavior
#[cfg(feature = "std")]
pub mod skipping_text_type;

#[derive(Debug, Clone, Copy)]
/// A 2D point with an x and a y
pub struct Point2D<T> {
    /// Coordinate on the x axis
    pub x: T,
    /// Coordinate on the y axis
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
/// A 3D point with an x, y, and z
pub struct Point3D<T> {
    /// Coordinate on the x axis
    pub x: T,
    /// Coordinate on the y axis
    pub y: T,
    /// Coordinate on the z axis
    pub z: T,
}
#[cfg(feature = "keycodes")]
#[cfg(feature = "std")]
/// A few lines of helper code for easier keybind handling time
pub mod keybinds;

/// A 2d point specialize for lines and columns
pub mod text_position;

#[macro_export]
/// Create a compile time warning using deprecation
macro_rules! compile_warn {
    ($msg:expr) => {
        #[deprecated(note = $msg)]
        const fn deprecated_trigger() {}
        let _ = deprecated_trigger;
    };
}
#[allow(clippy::implicit_hasher)]
#[cfg(feature = "std")]
/// Instead of key -> value, value -> key
pub fn find_key_by_value<K: Eq + std::hash::Hash + Clone, V: PartialEq>(
    map: &std::collections::HashMap<K, V>,
    value: &V,
) -> Option<K> {
    for (k, v) in map {
        if v == value {
            return Some(k.clone());
        }
    }
    None
}
#[cfg(feature = "discord")]
#[cfg(feature = "std")]
/// Send stuff to discord webhooks, created because `discord-webhooks` kinda sucks and `discord-webhook2` expects to be called in an async environment
pub mod discord;
#[must_use]
#[cfg(feature = "std")]
/// Get the name of the type of the inputted variable
pub fn type_name_of_val<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[allow(clippy::struct_excessive_bools, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// A boolean for each simple Direction
pub struct NormalDirections {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}
impl NormalDirections {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)] // Really clippy? 4 booleans is excessive in your eyes?
    /// Create a simple directional boolean struct
    pub const fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            top_left: top && left,
            top_right: top && right,
            bottom_left: bottom && left,
            bottom_right: bottom && right,
        }
    }
    /// "Yes"
    #[must_use]
    pub const fn all_true() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
            top_left: true,
            top_right: true,
            bottom_left: true,
            bottom_right: true,
        }
    }
    /// "No"
    #[must_use]
    pub const fn all_false() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            top_left: false,
            top_right: false,
            bottom_left: false,
            bottom_right: false,
        }
    }

    /// Check if a direction is true
    #[must_use]
    pub const fn is_direction_allowed_u8(&self, direction: u8) -> bool {
        match direction {
            0 => self.top_left,
            1 => self.top,
            2 => self.top_right,
            3 => self.right,
            4 => self.bottom_right,
            5 => self.bottom,
            6 => self.bottom_left,
            7 => self.left,
            _ => false,
        }
    }
}
/// Check if 2 objects are the same
#[cfg(feature = "std")]
pub trait Comparable {
    /// Convert self to `&dyn std::any::Any`
    fn compare_as_any(&self) -> &dyn std::any::Any;
    /// Check if this and another object are the same
    fn is_same(&self, other: &dyn Comparable) -> bool;
}
#[cfg(feature = "std")]
impl<T: 'static + PartialEq> Comparable for T {
    fn compare_as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_same(&self, other: &dyn Comparable) -> bool {
        // Try to downcast `other` to Foo
        other.compare_as_any().downcast_ref::<Self>() == Some(self)
    }
}
// impl<T: PartialEq> Comparable for T {
//     fn is_same(&self, other: &Self) -> bool {
//         self == other
//     }
// }

// impl<T: std::hash::Hash> Comparable for T {
//     fn is_same(&self, other: &Self) -> bool {
//         let mut own_hasher = std::hash::DefaultHasher::new();
//         let mut other_hasher = std::hash::DefaultHasher::new();
//         self.hash(&mut own_hasher);
//         other.hash(&mut other_hasher);
//         own_hasher == other_hasher
//     }
// }
#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// A camera that is bound to a region
pub struct ScrollableCamera {
    /// The size of the horizontally scrollable region
    pub container_width: f32,
    /// The size of the vertically scrollable region
    pub container_height: f32,
    /// The size of the "thing" moving horizontally in the scrollable region
    pub content_width: f32,
    /// The size of the "thing" moving vertically in the scrollable region
    pub content_height: f32,
    /// The camera offset in the X direction
    pub offset_x: f32,
    /// The camera offset in the Y direction
    pub offset_y: f32,
    /// When scrolling, by what should the incoming scroll be affected horizontally
    pub scroll_multiplier_x: f32,
    /// When scrolling, by what should the incoming scroll be affected vertically
    pub scroll_multiplier_y: f32,
    /// When scrolling horizontally instead of vertically, should the scroll multipliers be switched (`x_mul` -> `y_mul`, `y_mul` -> `x_mul`)
    pub horizontal_context_switch_multipliers: bool,
    /// If the content inside is also scrollable when the bounds are bigger than the content
    pub allow_free_scroll: bool,
}
impl ScrollableCamera {
    /// Scroll within the contained region
    pub fn scroll(&mut self, by: (f32, f32), vertical: bool) {
        if vertical {
            self.offset_x += by.0 * self.scroll_multiplier_x;
            self.offset_y += by.1 * self.scroll_multiplier_y;
        } else if !self.horizontal_context_switch_multipliers {
            self.offset_x += by.1 * self.scroll_multiplier_y;
            self.offset_y += by.0 * self.scroll_multiplier_x;
        } else {
            self.offset_x += by.1 * self.scroll_multiplier_x;
            self.offset_y += by.0 * self.scroll_multiplier_y;
        }

        self.clamp_to_bounds();
    }
    /// Clamp the content to the bounds of the defined region
    pub fn clamp_to_bounds(&mut self) {
        let y_range = if self.allow_free_scroll {
            [self.container_height - self.content_height, 0.0]
        } else {
            [(self.container_height - self.content_height).min(0.0), 0.0]
        };

        let (y_min, y_max) = if y_range[0] <= y_range[1] {
            (y_range[0], y_range[1])
        } else {
            (y_range[1], y_range[0])
        };
        self.offset_y = self.offset_y.clamp(y_min, y_max);

        let x_range = if self.allow_free_scroll {
            [self.container_width - self.content_width, 0.0]
        } else {
            [(self.container_width - self.content_width).min(0.0), 0.0]
        };
        let (x_min, x_max) = if x_range[0] <= x_range[1] {
            (x_range[0], x_range[1])
        } else {
            (x_range[1], x_range[0])
        };
        self.offset_x = self.offset_x.clamp(x_min, x_max);
    }
}
#[derive(Debug)]
/// Choose between 2 values
pub enum TwoOptions<O1, O2> {
    /// Select the first of 2 options
    Option1(O1),
    /// Select the second of 2 options
    Option2(O2),
}
