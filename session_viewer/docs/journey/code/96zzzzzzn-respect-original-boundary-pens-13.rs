use crate::{scene::Scene,editor::{Editor,Action},prepared::PreparedMesh,pick_sources::{Mode,Part,Target,Sources},camera::Projection,cap_gpu_tests::{renderer,pixels_at,wait}};
use session_rust::{Mesh,Point,Color,NurbsSurface,BRep,Session,Xform,proto};
use prost::Message;
use std::rc::Rc;

fn panel(width:f64)->Mesh {
    let mut source=Mesh::from_vertices_and_faces(vec![Point::new(-900.,-650.,0.),Point::new(900.,-650.,0.),Point::new(900.,650.,0.),Point::new(-900.,650.,0.)],vec![vec![0,1,2,3]]);
    source.set_objectcolor(Color::new(0.2,0.4,0.6,1.));source.set_linecolors(vec![],vec![width]);source
}
fn sparse()->Mesh {
    let mut source=Mesh::new();let keys=[11,70,803,9004,500000,1000000];
    for (key,p) in keys.into_iter().zip([[-900.,-650.,0.],[0.,-650.,0.],[900.,-650.,0.],[-900.,650.,0.],[0.,650.,0.],[900.,650.,0.]]) {assert_eq!(source.add_vertex(Point::new(p[0],p[1],p[2]),Some(key)),key);}
    source.add_face(vec![11,70,500000,9004],Some(17));source.add_face(vec![70,803,1000000,500000],Some(93));source.set_objectcolor(Color::new(0.2,0.4,0.6,1.));
    source.set_linecolors(vec![Color::new(1.,0.,0.,1.),Color::new(1.,0.,1.,1.),Color::new(0.,1.,0.,1.),Color::new(0.,0.,1.,1.),Color::new(1.,0.,1.,1.),Color::new(0.,1.,1.,1.),Color::new(1.,0.5,0.,1.)],vec![7.,9.,3.,5.,0.,4.,6.]);source
}
fn camera()->crate::camera::Camera {let mut c=crate::camera_defaults_tests::top_fixture();c.distance=3000.;c.radius=1500.;c.aspect=4./3.;c}
fn point(matrix:[f32;16],p:[f64;3])->(usize,usize) {let p=super::contact_feature_tests::project(matrix,p);((((p[0]+1.)*160.).floor()as usize).min(319),(((1.-p[1])*120.).floor()as usize).min(239))}
fn at(image:&[u8],matrix:[f32;16],p:[f64;3])->[u8;4] {let(x,y)=point(matrix,p);crate::cap_gpu_tests::at(image,x,y).try_into().unwrap()}
fn span(image:&[u8],matrix:[f32;16],p:[f64;3],horizontal:bool,accept:impl Fn(&[u8])->bool)->usize {
    let(x,y)=point(matrix,p);(-24..=24).filter(|delta|{let (x,y)=if horizontal{(x as isize+delta,y as isize)}else{(x as isize,y as isize+delta)};x>=0&&y>=0&&x<320&&y<240&&accept(crate::cap_gpu_tests::at(image,x as usize,y as usize))}).count()
}
fn pick(gpu:&mut crate::renderer::Renderer,scene:&Scene,matrix:[f32;16],p:[f64;3],mode:Mode)->Option<Target> {gpu.set_pick_mode(mode);gpu.request_pick(super::contact_feature_tests::project(matrix,p),matrix).unwrap();wait(gpu);gpu.take_pick(matrix).unwrap().unwrap().and_then(|raw|gpu.resolve_pick(scene,raw))}
fn archive(scene:&Scene)->(Vec<proto::Session>,serde_json::Value) {let bundle=crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();(bundle.documents.iter().map(|bytes|proto::Session::decode(bytes.as_slice()).unwrap()).collect(),serde_json::to_value(bundle.metadata).unwrap())}
fn export(name:&str,scene:&Scene,pixels:Option<&[u8]>) {
    if let Some(root)=std::env::var_os("VIEWER_SOURCE_PEN_PROOF_DIR") {use std::io::Write;let root=std::path::Path::new(&root);std::fs::create_dir_all(root).unwrap();let bytes=crate::scene_archive::save(scene).unwrap();let bundle=crate::archive::Bundle::decode(&bytes).unwrap();assert_eq!(bundle.documents.len(),1);std::fs::write(root.join(format!("{name}.session")),bytes).unwrap();std::fs::write(root.join(format!("{name}.pb")),&bundle.documents[0]).unwrap();if let Some(pixels)=pixels {let mut file=std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();write!(file,"P6\n320 240\n255\n").unwrap();for pixel in pixels.chunks_exact(4){file.write_all(&pixel[..3]).unwrap();}}}
}
fn mesh_scene(source:Mesh)->Scene {let mut scene=Scene::empty();scene.insert(PreparedMesh::new(source).unwrap()).unwrap();scene}

#[test]
fn authored_mesh_pens_retain_sparse_keys_colors_and_flat_face_queries() {
    let mut scene=mesh_scene(sparse());let source=scene.objects()[0].geometry().unwrap().to_proto();let initial=archive(&scene);let owner=scene.objects()[0].id;
    let targets=Sources::new(&scene).unwrap();assert_eq!(targets.faces.iter().map(|t|t.part).collect::<Vec<_>>(),[Part::Face(17),Part::Face(93)]);assert_eq!(targets.mesh_ink.len(),5,"Seven true source edges, minus the coplanar seam and authored hidden edge");assert!(targets.edges.iter().any(|t|t.part==Part::MeshEdge(70,500000)),"Omitted ink does not rewrite source identity");
    let mut gpu=renderer(&scene);gpu.set_opacity(1.);let uploads=gpu.stats()[0];let mut c=camera();
    for projection in [Projection::Orthographic,Projection::Perspective] {c.projection=projection;let matrix=c.uniform();for density in [1.,2.] {gpu.set_density(density);let before=pixels_at(&gpu,matrix);assert_eq!(at(&before,matrix,[0.,0.,0.]),[51,102,153,255],"A coplanar source seam has no decorative stroke");
        assert!(span(&before,matrix,[-450.,-650.,0.],false,|p|p[0]>220&&p[1]<40&&p[2]<40)>=4*density as usize,"Authored red seven-pixel pen belongs to the first encountered edge");assert!(span(&before,matrix,[-900.,0.,0.],true,|p|p[2]>220&&p[0]<40&&p[1]<40)>=2*density as usize,"Source encounter order preserves the blue left pen");
        assert_eq!(pick(&mut gpu,&scene,matrix,[0.,0.,0.],Mode::Edge),None,"The coplanar seam is not visible selectable ink");assert_eq!(pick(&mut gpu,&scene,matrix,[450.,-650.,0.],Mode::Edge),None,"Authored zero-width edge remains hidden");assert_eq!(pick(&mut gpu,&scene,matrix,[-450.,-650.,0.],Mode::Edge),Some(Target{owner,part:Part::MeshEdge(11,70)}));for (x,key) in [(-450.,17),(450.,93)] {assert_eq!(pick(&mut gpu,&scene,matrix,[x,0.,0.],Mode::Face),Some(Target{owner,part:Part::Face(key)}));}
        assert_eq!(pixels_at(&gpu,matrix),before);assert_eq!(archive(&scene),initial);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);if projection==Projection::Orthographic&&density==1. {export("sparse-pens",&scene,Some(&before));}
    }}
    gpu.set_density(1.);Rc::make_mut(&mut scene.view).edges.insert(owner,[255,0,255]);gpu.set_scene(&scene,None);let override_pixels=pixels_at(&gpu,c.uniform());assert!(span(&override_pixels,c.uniform(),[-900.,0.,0.],true,|p|p[0]>220&&p[2]>220&&p[1]<40)>=2,"Existing explicit display-edge override wins");assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);
}

fn surface(width:f64)->NurbsSurface {let mut source=NurbsSurface::create(false,false,1,1,2,2,&[Point::new(-900.,-650.,0.),Point::new(-900.,650.,0.),Point::new(900.,-650.,0.),Point::new(900.,650.,0.)]).unwrap();source.width=width;source.facecolors=vec![Color::new(0.2,0.4,0.6,1.)];source}
#[test]
fn actual_surface_and_brep_boundary_widths_use_css_pens() {
    for kind in ["mesh","surface","brep"] {let mut scenes=Vec::new();for width in [1.,7.,0.,-2.] {let mut scene=Scene::empty();match kind {"mesh"=>{scene.insert(PreparedMesh::new(panel(width)).unwrap()).unwrap();},"surface"=>{scene.insert_surface(Rc::new(surface(width))).unwrap();},_=>{let mut source=BRep::create_box(1800.,1300.,100.);source.width=width;source.surfacecolor=Color::new(0.2,0.4,0.6,1.);scene.insert_cad(Rc::new(source)).unwrap();}}scenes.push(scene);}
        let mut gpu=renderer(&scenes[0]);gpu.set_opacity(1.);let c=camera();let matrix=c.uniform();let sample=[900.,0.,if kind=="brep" {50.} else {0.}];
        for density in [1.,2.] {gpu.set_density(density);let mut images=Vec::new();let mut widths=Vec::new();for (index,scene) in scenes.iter().enumerate() {let original=archive(scene);gpu.set_scene(scene,None);let pixels=pixels_at(&gpu,matrix);widths.push(span(&pixels,matrix,sample,true,|p|p[..3].iter().all(|v|*v<64)));assert_eq!(archive(scene),original);if density==1. {export(&format!("{kind}-{}",["default","wide","zero","negative"][index]),scene,Some(&pixels));}images.push(pixels);}
            assert!(widths[1]>=widths[0]+4*density as usize,"Actual {kind} seven-CSS-pixel border must be wider: {widths:?}");if kind!="mesh" {assert_eq!(images[2],images[0],"Nonpositive {kind} width uses the default pen");}assert_eq!(images[3],images[0],"Negative width is the default pen, not a world-unit stroke");
        }
    }
}

#[test]
fn prepared_pens_survive_unload_and_shared_placement() {
    let mut document=Session::new("Original unload pens");document.add_mesh(panel(7.),None);let bytes=document.to_proto().encode_to_vec();let mut editor=Editor::default();editor.apply(Action::ReplaceAt(bytes.clone(),Rc::new(crate::reload_url::ReloadUrl::new("test:source-pens".into())))).unwrap();editor.camera=camera();let source=archive(&editor.scene);let display=Rc::clone(&editor.scene.objects()[0].mesh);let original=Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());let document=Rc::downgrade(&editor.scene.objects()[0].source().unwrap().document);assert_eq!(display.parts.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>(),8,"Identical pens retain one packed eight-byte entry");
    let mut gpu=renderer(&editor.scene);gpu.set_opacity(1.);let matrix=editor.camera.uniform();let before=pixels_at(&gpu,matrix);let uploads=gpu.stats()[0];editor.apply(Action::UnloadSources).unwrap();assert!(original.upgrade().is_none()&&document.upgrade().is_none(),"Prepared numeric pens must not pin original Mesh/Document ownership");assert!(editor.scene.objects()[0].geometry().is_none());assert!(Rc::ptr_eq(&display,&editor.scene.objects()[0].mesh));gpu.set_scene(&editor.scene,None);assert_eq!(pixels_at(&gpu,matrix),before,"Source unload must retain exact original pen pixels");assert_eq!(gpu.stats()[0],uploads);let keys=editor.reload_keys();assert!(editor.hydrate(keys.into_iter().map(|key|(key,bytes.clone())).collect()).unwrap());assert_eq!(archive(&editor.scene),source);gpu.set_scene(&editor.scene,None);assert_eq!(pixels_at(&gpu,matrix),before);assert_eq!(gpu.stats()[0],uploads);
    let mesh=panel(7.);let guid=mesh.guid().to_owned();let mut shared=crate::instance_specimen::message();shared.definitions.as_mut().unwrap().meshes=vec![mesh.to_proto()];for instance in &mut shared.objects.as_mut().unwrap().instances {instance.definition_guid=guid.clone();instance.xform=None;instance.flags=0;}shared.xforms=vec![proto::XformEntry{guid:"first".into(),xform:Some(Xform::translation(-1200.,0.,0.).to_proto())},proto::XformEntry{guid:"second".into(),xform:Some((&Xform::translation(1200.,0.,0.)*&Xform::scale_xyz(-0.8,1.2,1.4)).to_proto())}];let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&shared.encode_to_vec(),"",&Xform::identity(),0.).unwrap();assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));let initial=archive(&scene);let ids=scene.objects().iter().map(|r|r.id).collect::<Vec<_>>();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let mut c=camera();c.distance=6000.;c.radius=3000.;let matrix=c.uniform();let before=pixels_at(&gpu,matrix);let uploads=gpu.stats()[0];let placement=scene.placement(ids[0]).unwrap();scene.place(ids[0],&Xform::translation(100.,150.,0.)*&placement).unwrap();gpu.set_scene(&scene,None);assert_ne!(pixels_at(&gpu,matrix),before);scene.place(ids[0],placement).unwrap();gpu.set_scene(&scene,None);assert_eq!(pixels_at(&gpu,matrix),before);assert_eq!(gpu.stats()[0],uploads);assert_eq!(archive(&scene),initial);export("shared-placed-pens",&scene,Some(&before));
}

#[test]
fn printed_and_dense_originals_keep_faces_without_decoration() {
    let scene=mesh_scene(panel(0.));let initial=archive(&scene);let owner=scene.objects()[0].id;assert!(scene.objects()[0].mesh.parts.edges.is_empty());assert!(Sources::new(&scene).unwrap().mesh_ink.is_empty());let mut gpu=renderer(&scene);gpu.set_opacity(1.);let matrix=camera().uniform();let pixels=pixels_at(&gpu,matrix);assert_eq!(at(&pixels,matrix,[0.,0.,0.]),[51,102,153,255]);assert_eq!(pick(&mut gpu,&scene,matrix,[0.,0.,0.],Mode::Face),Some(Target{owner,part:Part::Face(0)}));assert_eq!(archive(&scene),initial);export("mesh-printed-fill",&scene,Some(&pixels));
    let n=317;let mut points=Vec::new();for y in 0..=n {for x in 0..=n {points.push(Point::new(-900.+1800.*x as f64/n as f64,-650.+1300.*y as f64/n as f64,0.));}}let mut faces=Vec::new();for y in 0..n {for x in 0..n {let a=y*(n+1)+x;faces.extend([vec![a,a+1,a+n+2],vec![a,a+n+2,a+n+1]]);}}let mut dense=Mesh::from_vertices_and_faces(points,faces);dense.set_objectcolor(Color::new(0.2,0.4,0.6,1.));let count=dense.number_of_faces();assert!(count>200_000);let scene=mesh_scene(dense);assert_eq!(scene.objects()[0].mesh.indices().len()/3,count);assert!(scene.objects()[0].mesh.parts.edges.is_empty());assert!(scene.objects()[0].mesh.parts.pens.is_empty());gpu.set_scene(&scene,None);let pixels=pixels_at(&gpu,matrix);assert_eq!(at(&pixels,matrix,[0.,0.,0.]),[51,102,153,255]);assert_eq!(pick(&mut gpu,&scene,matrix,[100.,100.,0.],Mode::Object),Some(Target{owner:scene.objects()[0].id,part:Part::Object}));
}
