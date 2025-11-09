#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A 2d position in a text context
pub struct TextPosition {
    /// Vertical
    pub line: usize,
    /// Horizontal
    pub column: usize,
}

impl Ord for TextPosition {
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self < other {
            self
        } else {
            other
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self > other {
            self
        } else {
            other
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        self.min(min).max(max)
    }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.line > other.line {
            std::cmp::Ordering::Greater
        } else if self.line < other.line {
            std::cmp::Ordering::Less
        } else if self.column > other.column {
            std::cmp::Ordering::Greater
        } else if self.column < other.column {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for TextPosition {
    fn ge(&self, other: &Self) -> bool {
        self.gt(other) || self == other
    }
    fn gt(&self, other: &Self) -> bool {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Equal => self.column > other.column,
            std::cmp::Ordering::Less => false,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => self.column < other.column,
            std::cmp::Ordering::Less => true,
        }
    }
    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self == other
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TextPosition {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}
impl Default for TextPosition {
    fn default() -> Self {
        Self {
            line: usize::MAX,
            column: usize::MAX,
        }
    }
}
