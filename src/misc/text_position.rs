#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A 2d position in a text context
pub struct TextPosition {
    /// Vertical
    pub line: usize,
    /// Horizontal
    pub column: usize,
}

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
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
    /// Move column by one
    pub const fn advance_char(&mut self) {
        self.column += 1;
    }
    /// Move column by one
    pub const fn advance_char_by(&mut self, by: usize) {
        self.column += by;
    }
    /// Move line by one and reset column
    pub const fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
    /// Move line by one and reset column
    pub const fn advance_line_by(&mut self, by: usize) {
        self.line += by;
        self.column = 0;
    }
    #[must_use]
    /// Get the line and offset from the given offset
    pub fn from_offset(offset: usize, text: &str) -> Self {
        let (line, col) = line_and_column_from_offset(offset, text);

        Self::new(line, col)
    }
}
#[cfg(feature = "std")]
impl Default for TextPosition {
    fn default() -> Self {
        Self {
            line: usize::MAX,
            column: usize::MAX,
        }
    }
}

// #[cfg_attr(feature = "std", derive(PartialOrd, Ord))]
// /// A text position range
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct PositionRange {
//     /// The start of the range
//     pub start: TextPosition,
//     /// The end of the range
//     pub end: TextPosition,
// }

// impl PositionRange {
//     /// Create a new range based on start and end points
//     #[must_use]
//     pub const fn new(start: TextPosition, end: TextPosition) -> Self {
//         Self {
//             start,
//             end,
//         }
//     }
// }
#[must_use]
/// Get the line and offset from the given offset
pub fn line_and_column_from_offset(
    offset: usize,
    text: &str,
) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in text.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}
