pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;


#[cfg(target_arch = "wasm32")]
#[path = "../examples/course/entry.rs"]
mod entry;
#[cfg(target_arch = "wasm32")]
#[path = "../examples/course/registration.rs"]
mod registration;
