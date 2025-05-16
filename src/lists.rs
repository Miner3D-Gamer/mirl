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
