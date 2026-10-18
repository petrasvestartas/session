
#[cfg(test)]
mod tiles_contract {
    #[test]
    fn assembled_tiles_shader_interfaces_validate() {
        use wgpu::naga;
        let source = crate::finite_project_source::tiles();
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        for name in ["vs_main","fs_count","fs_fill"] {
            assert!(module.entry_points.iter().any(|entry| entry.name == name));
        }
    }
}
