use crate::comparison;
use prost::Message;

#[test]
fn comparison_loads_the_same_four_original_solids(){
    let bytes=comparison::specimen();let source=session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    assert_eq!(source.objects.as_ref().unwrap().breps.len(),4);
    let scene=comparison::original_scene(&bytes).unwrap();assert_eq!(scene.objects().len(),4);
    assert!(scene.objects().iter().all(|row|row.cad.is_some()));
    let mut camera=crate::camera::Camera::default();camera.isometric();camera.fit(&scene.bounds().unwrap());
    assert!(camera.radius>500.);assert!(camera.view_projection().m.iter().all(|v|v.is_finite()));
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn maintained_renderer_draws_the_original_curved_specimen(){
    let directory=std::env::temp_dir().join(format!("viewer-course-reference-{}",std::process::id()));std::fs::create_dir_all(&directory).unwrap();
    let source=directory.join("source.pb");let image=directory.join("current.ppm");let original=comparison::specimen();std::fs::write(&source,&original).unwrap();
    let report=crate::reference_frame::render(&source,&image,[640,480]);assert!(report.contains("640x480"));
    let pixels=std::fs::read(&image).unwrap();let rgb=crate::reference_frame::rgb(&pixels,[640,480]).unwrap();
    let blue=rgb.chunks_exact(3).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();assert!(blue>4000,"maintained original blue faces: {blue}");
    assert_eq!(std::fs::read(&source).unwrap(),original);
    assert!(crate::reference_frame::rgb(&pixels[..pixels.len()-1],[640,480]).is_err());
    std::fs::remove_dir_all(directory).unwrap();
}
