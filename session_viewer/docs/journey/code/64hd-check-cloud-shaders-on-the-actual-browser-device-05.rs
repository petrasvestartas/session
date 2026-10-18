#[test]
fn completed_cloud_modules_have_valid_vertex_and_fragment_interfaces() {
    use wgpu::naga;
    for source in [include_str!("cloud_points.wgsl"), include_str!("cloud_resolve.wgsl")] {
        let module = naga::front::wgsl::parse_str(source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        assert!(module.entry_points.iter().any(|entry| entry.stage == naga::ShaderStage::Vertex));
        assert!(module.entry_points.iter().any(|entry| entry.stage == naga::ShaderStage::Fragment));
    }
}
