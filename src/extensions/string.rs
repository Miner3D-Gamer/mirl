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
    fn expandtabs(&self) -> String;
    // fn extract_file_name(&self) -> String;
    // fn extract_file_extension(&self) -> String;
    // fn extract_file_name_without_extension(&self) -> String;
    /// Replacs the first occurence of X with Y
    fn replace_first_occurrence(
        &self,
        target: &str,
        replacement: &str,
    ) -> String;
    /// Replacs the first occurence of X with Y but error if there is not occurence of X
    fn replace_first_occurrence_error(
        &self,
        target: &str,
        replacement: &str,
    ) -> String;
    /// Replacs the all occurence of X with Y
    fn replace_occurences(
        &self,
        target: &str,
        replacement: &str,
        amount: u32,
    ) -> String;
    /// Replacs the all occurence of X with Y but error if there is not occurence of X
    fn replace_occurences_error(
        &self,
        target: &str,
        replacement: &str,
        amount: u32,
    ) -> String;
}

impl StringExtensions for str {
    fn is_number(&self) -> bool {
        self.chars().all(|c| c.is_ascii_digit())
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
    fn expandtabs(&self) -> String {
        self.replace("\t", "    ")
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

    fn replace_first_occurrence_error(
        &self,
        target: &str,
        replacement: &str,
    ) -> String {
        let mut result = self.to_string();
        if let Some(pos) = result.find(target) {
            result.replace_range(pos..pos + target.len(), replacement);
        } else {
            panic!("Could not find {} in {}", target, self);
        }
        result
    }

    fn replace_occurences(
        &self,
        target: &str,
        replacement: &str,
        amount: u32,
    ) -> String {
        let mut result = self.to_string();
        for _ in 0..amount {
            result = result.replace_first_occurrence(target, replacement);
        }
        result
    }

    fn replace_occurences_error(
        &self,
        target: &str,
        replacement: &str,
        amount: u32,
    ) -> String {
        let mut result = self.to_string();
        for _ in 0..amount {
            result = result.replace_first_occurrence_error(target, replacement);
        }
        result
    }
}
