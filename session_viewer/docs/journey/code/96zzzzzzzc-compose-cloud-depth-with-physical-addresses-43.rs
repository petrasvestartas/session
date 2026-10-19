mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="cloud_address_tests.rs"]
mod cloud_address_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="cloud_composite_tests.rs"]
mod cloud_composite_tests;