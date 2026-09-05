//! The app layer: how the viewer is brought up (the loader). Above the engine, below the
//! shell in lib.rs. Never names a wgpu type.

#[cfg(target_arch = "wasm32")]
pub mod loader;
