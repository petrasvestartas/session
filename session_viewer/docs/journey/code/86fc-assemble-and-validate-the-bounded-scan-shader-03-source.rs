
#[cfg(test)]
mod scan_contract {
    #[test]
    fn assembled_scan_shader_interfaces_validate() {
        use wgpu::naga;
        let source = crate::finite_project_source::scan();
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        for name in ["scan_tiles","scan_blocks","finish_offsets"] {
            assert!(module.entry_points.iter().any(|entry| entry.name == name));
        }
    }
}
