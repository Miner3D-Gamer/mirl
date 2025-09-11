/// Add item to list without exceeding the specified maximal size
pub fn add_item_to_max_sized_list<T>(
    list: &mut Vec<T>,
    max_size: usize,
    item: T,
) {
    list.push(item);
    if list.len() < max_size {
        return;
    }
    let to_remove = list.len() - max_size;
    for _ in 0..to_remove {
        list.remove(0);
    }
}
/// Get a 1d cut out from a 1d color list (1d internally, 2d textures)
#[must_use]
pub fn get_sub_vec_of_vec<T: Copy>(
    vec: &[T],
    width: usize,
    cutout_x: usize,
    cutout_y: usize,
    cutout_width: usize,
    cutout_height: usize,
) -> Vec<T> {
    let mut sub_vec: Vec<T> = Vec::new();

    for y in cutout_y..cutout_y + cutout_height {
        for x in cutout_x..cutout_x + cutout_width {
            sub_vec.push(vec[y * width + x]);
        }
    }
    sub_vec
}
/// Returns what it would be if T was pushed onto [`Vec<T>`]
pub fn combined<T: Clone + Sized>(vec: &[T], other: T) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    new_vec.push(other);
    new_vec
}
/// Get the average value of a list
///
/// Returns None if the length of the input is 0
#[must_use]
pub fn average<T: num_traits::Num + num_traits::NumCast + Copy>(
    vec: &[T],
) -> Option<T> {
    let sum: T = vec.iter().copied().fold(T::zero(), |a, b| a + b);
    let len = T::from(vec.len());

    if let Some(length) = len {
        if length == T::zero() {
            return None;
        }
        return Some(sum / length);
    }
    None
}
#[must_use]
/// Get additions to a list
pub fn get_difference_new<'a, T: std::cmp::PartialEq>(
    old: &'a [T],
    new: &'a [T],
) -> Vec<&'a T> {
    let mut result = Vec::new();
    for i in new {
        if !old.contains(i) {
            result.push(i);
        }
    }
    result
}
/// Returns if the list has duplicate values
#[must_use]
pub fn has_duplicates<T: std::hash::Hash + Eq>(vec: &Vec<T>) -> bool {
    let mut seen = std::collections::HashSet::new();
    for item in vec {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}
/// Traits for easy function usage
pub mod traits {
    use crate::extensions::lists::{
        add_item_to_max_sized_list, average, combined, get_difference_new, get_sub_vec_of_vec, has_duplicates
    };

    /// Add item to list without exceeding the specified maximal size
    pub trait ListPushOrReplaceOnMaxSize<T> {
        /// Add item to list without exceeding the specified maximal size
        fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T);
    }
    /// Cut out a 2d area from a 1d array and return it as a 1d array
    pub trait ListGetRegion<T: Copy> {
        /// Cut out a 2d area from a 1d array and return it as a 1d array
        fn get_region(
            &self,
            vec_width: usize,
            cutout_x: usize,
            cutout_y: usize,
            cutout_width: usize,
            cutout_height: usize,
        ) -> Vec<T>;
    }
    /// Returns what it would be if `T` was pushed onto [`Vec<T>`]
    pub trait ListCombined<T: Clone + Sized> {
        /// Returns what it would be if `T` was pushed onto [`Vec<T>`]
        fn combined(&self, other: T) -> Vec<T>;
    }
    /// Get additions to a list
    pub trait ListAverage<T: num_traits::Num + num_traits::NumCast + Copy> {
        /// Get additions to a list
        fn average(&self) -> Option<T>;
    }
    /// Returns if the list has duplicate values
    pub trait ListHasDuplicates<T: std::hash::Hash + Eq> {
        /// Returns if the list has duplicate values
        fn has_duplicates(&self) -> bool;
    }

    /// Get the difference between 2 lists
    pub trait ListGetNewItems<'a, T: std::cmp::PartialEq> {
        /// Get what is new in the list compared to another
        fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T>;
        /// Get what is new in the other list compared to this one
        fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T>;
    }
    impl<T> ListPushOrReplaceOnMaxSize<T> for Vec<T> {
        fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T) {
            add_item_to_max_sized_list(self, max_size, item);
        }
    }
    impl<T: std::cmp::Eq + std::hash::Hash> ListHasDuplicates<T> for Vec<T> {
        fn has_duplicates(&self) -> bool {
            has_duplicates(self)
        }
    }
    impl<T: Copy> ListGetRegion<T> for Vec<T> {
        fn get_region(
            &self,
            vec_width: usize,
            cutout_x: usize,
            cutout_y: usize,
            cutout_width: usize,
            cutout_height: usize,
        ) -> Self {
            get_sub_vec_of_vec(
                self,
                vec_width,
                cutout_x,
                cutout_y,
                cutout_width,
                cutout_height,
            )
        }
    }
    impl<'a, T: std::cmp::PartialEq> ListGetNewItems<'a, T> for Vec<T> {
        fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T> {
            get_difference_new(old, self)
        }
        fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T> {
            get_difference_new(self, new)
        }
    }

    impl<T: std::clone::Clone> ListCombined<T> for Vec<T> {
        fn combined(&self, other: T) -> Self {
            combined(self, other)
        }
    }
    impl<T: num_traits::Num + num_traits::NumCast + Copy> ListAverage<T> for Vec<T> {
        fn average(&self) -> Option<T> {
            average(self)
        }
    }
}
