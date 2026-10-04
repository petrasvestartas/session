fn detail(message: &str) -> String {
    let text = message.trim();
    // Iterating characters bounds diagnostic text without cutting a UTF-8 character; empty browser messages still receive a useful fallback.
    if text.is_empty() { "Unspecified error".into() }
    else { text.chars().take(2048).collect() }
}

pub fn error(message: &str, filename: &str, line: u32, column: u32) -> String {
    let source: String = filename.split(['?', '#']).next().unwrap_or("").chars().take(1024).collect();
    let message = format!("Browser error: {}", detail(message));
    if source.is_empty() { message } else { format!("{message}\nAt {source}:{line}:{column}") }
}

pub fn rejection(reason: &str) -> String { format!("Unhandled rejection: {}", detail(reason)) }
