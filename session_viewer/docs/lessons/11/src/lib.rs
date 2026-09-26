#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Browser entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // panics print to the console
    console_error_panic_hook::set_once();
    engine::performance::mark("wasm entry"); // register:frame
    Ok(())
}

// --8<-- [start:01]
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    ($name:literal) => {
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

mod engine;
// --8<-- [end:01]

// --8<-- [start:02]
mod camera;
// --8<-- [end:02]

// --8<-- [start:06]
pub mod app;
// --8<-- [end:06]

// --8<-- [start:11]
#[cfg(target_arch = "wasm32")]
pub mod text_quality;
// --8<-- [end:11]
