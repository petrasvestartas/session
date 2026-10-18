use crate::{camera::{Camera,Projection},cap_gpu_tests::{renderer,pixels_at,at},scene::Scene};
fn original()->session_rust::Mesh{
    let mut mesh=session_rust::Mesh::from_vertices_and_faces(vec![session_rust::Point::new(0.,-800.,-800.),session_rust::Point::new(0.,800.,-800.),session_rust::Point::new(0.,800.,800.),session_rust::Point::new(0.,-800.,800.)],vec![vec![0,1,2,3]]);mesh.set_objectcolor(session_rust::Color::new(0.2,0.4,0.6,1.));mesh
}
#[test]
fn actual_headlight_backfaces_and_original_record_restoration(){
    let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(original()).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();
    let mut c=Camera::default();c.aspect=4./3.;let mut r=renderer(&scene);
    for mirrored in [false,true]{
        scene.place_owner(owner,if mirrored{session_rust::Xform::scale_xyz(1.,-1.,1.)}else{session_rust::Xform::identity()}).unwrap();r.set_scene(&scene,None);let uploads=r.stats()[0];
        for projection in [Projection::Orthographic,Projection::Perspective]{c.projection=projection;let matrix=c.uniform();r.set_lighting(false,false);let flat=pixels_at(&r,matrix);assert_eq!(at(&flat,160,120),&[51,102,153,255]);
            r.set_lighting(true,false);let head=pixels_at(&r,matrix);assert_ne!(head,flat);let color=at(&head,160,120);assert!(color[0]>10&&color[0]<51&&color[1]<102&&color[2]<153,"{projection:?} headlight {color:?}");
            r.set_lighting(false,true);let back=pixels_at(&r,matrix);assert_eq!(at(&back,160,120),&[204,13,13,255],"Original backface survives mirrored placement");
            for uniform in [true,false]{if uniform{r.use_uniform_surface_records();}else{r.set_scene(&scene,None);}
                r.set_lighting(true,false);assert_eq!(pixels_at(&r,matrix),head);r.set_lighting(false,true);assert_eq!(pixels_at(&r,matrix),back);
                r.set_lighting(false,false);assert_eq!(pixels_at(&r,matrix),flat);
            }assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);
        }
    }
    let mut front=original();for face in front.face.values_mut(){face.reverse();}let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::new(front).unwrap()).unwrap();r.set_scene(&scene,None);r.set_lighting(false,true);assert_eq!(at(&pixels_at(&r,c.uniform()),160,120),&[51,102,153,255],"Front-facing original retains its source color");
}
