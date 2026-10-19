pub fn source() -> String {
    let shared = include_str!("projected_triangle.wgsl")
        .split("// Pixels per tile side for this canvas.").next().unwrap();
    include_str!("finite_project.wgsl").replace("#include \"projected_triangle.wgsl\"", shared)
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        let source = super::source();
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        assert!(module.entry_points.iter().any(|entry| entry.name == "cs_main"
            && entry.stage == naga::ShaderStage::Compute));
    }
}

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
