pub mod feedback; // register:feedback
pub mod gesture; // register:gesture
pub mod input; // register:input
#[cfg(any(target_arch = "wasm32", test))] // register:inspection
pub mod inspection; // register:inspection
pub mod keys; // register:keys
pub mod knobs; // register:knobs
#[cfg(target_arch = "wasm32")] // register:loader
pub mod loader; // register:loader
pub mod scene; // register:scene
pub mod selection; // register:selection
pub mod touch; // register:touch
pub mod walk; // register:walk
