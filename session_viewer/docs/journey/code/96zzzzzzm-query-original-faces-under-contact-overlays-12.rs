use crate::{editor::{Editor,Action},scene::Scene,renderer::Renderer,pick_sources::{Mode,Part,Target,Sources},camera::Projection};
use session_rust::{Point,Mesh,Element,Session,Xform,proto,element::{ElementFeature,ElementGeometry}};
use prost::Message;

fn export_source(name:&str,editor:&Editor){
    if let Some(root)=std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR"){let root=std::path::Path::new(&root);std::fs::create_dir_all(root).unwrap();let archive=crate::scene_archive::save(&editor.scene).unwrap();let bundle=crate::archive::Bundle::decode(&archive).unwrap();assert_eq!(bundle.documents.len(),1);std::fs::write(root.join(format!("{name}.session")),archive).unwrap();std::fs::write(root.join(format!("{name}.pb")),&bundle.documents[0]).unwrap();}
}
fn export_picture(name:&str,pixels:&[u8]){
    if let Some(root)=std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR"){use std::io::Write;let mut file=std::fs::File::create(std::path::Path::new(&root).join(format!("{name}.ppm"))).unwrap();write!(file,"P6\n320 240\n255\n").unwrap();for p in pixels.chunks_exact(4){file.write_all(&p[..3]).unwrap();}}
}
fn panel(sparse:bool,half:f64)->Mesh {
    let mut mesh=Mesh::from_vertices_and_faces(vec![Point::new(-half,-650.,0.),Point::new(0.,-650.,0.),Point::new(half,-650.,0.),Point::new(-half,650.,0.),Point::new(0.,650.,0.),Point::new(half,650.,0.)],vec![vec![0,1,4,3],vec![1,2,5,4]]);
    if sparse {mesh.remove_face(0);mesh.remove_face(1);assert_eq!(mesh.add_face(vec![0,1,4,3],Some(17)),Some(17));assert_eq!(mesh.add_face(vec![1,2,5,4],Some(93)),Some(93));}
    mesh.set_objectcolor(session_rust::Color::new(0.2,0.4,0.6,1.));mesh
}
fn imported(geometry:ElementGeometry,z:f64)->Editor {
    let mut element=Element::new("Original faces beneath readonly contact");element.set_element_geometry(geometry);
    let mut outline=super::contact_feature_tests::polygon();for p in outline.coords.chunks_exact_mut(3){p[2]=z;}
    element.add_feature(ElementFeature::new("contact",0,vec![outline],"readonly contact"));let mut source=Session::new("Original face query");source.add_element(element,None);
    let mut editor=Editor::default();editor.scene=Scene::empty();crate::scene_document::import(&mut editor.scene,&source.to_proto().encode_to_vec(),"original-face-query",&Xform::identity(),0.).unwrap();editor.camera=crate::camera_defaults_tests::top_fixture();editor.camera.distance=3000.;editor.camera.radius=1500.;editor.camera.aspect=4./3.;editor
}
fn point(editor:&Editor,local:[f64;3])->[f64;3]{let p=editor.scene.objects()[0].model.transform_point(&Point::new(local[0],local[1],local[2]));[p[0],p[1],p[2]]}
fn pick(r:&mut Renderer,scene:&Scene,matrix:[f32;16],p:[f64;3],mode:Mode)->Option<Target>{r.set_pick_mode(mode);r.request_pick(super::contact_feature_tests::project(matrix,p),matrix).unwrap();crate::cap_gpu_tests::wait(r);r.take_pick(matrix).unwrap().unwrap().and_then(|raw|r.resolve_pick(scene,raw))}
fn archive(scene:&Scene)->(Vec<proto::Session>,Vec<proto::Mesh>,Vec<proto::BRep>,serde_json::Value){
    let bundle=crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();let mut docs=bundle.documents.iter().map(|b|proto::Session::decode(b.as_slice()).unwrap()).collect::<Vec<_>>();let(mut meshes,mut breps)=(Vec::new(),Vec::new());
    for doc in &mut docs {for objects in doc.objects.iter_mut().chain(doc.definitions.iter_mut()){for element in &mut objects.elements {match element.geometry_type.as_str(){"Mesh"=>{meshes.push(proto::Mesh::decode(element.geometry_data.as_slice()).unwrap());element.geometry_data.clear();},"BRep"=>{breps.push(proto::BRep::decode(element.geometry_data.as_slice()).unwrap());element.geometry_data.clear();},_=>{}}}}}
    (docs,meshes,breps,serde_json::to_value(&bundle.metadata).unwrap())
}
fn pixel<'a>(image:&'a[u8],matrix:[f32;16],point:[f64;3])->&'a[u8]{let p=super::contact_feature_tests::project(matrix,point);let x=((p[0]+1.)*160.).floor()as usize;let y=((1.-p[1])*120.).floor()as usize;crate::cap_gpu_tests::at(image,x,y)}

#[test]
fn source_faces_under_contacts_retain_sparse_keys_and_original_pixels(){
    for mirror in [false,true]{let mut editor=imported(ElementGeometry::Mesh(panel(true,900.)),0.);let owner=editor.scene.objects()[0].id;if !mirror{export_source("sparse",&editor);}
        editor.select_objects([owner],false);editor.apply(Action::Transformation{kind:crate::transform_draft::Kind::Rotate,delta:&Xform::rotation_x(30.,true)*&Xform::scale_xyz(if mirror{-1.2}else{1.},0.8,1.4)}).unwrap();editor.select_objects([],false);
        let original=archive(&editor.scene);let sources=Sources::new(&editor.scene).unwrap();assert_eq!(sources.faces.iter().map(|t|t.part).collect::<Vec<_>>(),[Part::Face(17),Part::Face(93)]);assert!(sources.faces.iter().all(|t|t.owner==owner));assert_eq!(sources.mesh_bases.len(),2);
        let mut r=crate::cap_gpu_tests::renderer(&editor.scene);r.set_opacity(1.);let uploads=r.stats()[0];
        for projection in [Projection::Orthographic,Projection::Perspective]{editor.camera.projection=projection;let matrix=editor.camera.uniform();for density in [1.,2.]{r.set_density(density);let before=crate::cap_gpu_tests::pixels_at(&r,matrix);if !mirror&&projection==Projection::Orthographic&&density==1.{export_picture("sparse-native-original",&before);}
            for(p,key)in[([-350.,-150.,0.],17),([350.,-150.,0.],93)]{let p=point(&editor,p);assert_eq!(pick(&mut r,&editor.scene,matrix,p,Mode::Face),Some(Target{owner,part:Part::Face(key)}));assert_eq!(pick(&mut r,&editor.scene,matrix,p,Mode::Object),Some(Target{owner,part:Part::Object}));}
            let target=Target{owner,part:Part::Face(17)};r.set_highlight(&editor.scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&r,matrix);if !mirror&&projection==Projection::Orthographic&&density==1.{export_picture("sparse-native-left-face",&selected);}let yellow=pixel(&selected,matrix,point(&editor,[-750.,-450.,0.]));assert!(yellow[0]>240&&yellow[1]>240&&yellow[2]<20,"Actual original face is highlighted: {yellow:?}");assert_eq!(pixel(&selected,matrix,point(&editor,[-350.,-150.,0.])),pixel(&before,matrix,point(&editor,[-350.,-150.,0.])),"Readonly red contact remains above the original face highlight");
            r.set_highlight(&editor.scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&r,matrix),before);r.use_uniform_surface_records();assert_eq!(pick(&mut r,&editor.scene,matrix,point(&editor,[-350.,-150.,0.]),Mode::Face),Some(target));assert_eq!(crate::cap_gpu_tests::pixels_at(&r,matrix),before);r.set_scene(&editor.scene,None);assert_eq!(r.stats()[0],uploads);assert_eq!(archive(&editor.scene),original);
        }}
    }
}

#[test]
fn readonly_contacts_cannot_fabricate_faces_or_remove_original_occlusion(){
    let editor=imported(ElementGeometry::Mesh(panel(true,300.)),0.);export_source("overhang",&editor);let owner=editor.scene.objects()[0].id;let matrix=editor.camera.uniform();let mut r=crate::cap_gpu_tests::renderer(&editor.scene);r.set_opacity(1.);crate::cap_gpu_tests::pixels_at(&r,matrix);
    assert_eq!(pick(&mut r,&editor.scene,matrix,[-500.,-150.,0.],Mode::Object),Some(Target{owner,part:Part::Object}));assert_eq!(pick(&mut r,&editor.scene,matrix,[-500.,-150.,0.],Mode::Face),None,"Readonly overhang cannot invent an original face");
    let cursor=super::contact_feature_tests::project(matrix,[-500.,-150.,0.]);let edge=super::contact_feature_tests::project(matrix,[-300.,-150.,0.]);assert!((cursor[0]-edge[0]).abs()*160.>crate::pick_window::Window::new(cursor,[320,240],1.).unwrap().radius as f32+1.,"The overhang sample is beyond the actual cursor pick radius");
    assert_eq!(pick(&mut r,&editor.scene,matrix,[-200.,-150.,0.],Mode::Face),Some(Target{owner,part:Part::Face(17)}));
    for parts in [false,true]{let mut scene=editor.scene.clone();let blocker=if parts{crate::prepared::PreparedMesh::new(Mesh::from_vertices_and_faces(vec![Point::new(-1200.,-1000.,20.),Point::new(500.,-1000.,20.),Point::new(500.,1000.,20.),Point::new(-1200.,1000.,20.)],vec![vec![0,1,2,3]])).unwrap()}else{let mut p=crate::prepared::PreparedMesh::triangle([[-1800.,-1500.,20.],[1000.,-1500.,20.],[-400.,2000.,20.]],[0.1,0.8,0.2]).unwrap();p.display.parts=Default::default();p};let blocker=scene.insert(blocker).unwrap();std::rc::Rc::make_mut(&mut scene.view).lock(&[blocker],true);r.set_scene(&scene,None);crate::cap_gpu_tests::pixels_at(&r,matrix);assert_eq!(pick(&mut r,&scene,matrix,[-200.,-150.,20.],Mode::Face),None,"Locked original geometry retains occlusion, including rows without source face metadata; parts={parts}");}
}

#[test]
fn shared_instance_faces_remain_original_without_geometry_uploads(){
    let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&Xform::identity(),0.).unwrap();let ids=scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();assert!(std::rc::Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));assert!(ids.iter().all(|id|!scene.forest.derived(*id)));
    let mut r=crate::cap_gpu_tests::renderer(&scene);let camera=crate::camera_defaults_tests::top_fixture();let matrix=camera.uniform();let uploads=r.stats()[0];crate::cap_gpu_tests::pixels_at(&r,matrix);
    for density in[1.,2.]{r.set_density(density);for(i,id)in ids.iter().enumerate(){assert_eq!(pick(&mut r,&scene,matrix,[if i==0{-0.6}else{0.6},0.,0.],Mode::Face),Some(Target{owner:*id,part:Part::Face(0)}));}r.use_uniform_surface_records();assert_eq!(pick(&mut r,&scene,matrix,[-0.6,0.,0.],Mode::Face),Some(Target{owner:ids[0],part:Part::Face(0)}));}assert_eq!(r.stats()[0],uploads);
}

#[test]
fn actual_brep_contact_queries_the_original_top_face(){
    let mut editor=imported(ElementGeometry::BRep(session_rust::BRep::create_box(1800.,1300.,100.)),50.);export_source("brep",&editor);let row=&editor.scene.objects()[0];let owner=row.id;assert!(row.cad.is_some());let mesh=&row.mesh;
    let slot=mesh.indices().chunks_exact(3).zip(&mesh.parts.triangles).find(|(tri,_)|tri.iter().all(|i|(mesh.vertices()[*i as usize][2]-50.).abs()<1e-4)).map(|(_,slot)|*slot as usize).expect("Actual original BRep top triangles");let key=mesh.parts.faces[slot];if let Some(root)=std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR"){std::fs::write(std::path::Path::new(&root).join("brep-face-key.json"),key.to_string()).unwrap();}let source=archive(&editor.scene);let mut r=crate::cap_gpu_tests::renderer(&editor.scene);r.set_opacity(1.);
    for projection in[Projection::Orthographic,Projection::Perspective]{editor.camera.projection=projection;let matrix=editor.camera.uniform();let before=crate::cap_gpu_tests::pixels_at(&r,matrix);for density in[1.,2.]{r.set_density(density);assert_eq!(pick(&mut r,&editor.scene,matrix,[-350.,-150.,50.],Mode::Face),Some(Target{owner,part:Part::Face(key)}));assert_eq!(pick(&mut r,&editor.scene,matrix,[-350.,-150.,50.],Mode::Object),Some(Target{owner,part:Part::Object}));}r.set_density(1.);assert_eq!(crate::cap_gpu_tests::pixels_at(&r,matrix),before);}assert_eq!(archive(&editor.scene),source);
}
