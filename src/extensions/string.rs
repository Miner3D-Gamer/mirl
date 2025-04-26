pub trait StringExtensions {
    fn is_number(&self) -> bool;
    fn rjust(&self, length: usize, fillchar: Option<char>) -> String;
    fn ljust(&self, length: usize, fillchar: Option<char>) -> String;
    fn center(&self, length: usize, fillchar: Option<char>) -> String;
    fn expandtabs(&self) -> String;
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
}
