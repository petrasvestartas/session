pub mod background;
pub mod camera;
pub mod bounds;
pub mod mesh;
pub mod scene;
pub mod picking;
pub mod history;
pub mod editor;
pub mod viewport;
pub mod gesture;
pub mod shortcuts;
#[cfg(test)]
mod shortcuts_tests;

pub mod navigation;

pub mod document;
pub mod specimen;
#[cfg(test)]
mod document_tests;
#[cfg(test)]
mod fit_tests;
#[cfg(test)]
mod selected_fit_tests;
#[cfg(test)]
mod projection_tests;
#[cfg(test)]
mod navigation_tests;
#[cfg(test)]
mod gesture_tests;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
mod file_input;


#[cfg(any(test, target_arch = "wasm32"))]
mod command_dock;

#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
mod tool_buttons_tests;
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;

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
