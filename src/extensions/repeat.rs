#[const_trait]
/// A trait that allows for data to repeat
pub trait RepeatData
where
    Self: Sized + Clone,
{
    /// Repeat the given data X times and return a Vec containing the repeated data
    fn repeat_value(self, times: usize) -> Vec<Self>;
}
impl<T: Sized + Clone> RepeatData for T {
    fn repeat_value(self, amount: usize) -> Vec<T> {
        // let mut buffer = Vec::with_capacity(total_size);
        // unsafe {
        //     let ptr = buffer.as_mut_ptr();
        //     std::ptr::write(ptr, color);
        //     let mut filled = 1;
        //     while filled < total_size {
        //         let copy_len = std::cmp::min(filled, total_size - filled);
        //         std::ptr::copy_nonoverlapping(ptr, ptr.add(filled), copy_len);
        //         filled += copy_len;
        //     }
        //     buffer.set_len(total_size);
        // }
        std::vec::from_elem(self, amount)
    }
}

#[const_trait]
/// A trait that allows for data to repeat inside another container
pub trait RepeatDataInContainer {
    /// The output type of `Self<T>` should be `Self<Vec<T>>`
    type Output;
    /// Repeat the given data X times and return a Vec containing the repeated data
    fn repeat_value_inside(self, times: usize) -> Self::Output;
}
impl<T: RepeatData> RepeatDataInContainer for Option<T> {
    type Output = std::option::Option<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        self.map(|val| val.repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for Box<T> {
    type Output = Box<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        Box::new((*self).repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::cell::Cell<T> {
    type Output = std::cell::Cell<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::cell::Cell::new((self.into_inner()).repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::rc::Rc<T> {
    type Output = std::rc::Rc<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::rc::Rc::new((*self).clone().repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::sync::Arc<T> {
    type Output = std::sync::Arc<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::sync::Arc::new((*self).clone().repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::cell::RefCell<T> {
    type Output = std::cell::RefCell<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::cell::RefCell::new(self.into_inner().repeat_value(times))
    }
}

impl<T: RepeatData + Clone, E> RepeatDataInContainer for Result<T, E> {
    type Output = Result<std::vec::Vec<T>, E>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        match self {
            Ok(v) => Ok(v.repeat_value(times)),
            Err(e) => Err(e),
        }
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::task::Poll<T> {
    type Output = std::task::Poll<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        match self {
            Self::Ready(v) => std::task::Poll::Ready(v.repeat_value(times)),
            Self::Pending => std::task::Poll::Pending,
        }
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::num::Wrapping<T> {
    type Output = std::num::Wrapping<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::num::Wrapping(self.0.repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::cmp::Reverse<T> {
    type Output = std::cmp::Reverse<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::cmp::Reverse(self.0.repeat_value(times))
    }
}

impl<T: RepeatData + Clone> RepeatDataInContainer for std::sync::Mutex<T> {
    type Output = std::sync::Mutex<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        let inner = match Self::into_inner(self) {
            Ok(v) => v,
            Err(poisoned) => poisoned.into_inner(),
        };
        std::sync::Mutex::new(inner.repeat_value(times))
    }
}

impl<T: RepeatData + Clone> RepeatDataInContainer for std::sync::RwLock<T> {
    type Output = std::sync::RwLock<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        let inner = match Self::into_inner(self) {
            Ok(v) => v,
            Err(poisoned) => poisoned.into_inner(),
        };
        std::sync::RwLock::new(inner.repeat_value(times))
    }
}
