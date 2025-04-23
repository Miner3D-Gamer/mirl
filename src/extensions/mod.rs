pub trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    fn sign(self) -> Self;
}

macro_rules! impl_sign {
    ($($t:ty),*) => {
        $(impl Sign for $t {
            fn sign(self) -> Self {
                if self > 0 as $t { 1 as $t }
                else if self < 0 as $t { -1 as $t }
                else { 0 as $t }
            }
        })*
    };
}

impl_sign!(i8, i16, i32, i64, i128, isize, f32, f64);
pub trait IsNumber {
    /// Returns true if the string is a number
    fn is_number(&self) -> bool;
}

impl IsNumber for &str {
    fn is_number(&self) -> bool {
        self.chars().all(|c| c.is_ascii_digit())
    }
}

impl IsNumber for String {
    fn is_number(&self) -> bool {
        self.chars().all(|c| c.is_ascii_digit())
    }
}

pub fn get_sub_vec_of_vec<T: Copy>(
    vec: &Vec<T>,
    width: u32,
    cutout_x: u32,
    cutout_y: u32,
    cutout_width: u32,
    cutout_height: u32,
) -> Vec<T> {
    let mut sub_vec: Vec<T> = Vec::new();

    for y in cutout_y..cutout_y + cutout_height {
        for x in cutout_x..cutout_x + cutout_width {
            let index = (y * width + x) as usize;
            sub_vec.push(vec[index]);
        }
    }
    return sub_vec;
}

mod tuple;
pub use tuple::*;
