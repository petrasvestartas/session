// --8<-- [start:course-registration]
#[path = "canvas.rs"]
mod canvas;
#[path = "frame.rs"]
mod frame;
#[path = "painting.rs"]
mod painting;
#[path = "mathematics.rs"]
mod mathematics;
#[path = "dock.rs"]
mod dock;
#[path = "camera.rs"]
mod camera;
#[path = "rows.rs"]
mod rows;

type Start = fn() -> Result<(), wasm_bindgen::JsValue>;
pub(super) const STARTS: &[(&str, Start)] = &[
    ("frame", frame::start),
    ("painting", painting::start),
    ("mathematics", mathematics::start),
    ("dock", dock::start),
    ("camera", camera::start),
    ("rows", rows::start),
    ("viewer", crate::run_web),
];
// --8<-- [end:course-registration]
