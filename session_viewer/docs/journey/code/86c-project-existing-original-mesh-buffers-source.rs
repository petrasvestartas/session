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
