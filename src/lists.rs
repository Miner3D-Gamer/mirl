use std::collections::VecDeque;

pub struct FixedSizeQueue<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> FixedSizeQueue<T> {
    fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn add(&mut self, item: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front(); // Remove the oldest item
        }
        self.deque.push_back(item); // Add the new item
    }

    fn as_vec(&self) -> Vec<&T> {
        self.deque.iter().collect()
    }
}

fn add_item_to_max_sized_list<T>(list: &mut Vec<T>, max_size: usize, item: T) {
    list.push(item);
    if list.len() < max_size {
        return;
    }
    let to_remove = list.len() - max_size;
    for _ in 0..to_remove {
        list.remove(0);
    }
}
