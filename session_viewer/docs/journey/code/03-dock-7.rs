pub(crate) fn ranked(names: &[&'static str], line: &str) -> Vec<&'static str> {
    let prefix = line.to_ascii_lowercase();
    let mut choices = names.iter().copied()
        .filter(|name| name.to_ascii_lowercase().starts_with(&prefix))
        .collect::<Vec<_>>();
    choices.sort_by_key(|name| {
        if name.eq_ignore_ascii_case(line) { 0 }
        else if name.get(line.len()..).is_some_and(|rest| rest.starts_with(' ')) { 1 }
        else { 2 }
    });
    choices
}

