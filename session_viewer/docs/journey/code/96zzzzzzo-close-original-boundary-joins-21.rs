use crate::{scene::Scene,prepared::PreparedMesh,pick_sources::{Mode,Part,Target},cap_gpu_tests::{renderer,pixels_at,wait}};
use session_rust::{Mesh,Point,Color,NurbsSurface,BRep,Line,Xform,Session,proto};
use std::rc::Rc;
use prost::Message;

fn panel()->Mesh {
    let mut mesh=Mesh::new();
    for (key,p) in [11,70,803,9004].into_iter().zip([[-900.,-650.,0.],[900.,-650.,0.],[900.,650.,0.],[-900.,650.,0.]]){mesh.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![11,70,803,9004],Some(17));mesh.set_objectcolor(Color::new(0.2,0.4,0.6,1.));mesh.set_linecolors(vec![],vec![7.]);mesh
}
fn camera()->crate::camera::Camera {let mut c=crate::camera_defaults_tests::top_fixture();c.distance=3000.;c.radius=1500.;c.aspect=4./3.;c}
fn projected(m:[f32;16],p:[f64;3])->[f64;2]{let c:[f64;4]=std::array::from_fn(|i|m[12+i]as f64+(0..3).map(|j|m[j*4+i]as f64*p[j]).sum::<f64>());[(c[0]/c[3]+1.)*160.,(1.-c[1]/c[3])*120.]}
fn rgba(image:&[u8],x:usize,y:usize)->&[u8]{crate::cap_gpu_tests::at(image,x,y)}
fn corner(image:&[u8],m:[f32;16],p:[f64;3],outside:[f64;2],density:f32)->[usize;2]{
    let at=projected(m,p);let sample=std::array::from_fn(|i|if outside[i]<0.{at[i].floor()as usize-density as usize}else{at[i].ceil()as usize+density as usize-1});
    let d:[f64;2]=std::array::from_fn(|i|sample[i]as f64+0.5-at[i]);assert!((0..2).all(|i|d[i]*outside[i]>0.49));assert!(d[0].hypot(d[1])<3.5*density as f64-0.75);
    assert_eq!(rgba(image,sample[0],sample[1]),[0,0,0,255],"Actual complete round corner, not the old exterior butt gap");sample
}
fn pick(gpu:&mut super::Renderer,scene:&Scene,m:[f32;16],pixel:[usize;2],mode:Mode)->Option<Target>{gpu.set_pick_mode(mode);gpu.request_pick([(pixel[0]as f32+0.5)/160.-1.,1.-(pixel[1]as f32+0.5)/120.],m).unwrap();wait(gpu);gpu.take_pick(m).unwrap().unwrap().and_then(|raw|gpu.resolve_pick(scene,raw))}
fn source(scene:&Scene)->(Vec<proto::Session>,serde_json::Value){let bundle=crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();(bundle.documents.iter().map(|b|proto::Session::decode(b.as_slice()).unwrap()).collect(),serde_json::to_value(bundle.metadata).unwrap())}
fn export_archive(name:&str,scene:&Scene){if let Some(root)=std::env::var_os("VIEWER_ROUND_CAP_PROOF_DIR"){let root=std::path::Path::new(&root);std::fs::create_dir_all(root).unwrap();std::fs::write(root.join(format!("{name}.session")),crate::scene_archive::save(scene).unwrap()).unwrap();}}
fn export(name:&str,image:&[u8]){if let Some(root)=std::env::var_os("VIEWER_ROUND_CAP_PROOF_DIR"){use std::io::Write;let root=std::path::Path::new(&root);std::fs::create_dir_all(root).unwrap();let mut f=std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();write!(f,"P6\n320 240\n255\n").unwrap();for p in image.chunks_exact(4){f.write_all(&p[..3]).unwrap();}}}

#[test]
fn original_mesh_surface_and_brep_round_corners_keep_source_queries() {
    for kind in ["mesh","surface","brep"]{
        let mut scene=Scene::empty();match kind{
            "mesh"=>{scene.insert(PreparedMesh::new(panel()).unwrap()).unwrap();},
            "surface"=>{let mut s=NurbsSurface::create(false,false,1,1,2,2,&[Point::new(-900.,-650.,0.),Point::new(-900.,650.,0.),Point::new(900.,-650.,0.),Point::new(900.,650.,0.)]).unwrap();s.width=7.;s.facecolors=vec![Color::new(0.2,0.4,0.6,1.)];scene.insert_surface(Rc::new(s)).unwrap();},
            _=>{let mut s=BRep::create_box(1800.,1300.,100.);s.width=7.;s.surfacecolor=Color::new(0.2,0.4,0.6,1.);scene.insert_cad(Rc::new(s)).unwrap();}
        }
        let original=source(&scene);let owner=scene.objects()[0].id;let mut gpu=renderer(&scene);gpu.set_opacity(1.);let uploads=gpu.stats()[0];let mut c=camera();
        for projection in [crate::camera::Projection::Orthographic,crate::camera::Projection::Perspective]{c.projection=projection;let m=c.uniform();for density in [1.,2.]{gpu.set_density(density);let before=pixels_at(&gpu,m);let z=if kind=="brep"{50.}else{0.};
            for (p,sign) in [([-900.,650.,z],[-1.,-1.]),([900.,650.,z],[1.,-1.]),([-900.,-650.,z],[-1.,1.]),([900.,-650.,z],[1.,1.])]{let at=corner(&before,m,p,sign,density);let target=pick(&mut gpu,&scene,m,at,Mode::Edge).unwrap();assert_eq!(target.owner,owner);assert!(matches!(target.part,Part::MeshEdge(..)|Part::SurfaceEdge(..)|Part::BRepEdge(..)));}
            let center=projected(m,[0.,0.,z]).map(|v|v.floor()as usize);let face=pick(&mut gpu,&scene,m,center,Mode::Face).unwrap();assert_eq!(face.owner,owner);if kind=="mesh"{assert_eq!(face.part,Part::Face(17));}
            assert!(pixels_at(&gpu,m)==before);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source(&scene),original);
            if projection==crate::camera::Projection::Orthographic&&density==1.{export(&format!("{kind}-round-corners"),&before);export_archive(&format!("{kind}-round-corners"),&scene);}
        }}
    }
}

fn line(a:[f64;3],b:[f64;3])->Scene{let mut s=Line::new(a[0],a[1],a[2],b[0],b[1],b[2]);s.width=7.;s.linecolor=Color::new(0.,0.,0.,1.);let mut scene=Scene::empty();scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(s)).unwrap()).unwrap();scene}
#[test]
fn original_end_on_and_near_cut_lines_share_color_and_pick_coverage() {
    for density in [1.,2.]{
        let mut images=Vec::new();for(a,b)in[([0.,0.,-100.],[0.,0.,100.]),([0.,0.,100.],[0.,0.,-100.])]{let scene=line(a,b);let original=source(&scene);let owner=scene.lines()[0].id;let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.set_density(density);let m=camera().uniform();let image=pixels_at(&gpu,m);let at=projected(m,[0.,0.,0.]).map(|v|v.floor()as usize);assert_eq!(rgba(&image,at[0],at[1]),[0,0,0,255],"A nonzero original line viewed end-on remains a visible disk");assert_eq!(pick(&mut gpu,&scene,m,at,Mode::Edge),Some(Target{owner,part:Part::Curve}));assert_eq!(source(&scene),original);if density==1.{export_archive("end-on-original",&scene);}images.push(image);}
        assert!(images[0]==images[1],"Nearest reverse-depth end is independent of source endpoint order");if density==1.{export("end-on-original",&images[0]);}
        let scene=line([-0.6,0.,0.5],[0.6,0.,1.5]);let original=source(&scene);let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.set_density(density);let m=Xform::identity().m.map(|v|v as f32);let image=pixels_at(&gpu,m);
        assert_eq!(rgba(&image,150,120),[0,0,0,255]);assert_eq!(rgba(&image,162,120),[255,255,255,255],"A clipped near-plane cut does not invent a round source end");assert_eq!(source(&scene),original);if density==1.{export("flat-near-cut",&image);export_archive("flat-near-cut",&scene);}
    }
}

#[test]
fn round_caps_keep_legacy_fallback_source_release_and_shared_geometry() {
    let mut document=Session::new("Original round caps");document.add_mesh(panel(),None);let mut editor=crate::editor::Editor::default();editor.apply(crate::editor::Action::ReplaceAt(document.to_proto().encode_to_vec(),Rc::new(crate::reload_url::ReloadUrl::new("test:round-caps".into())))).unwrap();
    let owner=editor.scene.objects()[0].id;let original=source(&editor.scene);let display=Rc::clone(&editor.scene.objects()[0].mesh);let weak=Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());let mut gpu=renderer(&editor.scene);gpu.set_opacity(1.);let m=camera().uniform();let before=pixels_at(&gpu,m);let uploads=gpu.stats()[0];let metadata=gpu.mesh_ink.clipping_bytes();assert_eq!(metadata[0],4*8);
    let placement=editor.scene.placement(owner).unwrap();editor.scene.place(owner,&Xform::translation(100.,0.,0.)*&placement).unwrap();gpu.set_scene(&editor.scene,None);assert_ne!(pixels_at(&gpu,m),before);editor.scene.place(owner,placement).unwrap();gpu.set_scene(&editor.scene,None);assert!(pixels_at(&gpu,m)==before);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source(&editor.scene),original);
    gpu.mesh_ink.use_legacy_fixture();assert!(pixels_at(&gpu,m)==before,"Actual existing legacy pipeline keeps the same complete unoccluded boundary footprint");assert_eq!(gpu.mesh_ink.clipping_bytes(),metadata);
    editor.apply(crate::editor::Action::UnloadSources).unwrap();assert!(weak.upgrade().is_none());assert!(Rc::ptr_eq(&display,&editor.scene.objects()[0].mesh));gpu.set_scene(&editor.scene,None);assert!(pixels_at(&gpu,m)==before);assert_eq!(gpu.stats()[0],uploads);
    let mesh=panel();let guid=mesh.guid().to_owned();let mut message=crate::instance_specimen::message();message.definitions.as_mut().unwrap().meshes=vec![mesh.to_proto()];for instance in &mut message.objects.as_mut().unwrap().instances{instance.definition_guid=guid.clone();instance.xform=None;instance.flags=0;}
    message.xforms=vec![proto::XformEntry{guid:"first".into(),xform:Some(Xform::translation(-1200.,0.,0.).to_proto())},proto::XformEntry{guid:"second".into(),xform:Some((&Xform::translation(1200.,0.,0.)*&Xform::scale_xyz(-0.8,1.2,1.4)).to_proto())}];
    let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&message.encode_to_vec(),"",&Xform::identity(),0.).unwrap();assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));let original=source(&scene);let mut gpu=renderer(&scene);gpu.set_opacity(1.);let mut c=camera();c.distance=6000.;c.radius=3000.;let m=c.uniform();
    for density in [1.,2.]{gpu.set_density(density);let image=pixels_at(&gpu,m);corner(&image,m,[-2100.,650.,0.],[-1.,-1.],density);corner(&image,m,[1920.,780.,0.],[1.,-1.],density);assert_eq!(source(&scene),original);if density==1.{export("shared-mirrored-round-corners",&image);export_archive("shared-mirrored-round-corners",&scene);}}
}
