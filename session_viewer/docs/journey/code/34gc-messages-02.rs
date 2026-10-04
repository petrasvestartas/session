use crate::error_message::{error, rejection};

#[test]
fn long_unicode_errors_fit_the_report_budget() {
    let message = error(&"é😀".repeat(10000), &"source".repeat(1000), u32::MAX, u32::MAX);
    assert!(message.chars().count() < 4096);
    assert!(message.starts_with("Browser error: é😀"));
    assert!(message.ends_with(":4294967295:4294967295"));
    assert!(rejection(&"😀".repeat(10000)).chars().count() < 4096);
}

#[test]
fn source_locations_keep_positions_without_queries_or_fragments() {
    assert_eq!(error("bad input", "https://example.test/app.js?private=value#part", 42, 7),
        "Browser error: bad input\nAt https://example.test/app.js:42:7");
    assert_eq!(error("bad input", "app.js#part?private=value", 8, 2), "Browser error: bad input\nAt app.js:8:2");
}

#[test]
fn missing_details_have_readable_labels() {
    assert_eq!(error("  ", "", 0, 0), "Browser error: Unspecified error");
    assert_eq!(rejection(""), "Unhandled rejection: Unspecified error");
    assert_eq!(rejection(" timeout "), "Unhandled rejection: timeout");
}
