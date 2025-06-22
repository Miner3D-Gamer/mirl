use std::hash::Hasher;

pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    return result;
}

pub fn extract_file_name(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    return parts[parts.len() - 1].to_string();
}

pub fn extract_file_extension(path: &str) -> String {
    let parts: Vec<&str> = path.split('.').collect();
    return parts[parts.len() - 1].to_string();
}

pub fn extract_file_name_without_extension(path: &str) -> String {
    let parts: Vec<&str> = path.split('.').collect();
    let parts: Vec<&str> = parts[0].split('/').collect();
    return parts[parts.len() - 1].to_string();
}

pub fn replace_first_occurrence(
    text: &str,
    target: &str,
    replacement: &str,
) -> String {
    let mut result = text.to_string();

    if let Some(pos) = result.find(target) {
        result.replace_range(pos..pos + target.len(), replacement);
    }

    result
}
pub fn replace_first_occurrence_error(
    text: &str,
    target: &str,
    replacement: &str,
) -> String {
    let mut result = text.to_string();

    if let Some(pos) = result.find(target) {
        result.replace_range(pos..pos + target.len(), replacement);
    } else {
        panic!("Could not find {} in {}", target, text);
    }

    result
}

pub fn replace_occurences(
    text: &str,
    target: &str,
    replacement: &str,
    amount: u32,
) -> String {
    let mut result = text.to_string();
    for _ in 0..amount {
        result = replace_first_occurrence(&result, target, replacement);
    }
    result
}

pub fn replace_occurences_error(
    text: &str,
    target: &str,
    replacement: &str,
    amount: u32,
) -> String {
    let mut result = text.to_string();
    for _ in 0..amount {
        result = replace_first_occurrence_error(&result, target, replacement);
    }
    result
}

pub fn hash_value<T: std::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}
