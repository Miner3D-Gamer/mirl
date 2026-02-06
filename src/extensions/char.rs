use crate::misc::EasyUnwrapUnchecked;

/// A trait from getting a char from
pub const trait CharGetPos {
    /// When you 100% know the position exists
    /// 
    /// The person using this function is responsible for its safety
    fn get_at(&mut self, position: usize) -> char;
}
impl CharGetPos for core::str::Chars<'_> {
    fn get_at(&mut self, position: usize) -> char {
        let _ = self.advance_by(position);
        self.next().easy_unwrap_unchecked()
    }
}
