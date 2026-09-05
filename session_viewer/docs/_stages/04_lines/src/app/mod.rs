//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (route, fetch, decode, the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.

pub mod input;
pub mod knobs;
pub mod manifest;
pub mod scene;
pub mod touch;
pub mod walk;

#[cfg(target_arch = "wasm32")]
pub mod decode;
#[cfg(target_arch = "wasm32")]
pub mod fetch;
#[cfg(target_arch = "wasm32")]
pub mod loader;
#[cfg(target_arch = "wasm32")]
pub mod route;
