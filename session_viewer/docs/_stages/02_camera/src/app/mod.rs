//! The app layer: how the viewer is brought up (the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.

pub mod input;
pub mod touch;

#[cfg(target_arch = "wasm32")]
pub mod loader;
#[cfg(target_arch = "wasm32")]
pub mod route;
