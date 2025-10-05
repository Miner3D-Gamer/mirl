
use crate::extensions::U2;
use crate::{graphics::switch_red_and_blue, platform::Buffer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A list with a known size that is able to be copied
pub struct CopyableList<T, const N: usize>
where
    T: Copy,
{
    /// T is held N times
    pub data: [T; N],
}
impl<T, const N: usize> Default for CopyableList<T, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> CopyableList<T, N>
where
    T: Copy + Default,
{
    /// Create a new instance with size N
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
    /// Create a new instance with already existing data, to get repeating data use [vec!] or `.repeat_value()`
    #[must_use]
    pub const fn from_array(data: [T; N]) -> Self {
        Self {
            data,
        }
    }
    /// Get the value at the index
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    /// # Set the value at the index
    ///
    /// # Errors
    /// Index out of bounce
    pub const fn set(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), &'static str> {
        if index < N {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    /// Get the known length of the list
    pub const fn len(&self) -> usize {
        N
    }
    /// This is stupid
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
    /// Receive the stored data as a reference
    pub const fn as_slice(&self) -> &[T] {
        &self.data
    }
}

/// 32²
pub type List1K<T> = CopyableList<T, 1024>;
/// 64²
pub type List4K<T> = CopyableList<T, 4096>;
/// 128²
pub type List16K<T> = CopyableList<T, 16384>;
/// 256²
pub type List64K<T> = CopyableList<T, 65536>;

/// Enum to hold different sized lists at runtime
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariableSizeList<T>
where
    T: Copy,
{
    /// 32² = 1,024
    Image32(List1K<T>),
    /// 64² = 4,096
    Image64(List4K<T>),
    /// 128² = 16,384
    Image128(List16K<T>),
    /// 256² = 65,536
    Image256(List64K<T>),
}
#[must_use]
/// Convert a U2 into a copyable size list with the sizes of 32, 64, 128, and 256
pub fn u2_to_size_list<T: Copy + std::default::Default>(
    size: U2,
) -> VariableSizeList<T> {
    match size {
        U2 {
            b0: false,
            b1: false,
        } => VariableSizeList::Image32(List1K::new()),
        U2 {
            b0: true,
            b1: false,
        } => VariableSizeList::Image64(List4K::new()),
        U2 {
            b0: false,
            b1: true,
        } => VariableSizeList::Image128(List16K::new()),
        U2 {
            b0: true,
            b1: true,
        } => VariableSizeList::Image256(List64K::new()),
    }
}

impl<T> VariableSizeList<T>
where
    T: Copy + Default,
{
    /// Create a new list with the size of 32**2
    #[must_use]
    pub fn new_32() -> Self {
        Self::Image32(List1K::new())
    }
    /// Create a new list with the size of 64**2
    #[must_use]
    pub fn new_64() -> Self {
        Self::Image64(List4K::new())
    }
    /// Create a new list with the size of 128**2
    #[must_use]
    pub fn new_128() -> Self {
        Self::Image128(List16K::new())
    }
    /// Create a new list with the size of 256**2
    #[must_use]
    pub fn new_256() -> Self {
        Self::Image256(List64K::new())
    }
    #[must_use]
    /// Get the known length of the list
    pub const fn len(&self) -> usize {
        match self {
            Self::Image32(list) => list.len(),
            Self::Image64(list) => list.len(),
            Self::Image128(list) => list.len(),
            Self::Image256(list) => list.len(),
        }
    }
    #[must_use]
    /// This is stupid
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Image32(list) => list.is_empty(),
            Self::Image64(list) => list.is_empty(),
            Self::Image128(list) => list.is_empty(),
            Self::Image256(list) => list.is_empty(),
        }
    }
    #[must_use]
    /// Get the value at the index
    pub fn get(&self, index: usize) -> Option<&T> {
        match self {
            Self::Image32(list) => list.get(index),
            Self::Image64(list) => list.get(index),
            Self::Image128(list) => list.get(index),
            Self::Image256(list) => list.get(index),
        }
    }

    /// # Set the value at the index
    ///
    /// # Errors
    /// Index out of bounce
    pub const fn set(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), &'static str> {
        match self {
            Self::Image32(list) => list.set(index, value),
            Self::Image64(list) => list.set(index, value),
            Self::Image128(list) => list.set(index, value),
            Self::Image256(list) => list.set(index, value),
        }
    }
}
fn copy_list_to_buffer<const N: usize>(
    list: &CopyableList<u32, N>,
    width: usize,
    height: usize,
) -> Buffer {
    let mut buffer = Buffer::new_empty(width, height);
    buffer.data = list.data.to_vec();
    buffer.update_pointer();
    buffer
}

/// Fills all values of a buffer into a copyable list
///
/// # Errors
/// When the buffer is too big it will fill the copy list until it's full and then error
pub fn fill(
    from: &Buffer,
    to: Box<VariableSizeList<u32>>,
) -> Result<Box<VariableSizeList<u32>>, &'static str> {
    let mut list = to;
    for idx in 0..from.total_size {
        list.set(idx, from.data[idx])?;
    }
    Ok(list)
}

// /// Converts a buffer into a copyable list
// ///
// /// # Errors
// /// When the buffer is not in a size compatible with the used enum
// pub fn buffer_to_copy_list(
//     buffer: &Buffer,
// ) -> Result<Box<VariableSizeList<u32>>, &'static str> {
//     if buffer.width != buffer.height {
//         return Err("Width and height differ");
//     }
//     match buffer.width {
//         32 => Ok(fill(buffer, Box::new(VariableSizeList::new_32()))?),
//         64 => Ok(fill(buffer, Box::new(VariableSizeList::new_64()))?),
//         128 => Ok(fill(buffer, Box::new(VariableSizeList::new_128()))?),
//         256 => Ok(fill(buffer, Box::new(VariableSizeList::new_256()))?),
//         _ => Err("Invalid size"),
//     }
// }

impl<const N: usize> From<CopyableList<u32, N>> for Buffer {
    fn from(list: CopyableList<u32, N>) -> Self {
        let size = list.len().isqrt();
        copy_list_to_buffer(&list, size, size)
    }
}

// impl TryFrom<Buffer> for VariableSizeList<u32> {
//     fn try_from(value: Buffer) -> Result<Self, Self::Error> {
//         // let some = buffer_to_copy_list(&value);

//         match some {
//             Ok(v) => Ok(*v),
//             Err(v) => Err(v),
//         }
//     }

//     type Error = &'static str;
// }

// impl From<VariableSizeList<u32>> for Buffer {
//     fn from(list: VariableSizeList<u32>) -> Self {
//         match list {
//             VariableSizeList::Image128(value) => {
//                 copy_list_to_buffer(&value, 128, 128)
//             }
//             VariableSizeList::Image64(value) => {
//                 copy_list_to_buffer(&value, 64, 64)
//             }
//             VariableSizeList::Image32(value) => {
//                 copy_list_to_buffer(&value, 32, 32)
//             }
//             VariableSizeList::Image256(value) => {
//                 copy_list_to_buffer(&value, 256, 256)
//             }
//         }
//     }
// }

impl<const N: usize> CopyableList<u32, N> {
    /// Swaps red and blue channels in all ARGB values
    /// ARGB format: 0xAARRGGBB -> 0xAABBGGRR
    #[must_use]
    pub const fn swap_red_blue(&self) -> Self {
        let mut result = *self;
        let mut i = 0;
        while i < N {
            result.data[i] = switch_red_and_blue(result.data[i]);
            i += 1;
        }
        result
    }
}
