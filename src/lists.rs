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
/// Returns what it would be if T was pushed onto Vec<T>
pub fn combined<T: Clone + Sized>(vec: &Vec<T>, other: T) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    new_vec.push(other);
    new_vec
}
/// Get the average value of a list
pub fn average<T: num_traits::Num + num_traits::NumCast + Copy>(
    vec: &Vec<T>,
) -> T {
    let sum: T = vec.iter().copied().fold(T::zero(), |a, b| a + b);
    let len = T::from(vec.len()).unwrap();
    sum / len
}
