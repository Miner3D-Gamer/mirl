// #[derive(Debug, Clone, Copy, PartialEq, Eq, std::marker::ConstParamTy)]
// pub enum CoordinateSystem {
//     BottomHigher = true,
//     BottomLower = false,
// }

pub const BOTTOM_HIGHER: bool = true;
pub const BOTTOM_LOWER: bool = false;

impl<
        T: std::ops::Add<Output = T> + std::marker::Copy + std::cmp::PartialOrd,
        const CS: bool,
    > From<(T, T, T, T)> for Rectangle<T, CS>
{
    fn from(bush: (T, T, T, T)) -> Self {
        Rectangle::new(bush.0, bush.1, bush.2, bush.3)
    }
}
// impl<T, const CS: bool> std::fmt::Display for Rectangle<T, CS> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle<T, const CS: bool> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T, const BOTTOM_HIGHER: bool> Rectangle<T, BOTTOM_HIGHER>
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_x(&self) -> T {
        self.x
    }
    pub fn get_y(&self) -> T {
        self.y
    }
    pub fn get_width(&self) -> T {
        self.width
    }
    pub fn get_height(&self) -> T {
        self.height
    }

    pub fn left(&self) -> T {
        self.get_x()
    }

    pub fn right(&self) -> T {
        self.get_x() + self.get_width()
    }

    pub fn top(&self) -> T {
        if BOTTOM_HIGHER {
            self.get_y() + self.get_height()
        } else {
            self.get_y()
        }
    }

    pub fn bottom(&self) -> T {
        if BOTTOM_HIGHER {
            self.get_y()
        } else {
            self.get_y() + self.get_height()
        }
    }

    pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
        if BOTTOM_HIGHER {
            point.0 >= self.left()
                && point.0 <= self.right()
                && point.1 >= self.bottom()
                && point.1 <= self.top()
        } else {
            point.0 >= self.left()
                && point.0 <= self.right()
                && point.1 >= self.top()
                && point.1 <= self.bottom()
        }
    }
}

pub fn does_area_fully_include_other_area<
    T: std::cmp::PartialOrd + std::ops::Add<Output = T> + std::marker::Copy,
    const BOTTOM_HIGHER: bool,
>(
    bigger_area: &Rectangle<T, BOTTOM_HIGHER>,
    smaller_area: &Rectangle<T, BOTTOM_HIGHER>,
) -> bool {
    if BOTTOM_HIGHER {
        return bigger_area.left() <= smaller_area.left()
            && bigger_area.right() >= smaller_area.right()
            && bigger_area.bottom() <= smaller_area.bottom()
            && bigger_area.top() >= smaller_area.top();
    } else {
        return bigger_area.left() <= smaller_area.left()
            && bigger_area.right() >= smaller_area.right()
            && bigger_area.bottom() >= smaller_area.bottom()
            && bigger_area.top() <= smaller_area.top();
    }
}
pub fn do_areas_intersect<
    T: std::cmp::PartialOrd + Copy + std::ops::Add<Output = T>,
    const CS: bool,
>(
    area1: &Rectangle<T, CS>,
    area2: &Rectangle<T, CS>,
) -> bool {
    let top = area1.top();
    let left = area1.left();
    let right = area1.right();
    let bottom = area1.bottom();
    let top_left = (top, left);
    let top_right = (top, right);
    let bottom_left = (bottom, left);
    let bottom_right = (bottom, right);
    return area2.does_area_contain_point(top_left)
        || area2.does_area_contain_point(top_right)
        || area2.does_area_contain_point(bottom_left)
        || area2.does_area_contain_point(bottom_right);
}
