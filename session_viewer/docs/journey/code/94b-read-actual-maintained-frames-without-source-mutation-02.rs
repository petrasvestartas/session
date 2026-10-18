[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }

[[example]]
name = "reference"
path = "src/reference_example.rs"
