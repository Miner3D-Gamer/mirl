use std::any::Any;

/// The same as [`std::any::Any`] but cloneable
pub const trait CloneAny: Any + core::fmt::Debug {
    /// Clone the content of the box
    fn clone_box(&self) -> Box<dyn CloneAny>;
}

impl<T> CloneAny for T
where
    T: Any + Clone + 'static + core::fmt::Debug,
{
    fn clone_box(&self) -> Box<dyn CloneAny> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
impl dyn CloneAny {
    #[inline]
    /// Downcast this wrapper as [`std::any::Any`]
    pub fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    /// Downcast this wrapper as a mutable [`std::any::Any`]
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
