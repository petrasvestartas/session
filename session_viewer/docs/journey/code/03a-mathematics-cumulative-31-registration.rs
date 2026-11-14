use std::{future::Future, pin::Pin};
use wasm_bindgen::JsValue;
type Start = fn() -> Pin<Box<dyn Future<Output = Result<(), JsValue>>>>;
pub const DEFAULT: &str = "math";
pub const REPORT: fn(&str) = crate::browser::report;
pub const MODES: &[(&str, Start)] = &[
    ("math", || Box::pin(crate::mathematics::run())),
    ("viewer", || Box::pin(crate::browser::run())),
];
