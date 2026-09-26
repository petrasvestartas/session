// --8<-- [start:entry]
// `#[cfg(...)]` keeps the next item only when the condition holds: here, only in the browser build.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// The browser runs this once the module has loaded.
#[cfg(target_arch = "wasm32")]
// wasm-bindgen writes the JavaScript glue around the module; `start` makes that glue call this function.
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // a panic then prints its message to the browser console instead of a bare `unreachable`
    console_error_panic_hook::set_once();
    engine::performance::mark("wasm entry"); // a named point on the browser's performance timeline; register:frame
    Ok(())
}
// --8<-- [end:entry]

// --8<-- [start:01-first-frame]
// --8<-- [start:shader-macro]
// `macro_rules!` makes a macro, code that writes code; it must come before the `mod` lines that use it.
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    // `$name:literal` matches one string literal, such as "background.wgsl".
    ($name:literal) => {
        // `include_str!` pastes the file into the binary at compile time; OUT_DIR is the folder build.rs wrote.
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

// `mod engine;` makes src/engine/mod.rs part of this crate.
mod engine;
// --8<-- [end:shader-macro]
// --8<-- [end:01-first-frame]
