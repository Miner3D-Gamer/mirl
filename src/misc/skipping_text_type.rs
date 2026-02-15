// TODO: Deduplicate letter pairs

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The basic type of a char
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum CharacterType {
    /// All letters (both upper and lower)
    Letter,
    /// All which are considered numeric
    Number,
    /// All that is this character: '_'
    Underscore,
    /// All you cannot reasonably discriminate
    Special,
    /// All you cannot see
    Whitespace,
}
/// Get a simple abstract type for a character
#[must_use]
pub fn get_char_type(ch: char) -> CharacterType {
    match ch {
        c if c.is_alphabetic() => CharacterType::Letter,
        c if c.is_numeric() => CharacterType::Number,
        '_' => CharacterType::Underscore,
        c if c.is_whitespace() => CharacterType::Whitespace,
        _ => CharacterType::Special,
    }
}
/// Skip towards the right
#[must_use]
pub fn skip_char_type(string: &str, current: usize) -> usize {
    let chars: Vec<(usize, char)> = string.char_indices().collect();

    // whitespace evil
    let mut pos = current;
    while pos < string.len() {
        if let Some((char_pos, ch)) =
            chars.iter().find(|(char_pos, _)| *char_pos >= pos)
        {
            if !ch.is_whitespace() {
                pos = *char_pos;
                break;
            }
            if let Some((next_pos, _)) =
                chars.iter().find(|(char_pos, _)| *char_pos > pos)
            {
                pos = *next_pos;
            } else {
                return string.len();
            }
        } else {
            return string.len();
        }
    }

    if let Some((_, ch)) = chars.iter().find(|(char_pos, _)| *char_pos == pos) {
        let start_type = get_char_type(*ch);

        match start_type {
            CharacterType::Letter
            | CharacterType::Number
            | CharacterType::Underscore => skip_identifier(string, pos),
            CharacterType::Whitespace => skip_whitespace(string, pos),
            CharacterType::Special => skip_special_chars(string, pos),
        }
    } else {
        string.len()
    }
}

fn skip_identifier(string: &str, current: usize) -> usize {
    let chars = string.char_indices().skip_while(|(i, _)| *i < current);

    for (idx, ch) in chars {
        let char_type = get_char_type(ch);
        match char_type {
            CharacterType::Letter
            | CharacterType::Number
            | CharacterType::Underscore => {}
            _ => return idx,
        }
    }
    string.len()
}

fn skip_whitespace(string: &str, current: usize) -> usize {
    let chars = string.char_indices().skip_while(|(i, _)| *i < current);

    for (idx, ch) in chars {
        if !ch.is_whitespace() {
            return idx;
        }
    }
    string.len()
}

fn skip_special_chars(string: &str, current: usize) -> usize {
    let mut chars = string.char_indices().skip_while(|(i, _)| *i < current);
    let Some((_, first_char)) = chars.next() else {
        return string.len();
    };

    // pair of things
    let bracket_chars: std::collections::HashSet<char> =
        ['(', ')', '[', ']', '{', '}', '<', '>'].iter().copied().collect();
    let operator_chars: std::collections::HashSet<char> =
        ['+', '-', '*', '/', '=', '!', '&', '|', '^', '%', '~']
            .iter()
            .copied()
            .collect();
    let punctuation_chars: std::collections::HashSet<char> =
        ['.', ',', ';', ':'].iter().copied().collect();
    let quote_chars: std::collections::HashSet<char> =
        ['"', '\'', '`'].iter().copied().collect();

    let target_group = if bracket_chars.contains(&first_char) {
        Some(&bracket_chars)
    } else if operator_chars.contains(&first_char) {
        Some(&operator_chars)
    } else if punctuation_chars.contains(&first_char) {
        Some(&punctuation_chars)
    } else if quote_chars.contains(&first_char) {
        Some(&quote_chars)
    } else {
        None
    };

    match target_group {
        Some(group) => {
            for (idx, ch) in chars {
                if get_char_type(ch) != CharacterType::Special
                    || !group.contains(&ch)
                {
                    return idx;
                }
            }
            string.len()
        }
        None => chars.next().map_or(string.len(), |(idx, _)| idx),
    }
}

/// Skip towards the left
#[must_use]
pub fn skip_char_type_backward(string: &str, current: usize) -> usize {
    if current == 0 {
        return 0;
    }

    let chars: Vec<(usize, char)> = string.char_indices().collect();

    // this whitespace evil too
    let mut pos = current;
    while pos > 0 {
        if let Some((char_pos, ch)) =
            chars.iter().rev().find(|(char_pos, _)| *char_pos < pos)
        {
            if !ch.is_whitespace() {
                pos = *char_pos;
                break;
            }
            pos = *char_pos;
        } else {
            return 0;
        }
    }

    if let Some((_, ch)) = chars.iter().find(|(char_pos, _)| *char_pos == pos) {
        let target_type = get_char_type(*ch);

        match target_type {
            CharacterType::Letter
            | CharacterType::Number
            | CharacterType::Underscore => {
                for (char_pos, ch) in chars.iter().rev() {
                    if *char_pos >= pos {
                        continue;
                    }
                    let char_type = get_char_type(*ch);
                    if !matches!(
                        char_type,
                        CharacterType::Letter
                            | CharacterType::Number
                            | CharacterType::Underscore
                    ) {
                        // Found the character before the identifier starts
                        return chars
                            .iter()
                            .find(|(cp, _)| *cp > *char_pos)
                            .map_or(0, |(cp, _)| *cp);
                    }
                }
                0
            }
            CharacterType::Whitespace => {
                // Find start of whitespace
                for (char_pos, ch) in chars.iter().rev() {
                    if *char_pos >= pos {
                        continue;
                    }
                    if !ch.is_whitespace() {
                        return chars
                            .iter()
                            .find(|(cp, _)| *cp > *char_pos)
                            .map_or(0, |(cp, _)| *cp);
                    }
                }
                0
            }
            CharacterType::Special => skip_special_backward(&chars, pos),
        }
    } else {
        0
    }
}

fn skip_special_backward(chars: &[(usize, char)], pos: usize) -> usize {
    let target_char =
        chars.iter().find(|(char_pos, _)| *char_pos == pos).map(|(_, ch)| *ch);

    if let Some(first_char) = target_char {
        let bracket_chars: std::collections::HashSet<char> =
            ['(', ')', '[', ']', '{', '}', '<', '>'].iter().copied().collect();
        let operator_chars: std::collections::HashSet<char> =
            ['+', '-', '*', '/', '=', '!', '&', '|', '^', '%', '~']
                .iter()
                .copied()
                .collect();
        let punctuation_chars: std::collections::HashSet<char> =
            ['.', ',', ';', ':'].iter().copied().collect();
        let quote_chars: std::collections::HashSet<char> =
            ['"', '\'', '`'].iter().copied().collect();

        let target_group = if bracket_chars.contains(&first_char) {
            Some(&bracket_chars)
        } else if operator_chars.contains(&first_char) {
            Some(&operator_chars)
        } else if punctuation_chars.contains(&first_char) {
            Some(&punctuation_chars)
        } else if quote_chars.contains(&first_char) {
            Some(&quote_chars)
        } else {
            None
        };

        match target_group {
            Some(group) => {
                for (char_pos, ch) in chars.iter().rev() {
                    if *char_pos >= pos {
                        continue;
                    }
                    if get_char_type(*ch) != CharacterType::Special
                        || !group.contains(ch)
                    {
                        return chars
                            .iter()
                            .find(|(cp, _)| *cp > *char_pos)
                            .map_or(0, |(cp, _)| *cp);
                    }
                }
                0
            }
            None => pos,
        }
    } else {
        pos
    }
}
