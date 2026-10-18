#[cfg(not(target_arch="wasm32"))]
#[test]
fn current_finite_triangle_shader_recipes_validate_with_explicit_course_bindings(){
    use wgpu::naga;
    let mut sources=vec![("project",crate::finite_project_source::source()),("tiles",crate::finite_project_source::tiles()),("scan",crate::finite_project_source::scan())];for stride in [44,72,32]{sources.push(("finite color",crate::finite_sources::source(stride,false)));sources.push(("finite pick",crate::finite_sources::source(stride,true)));}
    for(name,source)in sources {
        let module=naga::front::wgsl::parse_str(&source).unwrap_or_else(|error|panic!("{name}: {}",error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&module).unwrap_or_else(|error|panic!("{name}: {}",error.emit_to_string(&source)));
    }
}
