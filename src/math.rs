pub fn abs(x: i64) -> u64 {
    if x < 0 {
        return (0 - x) as u64;
    }
    return x as u64;
}

pub fn sign(x: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    return x / abs(x) as i64;
}



pub fn range_u16(start: u16, end: u16) -> Vec<u16> {
    (start..=end).collect()
}



pub fn top_clamp(val: u16, max: u16) -> u16 {
    if val > max {
        return max;
    }
    return val;
}

use num_traits::{PrimInt, ToPrimitive};

pub fn range<T>(start: T, end: T) -> Vec<T>
where
    T: PrimInt + ToPrimitive,
{
    (start.to_u64().unwrap()..=end.to_u64().unwrap())
        .map(|i| T::from(i).unwrap())
        .collect()
}
pub fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

pub fn to_num(s: &str) -> i32 {
    if !is_number(&s) {
        return 0;
    }

    return s.parse::<i32>().expect("Failed to parse integer");
}