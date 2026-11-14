pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;


#[cfg(target_arch = "wasm32")]
mod command_dock;
#[cfg(target_arch = "wasm32")]
mod panel;

#[cfg(target_arch = "wasm32")]
#[path = "../examples/course/entry.rs"]
mod entry;
#[cfg(target_arch = "wasm32")]
#[path = "../examples/course/registration.rs"]
mod registration;

pub mod app;
#[path = "../examples/course/fonts.rs"] mod lesson_fonts;
pub use lesson_fonts::fonts;
#[cfg(target_arch = "wasm32")]
#[path = "../examples/course/mathematics.rs"] mod mathematics;
#[cfg(test)] mod math_tests;
