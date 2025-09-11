/// More functionality for strings
pub trait StringExtensions {
    /// Checks if the string only contains numbers
    fn is_number(&self) -> bool;
    /// Pads the string on the right
    fn rjust(&self, length: usize, fillchar: Option<char>) -> String;
    /// Pads the string on the left
    fn ljust(&self, length: usize, fillchar: Option<char>) -> String;
    /// Pads the string to be in the middle
    fn center(&self, length: usize, fillchar: Option<char>) -> String;
    /// Converts '\t' into '    '
    fn expand_tabs(&self) -> String;
    // fn extract_file_name(&self) -> String;
    // fn extract_file_extension(&self) -> String;
    // fn extract_file_name_without_extension(&self) -> String;
    /// Replaces the first occurrence of X with Y
    fn replace_first_occurrence(
        &self,
        target: &str,
        replacement: &str,
    ) -> String;
    // /// Replaces the first occurrence of X with Y but error if there is not occurrence of X
    // fn replace_first_occurrence_error(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    // ) -> String;
    // /// Replaces the all occurrence of X with Y
    // fn replace_occurrences(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    //     amount: u32,
    // ) -> String;
    // /// Replaces the all occurrence of X with Y but error if there is not occurrence of X
    // fn replace_occurrences_error(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    //     amount: u32,
    // ) -> String;
    /// Reverse the string character order and return the result
    fn reversed(&self) -> String;
}

impl StringExtensions for str {
    fn is_number(&self) -> bool {
        self.chars().all(char::is_numeric)
    }

    fn rjust(&self, length: usize, fillchar: Option<char>) -> String {
        let pad = length.saturating_sub(self.len());
        let fill = fillchar.unwrap_or(' ');
        format!("{}{}", fill.to_string().repeat(pad), self)
    }

    fn ljust(&self, length: usize, fillchar: Option<char>) -> String {
        let pad = length.saturating_sub(self.len());
        let fill = fillchar.unwrap_or(' ');
        format!("{}{}", self, fill.to_string().repeat(pad))
    }

    fn center(&self, length: usize, fillchar: Option<char>) -> String {
        let pad = length.saturating_sub(self.len());
        let left_pad = pad / 2;
        let right_pad = pad - left_pad;
        let fill = fillchar.unwrap_or(' ');
        format!(
            "{}{}{}",
            fill.to_string().repeat(left_pad),
            self,
            fill.to_string().repeat(right_pad)
        )
    }
    #[allow(clippy::unwrap_used)]
    fn expand_tabs(&self) -> String {
        self.replace("\t".chars().next().unwrap(), "    ")
    }
    // fn extract_file_name(&self) -> String {
    //     let parts: Vec<&str> = self.split('/').collect();
    //     parts[parts.len() - 1].to_string()
    // }

    // fn extract_file_extension(&self) -> String {
    //     let parts: Vec<&str> = self.split('.').collect();
    //     parts[parts.len() - 1].to_string()
    // }

    // fn extract_file_name_without_extension(&self) -> String {
    //     let parts: Vec<&str> = self.split('.').collect();
    //     let parts: Vec<&str> = parts[0].split('/').collect();
    //     parts[parts.len() - 1].to_string()
    // }

    fn replace_first_occurrence(
        &self,
        target: &str,
        replacement: &str,
    ) -> String {
        let mut result = self.to_string();
        if let Some(pos) = result.find(target) {
            result.replace_range(pos..pos + target.len(), replacement);
        }
        result
    }

    // fn replace_first_occurrence_error(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    // ) -> String {
    //     let mut result = self.to_string();
    //     if let Some(pos) = result.find(target) {
    //         result.replace_range(pos..pos + target.len(), replacement);
    //     } else {
    //         panic!("Could not find {} in {}", target, self);
    //     }
    //     result
    // }

    // fn replace_occurrences(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    //     amount: u32,
    // ) -> String {
    //     let mut result = self.to_string();
    //     for _ in 0..amount {
    //         result = result.replace_first_occurrence(target, replacement);
    //     }
    //     result
    // }

    // fn replace_occurrences_error(
    //     &self,
    //     target: &str,
    //     replacement: &str,
    //     amount: u32,
    // ) -> String {
    //     let mut result = self.to_string();
    //     for _ in 0..amount {
    //         result = result.replace_first_occurrence_error(target, replacement);
    //     }
    //     result
    // }
    fn reversed(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
/// List operations for strings
pub trait RemoveChar {
    /// Remove the character at the position
    fn remove_char_at(&mut self, pos: usize);
    /// Remove the character at the position and returns it
    fn pop_char_at(&mut self, pos: usize) -> Option<char>;
    /// Remove the character at the position
    fn remove_chars_at(&mut self, pos: usize, amount: usize);
    /// Remove the character at the position and returns it
    fn pop_chars_at(&mut self, pos: usize, amount: usize) -> Option<Vec<char>>;
}

impl RemoveChar for String {
    fn remove_char_at(&mut self, pos: usize) {
        if pos < self.chars().count() {
            let mut chars: Vec<char> = self.chars().collect();
            chars.remove(pos);
            *self = chars.into_iter().collect();
        }
    }

    fn pop_char_at(&mut self, pos: usize) -> Option<char> {
        if pos < self.chars().count() {
            let mut chars: Vec<char> = self.chars().collect();
            let removed = chars.remove(pos);
            *self = chars.into_iter().collect();
            Some(removed)
        } else {
            None
        }
    }
    fn remove_chars_at(&mut self, pos: usize, amount: usize) {
        let len = self.chars().count();
        if pos < len {
            let mut chars: Vec<char> = self.chars().collect();
            let end = (pos + amount).min(len);
            chars.drain(pos..end);
            *self = chars.into_iter().collect();
        }
    }

    fn pop_chars_at(&mut self, pos: usize, amount: usize) -> Option<Vec<char>> {
        let mut rt = Vec::new();
        if pos < self.chars().count() {
            for _ in 0..amount {
                let mut chars: Vec<char> = self.chars().collect();
                let removed = chars.remove(pos);
                *self = chars.into_iter().collect();
                rt.push(removed);
            }
            Some(rt)
        } else {
            None
        }
    }
}
