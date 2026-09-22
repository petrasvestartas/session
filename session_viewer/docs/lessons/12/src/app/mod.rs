// --8<-- [start:step-21]
pub mod feedback;
pub mod input;
pub mod knobs;
pub mod scene;
pub mod selection;
pub mod stream;
pub mod touch;
pub mod walk;

#[cfg(target_arch = "wasm32")]
pub mod loader;
#[cfg(target_arch = "wasm32")]
pub mod route;

#[cfg(any(target_arch = "wasm32", test))]
pub mod inspection;
// --8<-- [end:step-21]
