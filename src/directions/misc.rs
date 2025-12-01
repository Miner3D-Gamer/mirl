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
#[cfg(feature = "num_traits")]
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
