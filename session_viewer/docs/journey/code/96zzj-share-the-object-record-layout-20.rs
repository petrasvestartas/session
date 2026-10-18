#[test]
fn assembled_object_uniform_and_instance_array_match_host_layout() {
    use wgpu::naga;
    let uniform = crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0);
    let instances = uniform.replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
        .replace("return object_settings;", "return objects[index];");
    for (source, offsets) in [(uniform, vec![0, 64, 80, 144]), (instances, vec![0, 64, 80, 144]),
        (crate::finite_project_source::source(), vec![0, 64, 68, 72, 80, 144])] {
        let module = naga::front::wgsl::parse_str(&source).unwrap();
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        let (_, ty) = module.types.iter().find(|(_, ty)| ty.name.as_deref() == Some("ObjectSettings")).unwrap();
        let naga::TypeInner::Struct { members, span } = &ty.inner else { panic!("ObjectSettings must be a struct") };
        assert_eq!(*span as usize, crate::object_record::BYTES);
        assert_eq!(members.iter().map(|member| member.offset).collect::<Vec<_>>(), offsets);
    }
}
