use crate::graphics::{
    self, get_alpha_of_u32_in_u8, get_blue_of_u32_in_u8, get_green_of_u32_in_u8, get_red_of_u32_in_u8,
};

// #[cfg(feature = "imagery")]
/// This struct hold the raw data of a file to be converted/used somewhere else
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileData {
    /// Raw data
    pub raw_data: Vec<u8>,
    /// Path to get raw data
    pub expected_data_type: DataType,
}
impl FileData {
    /// Creates a format that is just a little more pleasant to the eye
    #[must_use]
    pub fn to_printable(&self) -> String {
        match self.expected_data_type {
            DataType::Text => format!("Text: {:?}", self.as_string()),
            #[cfg(feature = "font_support")]
            DataType::Font => self.as_font().map_or_else(
                |_| "Not a font.".into(),
                |font| format!("Font: {font:?}"),
            ),
            #[cfg(feature = "imagery")]
            DataType::Image => format!("Bytes: {:?}", self.as_image()),
            DataType::Audio => format!("Audio: {:?}", "<Unsupported>"),
            #[cfg(not(feature = "do_not_compile_misc"))]
            DataType::ListOfText => {
                format!("List of text: {:#?}", self.as_list_of_strings())
            }
            DataType::Color => self.as_color().map_or_else(
                || "Not a color.".into(),
                |color| {
                    format!(
                        "Color: {:?} | r{} g{} b{} a{}",
                        color,
                        get_red_of_u32_in_u8(color),
                        get_green_of_u32_in_u8(color),
                        get_blue_of_u32_in_u8(color),
                        get_alpha_of_u32_in_u8(color)
                    )
                },
            ),
            _ => format!("Bytes: {:?}", self.as_bytes()),
        }
    }
}
/// What type the data is expected to be
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// Raw bytes/custom
    Bytes,
    /// Text/Strings
    Text,
    /// Font - Requires the `font_support` feature
    #[cfg(feature = "font_support")]
    Font,
    /// Image/Buffer - Requires the `imagery` feature
    #[cfg(feature = "imagery")]
    Image,
    /// Not supported
    Audio,
    /// As a list of strings/file paths
    ListOfText,
    /// As a color
    Color,
}
impl FileData {
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_bytes(
        data: Vec<u8>,
        expected_data_type: DataType,
    ) -> Self {
        Self {
            raw_data: data,
            expected_data_type,
        }
    }
    #[must_use]
    /// Constructor to load data from raw bytes
    pub const fn from_string(data: String) -> Self {
        Self::from_bytes(data.into_bytes(), DataType::Text)
    }
    #[cfg(not(feature = "do_not_compile_misc"))]
    #[must_use]
    /// Constructor to load data from a Vec<String>
    pub fn from_list_of_strings(value: &Vec<String>) -> Self {
        Self::from_bytes(
            crate::misc::strings_to_bytes(value),
            DataType::ListOfText,
        )
    }
}

impl FileData {
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
    #[cfg(feature = "font_support")]
    /// Convert the raw bytes to a [`fontdue::Font`] if possible
    ///
    /// # Errors
    /// When not a font it will error
    pub fn as_font(&self) -> Result<fontdue::Font, Box<dyn std::error::Error>> {
        let font = fontdue::Font::from_bytes(
            self.raw_data.clone(),
            fontdue::FontSettings::default(),
        )?;
        Ok(font)
    }
    /// Convert the raw bytes into an `image::DynamicImage` instance
    ///
    /// # Errors
    /// When unable to load the image from memory
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
    #[cfg(not(feature = "do_not_compile_misc"))]
    /// Get the list of strings/file paths
    #[must_use]
    pub fn as_list_of_strings(&self) -> Option<Vec<String>> {
        let data = crate::misc::bytes_to_strings(&self.raw_data.clone());
        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    }
    /// Get the data as a color:
    /// One byte: Gray scale is assumed
    /// Three bytes: RGB assumed
    /// Four bytes: RGBA assumed
    #[must_use]
    pub fn as_color(&self) -> Option<u32> {
        match self.raw_data.len() {
            1 => Some(graphics::rgb_u8_to_u32(
                self.raw_data[0],
                self.raw_data[0],
                self.raw_data[0],
            )),
            3 => Some(graphics::rgb_u8_to_u32(
                self.raw_data[0],
                self.raw_data[1],
                self.raw_data[2],
            )),
            4 => Some(graphics::rgba_u8_to_u32(
                self.raw_data[0],
                self.raw_data[1],
                self.raw_data[2],
                self.raw_data[3],
            )),
            _ => None,
        }
    }
}
