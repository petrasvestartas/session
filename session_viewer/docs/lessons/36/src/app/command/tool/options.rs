/// Option labels, excluding the Enter and Escape actions.
pub(super) fn choices(chips: &'static [(&str, &str)]) -> impl Iterator<Item = &'static str> {
    chips.iter().filter_map(|&(label, line)| {
        if line.is_empty() || line == "Escape" {
            None
        } else {
            Some(label)
        }
    })
}

/// Match an option and return its index and consumed word count.
pub(super) fn find(chips: &'static [(&str, &str)], words: &[&str]) -> Option<(usize, usize)> {
    for (index, label) in choices(chips).enumerate() {
        let target = compact(label);
        let mut typed = String::new();

        for (count, word) in words.iter().enumerate() {
            typed.push_str(&word.to_ascii_lowercase());

            if typed == target {
                return Some((index, count + 1));
            }

            if !target.starts_with(&typed) {
                break;
            }
        }
    }

    None
}

/// Lowercase without spaces.
pub(super) fn compact(text: &str) -> String {
    text.split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase()
}
