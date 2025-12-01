#[allow(clippy::cast_possible_truncation)] // Formats need to be consistent
/// Convert a list of strings into a list of u8
#[must_use]
pub fn strings_to_bytes(strings: &Vec<String>) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&(strings.len() as u32).to_le_bytes());

    for s in strings {
        let string_bytes = s.as_bytes();
        bytes.extend_from_slice(&(string_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(string_bytes);
    }

    bytes
}

#[allow(clippy::cast_possible_truncation)] // Formats need to be consistent
/// Convert a list of u8 into a list of Strings
#[must_use]
pub fn bytes_to_strings(bytes: &[u8]) -> Vec<String> {
    let mut cursor = 0;
    let mut strings = Vec::new();

    if bytes.len() < 4 {
        return strings;
    }
    let num_strings =
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    cursor += 4;

    for _ in 0..num_strings {
        if cursor + 4 > bytes.len() {
            break;
        }

        let len = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]) as usize;
        cursor += 4;

        if cursor + len > bytes.len() {
            break;
        }

        let string_bytes = &bytes[cursor..cursor + len];
        if let Ok(s) = String::from_utf8(string_bytes.to_vec()) {
            strings.push(s);
        }
        cursor += len;
    }

    strings
}

#[must_use]
/// Turn a hex string into a u32 value
pub fn hex_to_number(hex: &str) -> Option<u32> {
    u32::from_str_radix(hex, 16).ok()
}
#[must_use]
/// Turn a u32 into a hex string
pub fn number_to_hex(num: u32) -> String {
    format!("{num:x}")
}
