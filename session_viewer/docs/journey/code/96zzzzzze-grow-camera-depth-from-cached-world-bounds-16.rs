use crate::{bounds::Bounds,camera::{Camera,Projection,Unit,View},scene::Scene,prepared::PreparedMesh};
use std::rc::Rc;
fn pose(camera:&Camera)->Vec<f64>{
    let mut out=camera.target.to_vec();out.extend([camera.distance,camera.aspect]);out.extend(camera.frame);
    for axis in [session_rust::Vector::x_axis(),session_rust::Vector::y_axis(),session_rust::Vector::z_axis()] {let v=camera.orientation.rotate_vector(axis);out.extend((0..3).map(|i|v[i]));}out
}
#[test]
fn cached_extent_matches_current_growth_in_both_units_without_reframing(){
    let bounds=Bounds{min:[-7.,-3.,-11.],max:[5.,13.,17.]};let corners=bounds.corners();
    assert_eq!(corners.map(|p|p.map(f64::to_bits)).into_iter().collect::<std::collections::HashSet<_>>().len(),8);
    assert!(corners.contains(&bounds.min)&&corners.contains(&bounds.max));
    for unit in [Unit::Millimeters,Unit::Meters] {for physical_target in [[0.;3],[10.,-20.,30.],[1e6,-2e6,3e6]] {
        let scale=unit.to_meters();let mut camera=Camera::default();camera.unit=unit;camera.target=physical_target.map(|v|v/scale);camera.radius=0.;camera.frame=[-0.6,-0.8,0.9,0.7];let before=pose(&camera);
        let bounds=Bounds{min:bounds.min.map(|v|v/scale),max:bounds.max.map(|v|v/scale)};
        let mut current=crate::camera_reference_tests::Camera::new();current.unit=match unit{Unit::Millimeters=>crate::camera_reference_tests::Unit::Millimeters,Unit::Meters=>crate::camera_reference_tests::Unit::Meters};current.target=physical_target;current.scene_extent=0.;
        let points=bounds.corners().map(|p|session_rust::Point::new(p[0],p[1],p[2]));let aabb=session_rust::AABB::from_points(&points,0.);
        current.grow_extent(&aabb);camera.grow_extent(&bounds);
        assert!((camera.radius*scale-current.scene_extent).abs()<=current.scene_extent.max(1.)*1e-12);assert_eq!(pose(&camera),before);
        let grown=camera.radius;camera.grow_extent(&Bounds::point(camera.target));assert_eq!(camera.radius,grown);
        for bad in [Bounds{min:[f64::NAN,0.,0.],max:[1.;3]},Bounds{min:[0.;3],max:[f64::INFINITY;3]},Bounds{min:[2.;3],max:[1.;3]}] {camera.grow_extent(&bad);assert_eq!(camera.radius,grown);}
    }}
}
#[test]
fn numeric_bounds_outlive_originals_without_retaining_source_owners(){
    let mut scene=Scene::empty();let source=Rc::new(session_rust::Mesh::create_box(2.,4.,6.));let weak=Rc::downgrade(&source);
    let mut prepared=PreparedMesh::from_shared(source.clone()).unwrap();prepared.model=session_rust::Xform::translation(100.,200.,-20000.);scene.insert(prepared).unwrap();
    let mut renderer=crate::cap_gpu_tests::renderer(&scene);let uploads=renderer.stats()[0];let expected=scene.bounds().unwrap();
    drop(scene);drop(source);assert!(weak.upgrade().is_none(),"Numeric bounds cannot keep an original source or Scene alive");
    let mut camera=Camera::default();camera.radius=0.;let before=pose(&camera);
    let mut current=Camera::default();current.radius=0.;current.grow_extent(&expected);
    for _ in 0..1000 {renderer.grow_camera(&mut camera);}
    assert_eq!(camera.radius,current.radius);assert_eq!(pose(&camera),before);assert_eq!(renderer.stats()[0],uploads);
    renderer.set_scene(&Scene::empty(),None);let mut fresh=Camera::default();fresh.radius=0.;renderer.grow_camera(&mut fresh);assert_eq!(fresh.radius,0.);
}
#[test]
fn late_originals_expand_depth_before_real_pixels_and_original_gpu_picks(){
    let mut scene=Scene::empty();let mut renderer=crate::cap_gpu_tests::renderer(&scene);
    let mut camera=Camera::default();camera.set_view(View::Top);camera.projection=Projection::Orthographic;camera.aspect=4./3.;camera.radius=0.;
    let point=session_rust::Point::new(0.,0.,-20000.);assert!(camera.view_projection().transform_point(&point)[2]<0.,"Positive control lies beyond the previous range");
    let owner=scene.insert(PreparedMesh::triangle([[-500.,-500.,-20000.],[500.,-500.,-20000.],[0.,500.,-20000.]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();
    renderer.set_scene(&scene,None);let uploads=renderer.stats()[0];let before=pose(&camera);renderer.grow_camera(&mut camera);
    let depth=camera.view_projection().transform_point(&point)[2];assert!(depth>0.&&depth<1.);assert_eq!(pose(&camera),before);
    let matrix=renderer.camera_matrix(&camera);let pixels=crate::cap_gpu_tests::pixels_at(&renderer,matrix);assert_ne!(&crate::cap_gpu_tests::at(&pixels,160,120)[..3],&[255;3]);
    renderer.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&renderer);assert_eq!(renderer.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(renderer.stats()[0],uploads);
    let extent=camera.radius;Rc::make_mut(&mut scene.view).hide(&[owner],true);renderer.set_scene(&scene,None);assert!(renderer.world_bounds.borrow().is_none());renderer.grow_camera(&mut camera);assert_eq!(camera.radius,extent);
    Rc::make_mut(&mut scene.view).show_all();scene.translate(owner,[1000.,0.,0.]).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.world_bounds.borrow().as_ref().unwrap().min[0],500.);renderer.grow_camera(&mut camera);assert!(camera.radius>=extent);
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);assert!((uploads..=uploads+1).contains(&renderer.stats()[0]));
}
#[test]
fn active_preview_font_metrics_refresh_numeric_bounds_only_when_changed(){
    let mut editor=crate::editor::Editor::default();editor.scene.clear();
    let source=Rc::new(crate::scene_label::example(crate::text::TextPlacement::WorldPlane{world:[0.;3],right:[1.,0.,0.],up:[0.,1.,0.],world_height:100.}));
    let mut prepared=crate::scene_label::Prepared::new(source.clone()).unwrap();prepared.extent=[0.;2];let owner=editor.scene.insert_text(prepared).unwrap();
    let mut preview=crate::edit_preview::Preview::new(&editor.scene,vec![owner]).unwrap();preview.set(session_rust::Xform::translation(100.,0.,-20000.)).unwrap();editor.edit_preview=Some(preview);
    let mut renderer=crate::cap_gpu_tests::renderer(editor.display_scene());let uploads=renderer.stats()[0];
    assert!(renderer.refresh_text_bounds(editor.display_scene_mut()));assert_eq!(editor.scene.texts()[0].prepared.extent,[0.;2]);assert!(editor.display_scene().texts()[0].prepared.extent[0]>0.);
    assert!(Rc::ptr_eq(&editor.display_scene().texts()[0].prepared.source,&source));
    let expected=editor.display_scene().bounds().unwrap();assert_eq!(renderer.world_bounds.borrow().as_ref().unwrap().min,expected.min);assert_eq!(renderer.world_bounds.borrow().as_ref().unwrap().max,expected.max);
    assert!(!renderer.refresh_text_bounds(editor.display_scene_mut()));assert_eq!(renderer.stats()[0],uploads);
    let mut camera=Camera::default();camera.radius=0.;let before=pose(&camera);renderer.grow_camera(&mut camera);assert!(camera.radius>19000.);assert_eq!(pose(&camera),before);
    editor.edit_preview=None;renderer.set_scene(&editor.scene,None);assert!(renderer.refresh_text_bounds(editor.display_scene_mut()));assert!(!renderer.refresh_text_bounds(editor.display_scene_mut()));assert!(editor.scene.texts()[0].prepared.extent[0]>0.);assert!(renderer.world_bounds.borrow().as_ref().unwrap().min[2]>-1.);
}
