#[cfg(not(target_arch="wasm32"))]
fn maintained_root()->std::path::PathBuf{
    let manifest=include_str!("../Cargo.toml");let line=manifest.lines().find(|line|line.starts_with("current_viewer = ")).expect("Maintained native reference dependency");let path=line.split("path = \"").nth(1).unwrap().split('"').next().unwrap();std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn all_declared_current_rust_sources_and_external_dependencies_match(){
    let snapshot=crate::source_audit::recorded();assert_eq!(snapshot.session_head.len(),40);assert_eq!(snapshot.wood_head.len(),40);crate::source_audit::verify(&maintained_root(),&snapshot).unwrap();assert_eq!(snapshot.files.iter().filter(|v|v.root=="viewer").count(),312);
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn all_current_registered_names_options_and_aliases_match_the_observed_grammar(){
    let expected:serde_json::Value=serde_json::from_str(include_str!("current_registry.json")).unwrap();assert_eq!(crate::registry_audit::snapshot(),expected);
    let rows=expected.as_array().unwrap();assert_eq!(rows.len(),86);assert_eq!(rows.iter().map(|v|v["options"].as_array().unwrap().len()).sum::<usize>(),117);assert_eq!(rows.iter().map(|v|v["aliases"].as_array().unwrap().len()).sum::<usize>(),15);
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn current_command_differences_and_canvas_letters_are_explicit(){
    use current_viewer::app::{command,keys};
    assert!(command::parse("Controls").is_ok());assert!(command::parse("Controls On").is_err());assert!(crate::commands::parse("Controls On").is_ok());
    assert!(command::parse("Opacity 0.4 extra").is_ok());assert!(crate::commands::parse("Opacity 0.4 extra").is_err());
    for line in ["Opacity NaN","Opacity -1","Opacity 2","Length BoundingBox extra","BoundingBox extra"]{assert!(command::parse(line).is_err(),"{line}");}
    for(letter,line)in[("h","Hide"),("S","Show"),("f","Fit")]{assert_eq!(keys::letter(letter),Some(line));assert!(command::parse(line).is_ok());}assert_eq!(keys::letter("q"),None);
}
