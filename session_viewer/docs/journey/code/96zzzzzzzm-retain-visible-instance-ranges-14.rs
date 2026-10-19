#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_range_gpu_tests.rs"]
mod range_tests;

pub struct Renderer {