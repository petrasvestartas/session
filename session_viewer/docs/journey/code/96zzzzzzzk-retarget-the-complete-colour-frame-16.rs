mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="sample_composition_tests.rs"]
mod composition_tests;