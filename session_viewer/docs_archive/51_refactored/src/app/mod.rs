//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (fetch, decode, stream, the loader) and is driven (input). Above the engine, below the
//! shell in lib.rs.

pub mod decode;
pub mod fetch;
pub mod input;
pub mod knobs;
#[cfg(target_arch = "wasm32")]
pub mod loader;
pub mod manifest;
pub mod scene;
pub mod stream;
pub mod walk;
