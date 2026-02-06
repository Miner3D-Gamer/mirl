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
pub const fn corner_type_and_delta_to_metric_change<
    T: [const] core::ops::Neg<Output = T> + crate::math::ConstZero + Copy,
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

/// Get the direction from which the target sees the current position
///
/// Intended use in tandem with [`Rectangle::center`](crate::math::collision::Rectangle::center) and [`Rectangle::ratio`](crate::math::collision::Rectangle::get_ratio)
#[must_use]
pub fn direction_point_to_point<
    T: crate::math::ConstZero
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + core::cmp::PartialOrd
        + Copy
        + crate::extensions::Abs,
    const CS: bool,
>(
    current: (T, T),
    target: (T, T),
    target_ratio: T,
) -> crate::directions::Directions {
    let margin_x = target.0 - current.0;
    let margin_y = target.1 - current.1;

    if margin_y.abs() > margin_x.abs() * target_ratio {
        if (CS && margin_y > T::ZERO) || (!CS && margin_y < T::ZERO) {
            crate::directions::Directions::North
        } else {
            crate::directions::Directions::South
        }
    } else if margin_x > T::ZERO {
        crate::directions::Directions::East
    } else {
        crate::directions::Directions::West
    }
}

use crate::math::ConstNumbers128;

/// Get the relative direction from the target to the current [`Rectangle`](crate::math::collision::Rectangle)
#[must_use]
pub fn direction_rect_to_rect<
    T: Copy
        + crate::math::ConstOne
        + core::ops::Add<Output = T>
        + core::ops::Div<Output = T>
        + ConstNumbers128
        + core::cmp::PartialOrd
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + crate::math::ConstZero
        + crate::extensions::Abs,
    const CS: bool,
>(
    current: &crate::math::geometry::Pos2D<
        T,
        crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
    >,
    target: &crate::math::geometry::Pos2D<
        T,
        crate::math::geometry::d2::rectangle::Rectangle<T, CS>,
    >,
) -> crate::directions::Directions {
    direction_point_to_point::<T, CS>(
        current.center(),
        target.center(),
        target.get_ratio(),
    )
}
