fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) { report(&format!("Cannot draw: {message}")); }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {