use crate::{camera::{Camera,Projection,Unit},cap_gpu_tests::{renderer,pixels_at,at},scene::Scene};
use session_rust::{Point,Vector,Xform};

fn original() -> session_rust::Mesh {
    let mut mesh=session_rust::Mesh::from_vertices_and_faces(vec![Point::new(0.,-800.,-800.),Point::new(0.,800.,-800.),Point::new(0.,800.,800.),Point::new(0.,-800.,800.)],vec![vec![0,1,2,3]]);
    mesh.set_objectcolor(session_rust::Color::new(0.2,0.4,0.6,1.));mesh
}
fn camera()->Camera {let mut c=Camera::default();c.projection=Projection::Orthographic;c.aspect=4./3.;c}

#[test]
fn surface_frame_bytes_preserve_camera_and_display_domains() {
    assert_eq!(crate::surface_view::BYTES,112);
    for unit in [Unit::Millimeters,Unit::Meters] {for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut c=camera();c.unit=unit;c.distance=3./unit.to_meters();c.projection=projection;c.target=[1e9,-1e9,1e9];
        let matrix=c.view_projection_anchored(c.target).to_f32();
        let bytes=crate::surface_view::bytes(matrix,[1.,0.7,2.,1.],[640,480],2.);
        let values=bytes.chunks_exact(4).map(|b|f32::from_ne_bytes(b.try_into().unwrap())).collect::<Vec<_>>();
        assert_eq!(&values[..16],&matrix);assert_eq!(&values[16..20],&[640.,480.,2.,0.7]);assert_eq!(&values[20..24],&[1.,0.7,2.,1.]);
        assert!(values.iter().all(|v|v.is_finite()));
        if projection==Projection::Perspective {
            assert_eq!(values[27],0.);let forward=c.orientation.rotate_vector(Vector::y_axis());
            for i in 0..3 {let expected=-forward[i]*c.distance;assert!((values[24+i]as f64-expected).abs()<c.distance*2e-6);}
        }else{assert!(values[27]>0.);}
    }}
}

#[test]
fn scaled_camera_inverse_keeps_wide_views_and_invalid_guards() {
    for unit in [Unit::Millimeters,Unit::Meters] {for projection in [Projection::Perspective,Projection::Orthographic] {
        let mut c=camera();c.unit=unit;c.distance=24./unit.to_meters();c.projection=projection;
        let matrix=c.view_projection().to_f32();let transform=Xform::from_matrix(matrix.map(f64::from));
        let inverse=crate::clipping_uniform::inverse(&transform).unwrap();let identity=&transform*&inverse;
        for i in 0..16 {assert!((identity.m[i]-if i%5==0{1.}else{0.}).abs()<1e-7);}
        assert!(crate::ambient_camera::uniform(matrix,[320,240],[128,96],[0.,1.]).is_some());
        if unit==Unit::Millimeters&&projection==Projection::Orthographic {assert!(transform.inverse().is_none(),"The real unscaled-kernel regression control must fail");}
    }}
    for value in [0.,f64::NAN,f64::INFINITY] {assert!(crate::clipping_uniform::inverse(&Xform::from_matrix([value;16])).is_none());}
}

#[test]
fn actual_unlit_and_sky_preserve_original_records() {
    let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(original()).unwrap()).unwrap();
    let source=scene.objects()[0].geometry().unwrap().to_proto();let matrix=camera().uniform();let mut r=renderer(&scene);let uploads=r.stats()[0];
    let flat=pixels_at(&r,matrix);assert_eq!(at(&flat,160,120),&[51,102,153,255],"Unlit fill retains exact original color");
    // Isolate the actual sky fragment mode from the separate ambient compositor.
    r.ambient_enabled.set(true);assert!(r.ambient.borrow().is_none());let sky=pixels_at(&r,matrix);
    assert_eq!(at(&sky,160,120),&[44,88,132,255],"Current vertical-face sky factor is0.86");
    r.ambient_enabled.set(false);
    for uniform in [true,false] {
        if uniform {r.use_uniform_surface_records();}else{r.set_scene(&scene,None);}
        assert_eq!(pixels_at(&r,matrix),flat);
        r.ambient_enabled.set(true);assert_eq!(pixels_at(&r,matrix),sky);r.ambient_enabled.set(false);
    }
    assert_eq!(pixels_at(&r,matrix),flat);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].id,owner);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn wide_ambient_and_depth_pyramid_keep_real_passes_without_geometry_uploads() {
    let mut scene=Scene::empty();let source=session_rust::Mesh::create_box(6000.,4000.,1000.);scene.insert(crate::prepared::PreparedMesh::new(source).unwrap()).unwrap();
    let original=scene.objects()[0].geometry().unwrap().to_proto();let mut c=camera();c.set_view(crate::camera::View::Top);c.distance=24000.;let r=renderer(&scene);r.grow_camera(&mut c);
    r.set_ambient(true);r.set_depth_pyramid(true);let uploads=r.stats()[0];let first=pixels_at(&r,c.uniform());
    assert_eq!(r.ambient_error(),None);assert_eq!(r.depth_pyramid_error(),None);assert!(r.ambient_usage()[4]>0);assert_eq!(r.depth_pyramid_usage()[3],6);
    let ambient=r.ambient_usage();let pyramid=r.depth_pyramid_usage();assert_eq!(pixels_at(&r,c.uniform()),first);assert_eq!(r.ambient_usage()[3],ambient[3]);assert_eq!(r.depth_pyramid_usage(),pyramid);
    c.zoom(2.);let _=pixels_at(&r,c.uniform());assert_eq!(r.ambient_error(),None);assert_eq!(r.depth_pyramid_error(),None);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}
