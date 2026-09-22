// --8<-- [start:step-10a]
pub mod cloud_query;
// --8<-- [end:step-10a]
pub mod feedback;
pub mod input;
pub mod knobs;
pub mod scene;
pub mod selection;
pub mod stream;
pub mod touch;
pub mod walk;

// --8<-- [start:step-10b]
#[cfg(target_arch = "wasm32")]
pub mod fetch;
// --8<-- [end:step-10b]
#[cfg(target_arch = "wasm32")]
pub mod loader;
#[cfg(target_arch = "wasm32")]
pub mod route;

#[cfg(any(target_arch = "wasm32", test))]
pub mod inspection;
