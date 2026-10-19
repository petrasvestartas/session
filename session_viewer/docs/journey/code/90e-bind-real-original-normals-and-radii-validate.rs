#[test]
fn ambient_shader_contracts_validate_before_pipeline_connection() {
    let sources = [
        crate::clipping_gpu::source(include_str!("ambient.wgsl"), 0, 5),
        include_str!("ambient_composite.wgsl").to_string(),
    ];
    for source in sources {
        let module = wgpu::naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        wgpu::naga::valid::Validator::new(
            wgpu::naga::valid::ValidationFlags::all(),
            wgpu::naga::valid::Capabilities::all(),
        ).validate(&module).expect("valid ambient GPU contract");
        assert!(module.entry_points.iter().any(|entry| entry.name == "vs_main"));
        assert!(module.entry_points.iter().any(|entry| entry.name == "fs_main"));
    }
}
