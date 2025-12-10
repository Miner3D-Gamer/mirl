#[cfg(feature = "std")]
impl<
        T: std::ops::Add<Output = T> + std::marker::Copy + std::cmp::PartialOrd,
        const CS: bool,
    > From<(T, T, T, T)> for Rectangle<T, CS>
{
    fn from(bush: (T, T, T, T)) -> Self {
        Self::new(bush.0, bush.1, bush.2, bush.3)
    }
}
// impl<T, const CS: bool> std::fmt::Display for Rectangle<T, CS> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A simple Rectangle defining computational limits
#[allow(missing_docs)]
pub struct Rectangle<T, const CS: bool> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}
impl<T: Default, const CS: bool> Default for Rectangle<T, CS> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            width: T::default(),
            height: T::default(),
        }
    }
}

#[cfg(feature = "std")]
impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy,
{
    #[must_use]
    /// Create a new Rectangle
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    #[must_use]
    /// Get the current position
    pub const fn get_pos(&self) -> (T, T) {
        (self.x, self.y)
    }
    #[must_use]
    /// Get the current size
    pub const fn get_size(&self) -> (T, T) {
        (self.width, self.height)
    }
    /// Get the current x
    #[must_use]
    pub const fn get_x(&self) -> T {
        self.x
    }
    /// Get the current y
    #[must_use]
    pub const fn get_y(&self) -> T {
        self.y
    }
    /// Get the current width
    #[must_use]
    pub const fn get_width(&self) -> T {
        self.width
    }
    ///Get the current height
    #[must_use]
    pub const fn get_height(&self) -> T {
        self.height
    }
    /// Get the x of the left side of this rectangle
    #[must_use]
    pub const fn left(&self) -> T {
        self.get_x()
    }
    /// Get the x of the right side of this rectangle
    #[must_use]
    pub const fn right(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        self.get_x() + self.get_width()
    }
    /// Get the y of the top side of this rectangle
    #[must_use]
    pub const fn top(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.get_y() + self.get_height()
        } else {
            self.get_y()
        }
    }
    #[must_use]
    /// Get the y of the bottom side of this rectangle
    pub const fn bottom(&self) -> T
    where
        T: [const] core::ops::Add<Output = T> + Copy,
    {
        if BOTTOM_HIGHER {
            self.get_y()
        } else {
            self.get_y() + self.get_height()
        }
    }
    // #[must_use]
    // /// Checks if a point falls within the coordinate range defined by the triangle bounds
    // pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
    //     if BOTTOM_HIGHER {
    //         point.0 >= self.left()
    //             && point.0 <= self.right()
    //             && point.1 >= self.bottom()
    //             && point.1 <= self.top()
    //     } else {
    //         point.0 >= self.left()
    //             && point.0 <= self.right()
    //             && point.1 >= self.top()
    //             && point.1 <= self.bottom()
    //     }
    // }
}
#[cfg(feature = "std")]
macro_rules! impl_const_rectangle_ops {
    ($t:ty) => {
        impl<const CS: bool> Rectangle<$t, CS> {
            #[must_use]
            /// Checks if 2 rectangles collide anywhere
            pub const fn do_areas_intersect(
                &self,
                smaller: &Rectangle<$t, CS>,
            ) -> bool {
                let top = self.top();
                let left = self.left();
                let right = self.right();
                let bottom = self.bottom();
                let top_left = (top, left);
                let top_right = (top, right);
                let bottom_left = (bottom, left);
                let bottom_right = (bottom, right);

                smaller.does_area_contain_point(top_left)
                    || smaller.does_area_contain_point(top_right)
                    || smaller.does_area_contain_point(bottom_left)
                    || smaller.does_area_contain_point(bottom_right)
            }

            #[must_use]
            /// Checks if this area fully includes another area
            pub const fn does_area_fully_include_other_area(
                &self,
                other: &Rectangle<$t, CS>,
            ) -> bool {
                if CS {
                    self.left() <= other.left()
                        && self.right() >= other.right()
                        && self.bottom() <= other.bottom()
                        && self.top() >= other.top()
                } else {
                    self.left() <= other.left()
                        && self.right() >= other.right()
                        && self.bottom() >= other.bottom()
                        && self.top() <= other.top()
                }
            }

            #[must_use]
            /// Checks if a point falls within the coordinate range defined by the rectangle bounds
            pub const fn does_area_contain_point(
                &self,
                point: ($t, $t),
            ) -> bool {
                let (x, y) = point;
                x >= self.left()
                    && x <= self.right()
                    && if !CS {
                        y >= self.top() && y <= self.bottom()
                    } else {
                        y <= self.top() && y >= self.bottom()
                    }
            }
            #[must_use]
            /// Check if point is within margin distance of any edge
            pub const fn is_point_at_edge(
                &self,
                point: ($t, $t),
                margin: $t,
            ) -> bool {
                let (x, y) = point;
                let near_left =
                    x >= self.left() - margin && x <= self.left() + margin;
                let near_right =
                    x >= self.right() - margin && x <= self.right() + margin;
                let near_top =
                    y >= self.top() - margin && y <= self.top() + margin;
                let near_bottom =
                    y >= self.bottom() - margin && y <= self.bottom() + margin;

                near_left || near_right || near_top || near_bottom
            }
            #[must_use]
            #[allow(clippy::too_many_lines)] // Jokes on you clippy, I am NOT touching that as long as it works
            /// Returns which edge/corner a point is near
            /// 0: top-left, 1: top, 2: top-right, 3: right, 4: bottom-right, 5: bottom, 6: bottom-left, 7: left
            /// Returns `u8::MAX` if not within margin of any edge
            pub const fn get_edge_position(
                &self,
                point: ($t, $t),
                margin: $t,
            ) -> u8 {
                let (x, y) = point;

                let (near_left, near_right, near_top, near_bottom) = if CS {
                    let near_left = x >= self.left() - margin
                        && x <= self.left() + margin
                        && y >= self.bottom() - margin
                        && y <= self.top() + margin;
                    let near_right = x >= self.right() - margin
                        && x <= self.right() + margin
                        && y >= self.bottom() - margin
                        && y <= self.top() + margin;
                    let near_top = y >= self.bottom() - margin
                        && y <= self.bottom() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    let near_bottom = y >= self.top() - margin
                        && y <= self.top() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    (near_left, near_right, near_top, near_bottom)
                } else {
                    let near_left = x >= self.left() - margin
                        && x <= self.left() + margin
                        && y >= self.top() - margin
                        && y <= self.bottom() + margin;
                    let near_right = x >= self.right() - margin
                        && x <= self.right() + margin
                        && y >= self.top() - margin
                        && y <= self.bottom() + margin;
                    let near_top = y >= self.top() - margin
                        && y <= self.top() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    let near_bottom = y >= self.bottom() - margin
                        && y <= self.bottom() + margin
                        && x >= self.left() - margin
                        && x <= self.right() + margin;
                    (near_left, near_right, near_top, near_bottom)
                };

                // Extended corner regions - check diagonal extensions beyond the rectangle
                let (
                    in_top_left_diagonal,
                    in_top_right_diagonal,
                    in_bottom_right_diagonal,
                    in_bottom_left_diagonal,
                ) = if CS {
                    let in_top_left_diagonal = (x < self.left()
                        && y > self.bottom())
                        && (self.left() - x + y - self.bottom()) <= margin;
                    let in_top_right_diagonal = (x > self.right()
                        && y > self.bottom())
                        && (x - self.right() + y - self.bottom()) <= margin;
                    let in_bottom_right_diagonal = (x > self.right()
                        && y < self.top())
                        && (x - self.right() + self.top() - y) <= margin;
                    let in_bottom_left_diagonal = (x < self.left()
                        && y < self.top())
                        && (self.left() - x + self.top() - y) <= margin;
                    (
                        in_top_left_diagonal,
                        in_top_right_diagonal,
                        in_bottom_right_diagonal,
                        in_bottom_left_diagonal,
                    )
                } else {
                    let in_top_left_diagonal = (x < self.left()
                        && y < self.top())
                        && (self.left() - x + self.top() - y) <= margin;
                    let in_top_right_diagonal = (x > self.right()
                        && y < self.top())
                        && (x - self.right() + self.top() - y) <= margin;
                    let in_bottom_right_diagonal = (x > self.right()
                        && y > self.bottom())
                        && (x - self.right() + y - self.bottom()) <= margin;
                    let in_bottom_left_diagonal = (x < self.left()
                        && y > self.bottom())
                        && (self.left() - x + y - self.bottom()) <= margin;
                    (
                        in_top_left_diagonal,
                        in_top_right_diagonal,
                        in_bottom_right_diagonal,
                        in_bottom_left_diagonal,
                    )
                };

                // Check corners first (including diagonal extensions)
                if (near_top && near_left) || in_top_left_diagonal {
                    return 0; // top-left
                }
                if (near_top && near_right) || in_top_right_diagonal {
                    return 2; // top-right
                }
                if (near_bottom && near_right) || in_bottom_right_diagonal {
                    return 4; // bottom-right
                }
                if (near_bottom && near_left) || in_bottom_left_diagonal {
                    return 6; // bottom-left
                }

                // Check edges
                if near_top && !near_left && !near_right {
                    return 1; // top
                }
                if near_right && !near_top && !near_bottom {
                    return 3; // right
                }
                if near_bottom && !near_left && !near_right {
                    return 5; // bottom
                }
                if near_left && !near_top && !near_bottom {
                    return 7; // left
                }

                // Not near any edge
                u8::MAX
            }
        }
    };
}
#[cfg(feature = "std")]
impl_const_rectangle_ops!(i8);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(i16);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(i32);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(i64);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(i128);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(isize);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(u8);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(u16);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(u32);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(u64);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(u128);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(usize);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(f32);
#[cfg(feature = "std")]
impl_const_rectangle_ops!(f64);
