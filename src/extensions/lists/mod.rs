/// A trait for turning `Vec<Option<T>>` into `Option<Vec<T>>`
pub trait CollectOptions
where
    Self: Sized,
{
    /// `Option<Vec<T>>`
    type Output;
    /// Turn `Vec<Option<T>>` into `Option<Vec<T>>`
    fn collect_options(self) -> Self::Output;
}
impl<T> CollectOptions for Vec<Option<T>> {
    type Output = Option<Vec<T>>;
    fn collect_options(self) -> Self::Output {
        self.into_iter().collect()
    }
}
/// Helper functions containing the actual implementations
pub mod helper_functions_list;
use crate::extensions::lists::helper_functions_list::*;

#[const_trait]
/// Add item to list without exceeding the specified maximal size
pub trait ListPushOrReplaceOnMaxSize<T> {
    /// Add item to list without exceeding the specified maximal size
    fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T);
}
#[const_trait]
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
#[const_trait]
/// Returns what it would be if `T` was pushed onto [`Vec<T>`]
pub trait ListCombined<T: Clone + Sized> {
    /// Returns what it would be if `T` was pushed onto [`Vec<T>`]
    fn combined(&self, other: T) -> Vec<T>;
}
#[const_trait]
#[cfg(feature = "num_traits")]
/// Get additions to a list
pub trait ListAverage<T: num_traits::Num + num_traits::NumCast + Copy> {
    /// Get additions to a list
    fn average(&self) -> Option<T>;
}
#[const_trait]
/// Returns if the list has duplicate values
pub trait ListHasDuplicates<T: std::hash::Hash + Eq> {
    /// Returns if the list has duplicate values
    fn has_duplicates(&self) -> bool;
}
#[const_trait]
/// Find the first instance of T
pub trait Index<T: std::cmp::PartialEq> {
    /// Find the first instance of T
    fn find(&self, item: &T) -> Option<usize>;
}
impl<T: std::cmp::Eq> Index<T> for Vec<T> {
    fn find(&self, item: &T) -> Option<usize> {
        find_in_list(self, item)
    }
}

#[const_trait]
/// Get the difference between 2 lists
pub trait ListGetNewItems<'a, T: std::cmp::PartialEq> {
    /// Get what is new in the list compared to another
    fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T>;
    /// Get what is new in the other list compared to this one
    fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T>;
}
#[const_trait]
/// Get the difference between 2 lists
pub trait ListGetNewItemsCloned<T: std::cmp::PartialEq + Clone> {
    /// Get what is new in the list compared to another
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T>;
    /// Get what is new in the other list compared to this one
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T>;
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
#[allow(clippy::use_self)] // No clippy, Self is not allowed in this context
impl<T: std::cmp::PartialEq + Clone> ListGetNewItemsCloned<T> for Vec<T> {
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T> {
        get_difference_new_cloned(old, self)
    }
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T> {
        get_difference_new_cloned(self, new)
    }
}

impl<T: std::clone::Clone> ListCombined<T> for Vec<T> {
    fn combined(&self, other: T) -> Self {
        combined(self, other)
    }
}
#[cfg(feature = "num_traits")]
impl<T: num_traits::Num + num_traits::NumCast + Copy> ListAverage<T>
    for Vec<T>
{
    fn average(&self) -> Option<T> {
        average(self)
    }
}
