// #[cfg(feature = "imagery")]
/// This struct hold the raw data of a file to be converted/used somewhere else
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileData {
    /// Raw data
    pub raw_data: Vec<u8>,
    /// Path to get raw data
    pub expected_data_type: ExpectedDataType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedDataType {
    Bytes,
    Text,
    #[cfg(feature = "font_support")]
    Font,
    #[cfg(feature = "imagery")]
    Image,
    Unknown,
}

impl FileData {
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_bytes(
        data: Vec<u8>,
        expected_data_type: ExpectedDataType,
    ) -> Self {
        Self {
            raw_data: data,
            expected_data_type,
        }
    }
    /// Convert the raw bytes to a String (if valid UTF-8)
    ///
    /// # Errors
    /// If the data is not in utf8 format
    pub fn as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.raw_data.clone())
    }
    // #[must_use]
    // /// Convert the raw bytes to a Number (assumes data is in a binary format like little-endian)
    // pub fn as_number(&self) -> Result<i64, &'static str> {
    //     if self.raw_data.len() < 8 {
    //         return Err(
    //             "Not enough data",
    //         );
    //     }
    //     let number = i64::from_le_bytes(self.raw_data[0..8].try_into()?);
    //     Ok(number)
    // }
    #[must_use]
    #[cfg(feature = "font_support")]
    /// Convert the raw bytes to a [fontdue::Font] if possible
    pub fn as_font(&self) -> Result<fontdue::Font, Box<dyn std::error::Error>> {
        let font = fontdue::Font::from_bytes(
            self.raw_data.clone(),
            fontdue::FontSettings::default(),
        )?;
        return Ok(font);
    }
    #[must_use]
    /// Convert the raw bytes into an image::DynamicImage instance
    #[cfg(feature = "imagery")]
    pub fn as_image(
        &self,
    ) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
        // Decode the raw bytes as an image
        let img = image::load_from_memory(&self.raw_data)?;

        Ok(img)
    }
    #[must_use]
    /// Get raw bytes
    pub const fn as_bytes(&self) -> &Vec<u8> {
        &self.raw_data
    }
}
