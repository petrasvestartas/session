pub fn check(device: &wgpu::Device, source: String, label: &'static str) {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label), source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        let info = module.get_compilation_info().await;
        let errors = info.messages.iter().filter(|m| m.message_type == wgpu::CompilationMessageType::Error).count();
        for message in info.messages {
            let text = format!("{label}: {:?} {:?}: {}", message.message_type, message.location, message.message);
            if message.message_type == wgpu::CompilationMessageType::Error {
                web_sys::console::error_1(&text.into());
            } else { web_sys::console::log_1(&text.into()); }
        }
        web_sys::console::log_1(&format!("{label}: {errors} shader errors").into());
    });
    #[cfg(not(target_arch = "wasm32"))]
    drop(module);
}
