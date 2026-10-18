use crate::{scene::Scene,editor::{Editor,Action},cap_gpu_tests::{renderer,pixels_at,wait},camera::Projection};
use session_rust::{Point,Polyline,Mesh,Element,Session,Geometry,Xform};
use session_rust::element::ElementFeature;
use prost::Message;

fn polygon()->Polyline {
    Polyline::new([[-550.,-350.,0.],[0.,-350.,0.],[550.,-350.,0.],[550.,350.,0.],[150.,350.,0.],[150.,0.,0.],[-150.,0.,0.],[-150.,350.,0.],[-550.,350.,0.],[-550.,-350.,0.]].into_iter().map(|p|Point::new(p[0],p[1],p[2])).collect())
}
fn panel()->Mesh {
    let mut mesh=Mesh::from_vertices_and_faces(vec![Point::new(-900.,-650.,0.),Point::new(900.,-650.,0.),Point::new(900.,650.,0.),Point::new(-900.,650.,0.)],vec![vec![0,1,2,3]]);
    mesh.set_objectcolor(session_rust::Color::new(0.2,0.4,0.6,1.));mesh
}
fn original(reverse:bool)->Session {
    let mut element=Element::new("Original contact panel");element.set_geometry(panel());
    let mut outline=polygon();if reverse {outline.coords=outline.coords.chunks_exact(3).rev().flatten().copied().collect();}
    element.add_feature(ElementFeature::new("contact",0,vec![outline],"concave contact"));
    let mut session=Session::new("Original contact feature");session.add_element(element,None);session
}
fn imported(reverse:bool)->(Editor,session_rust::proto::Session) {
    let source=original(reverse).to_proto();let mut editor=Editor::default();editor.scene=Scene::empty();
    crate::scene_document::import(&mut editor.scene,&source.encode_to_vec(),"original-contact",&Xform::identity(),0.).unwrap();
    editor.camera=crate::camera_defaults_tests::top_fixture();editor.camera.distance=3000.;editor.camera.radius=1500.;editor.camera.aspect=4./3.;
    (editor,source)
}
fn retained(editor:&Editor)->session_rust::proto::Element {
    let Geometry::Element(element)=editor.scene.objects()[0].original.as_ref().unwrap()else{panic!("Original Element retained")};element.to_proto()
}
fn same_element(a:&session_rust::proto::Element,b:&session_rust::proto::Element) {
    assert_eq!(a.geometry_type,b.geometry_type);assert_eq!(a.geometry_type,"Mesh");
    assert_eq!(session_rust::proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),session_rust::proto::Mesh::decode(b.geometry_data.as_slice()).unwrap(),"Every embedded original mesh field is retained regardless of protobuf map order");
    let(mut a,mut b)=(a.clone(),b.clone());a.geometry_data.clear();b.geometry_data.clear();assert_eq!(a,b,"Every original Element metadata/feature field is retained");
}
fn project(matrix:[f32;16],p:[f64;3])->[f32;2] {
    let p=p.map(|v|v as f32);let c:[f32;4]=std::array::from_fn(|row|p.iter().enumerate().fold(matrix[12+row],|s,(i,v)|s+matrix[i*4+row]*v));[c[0]/c[3],c[1]/c[3]]
}
fn pixel<'a>(image:&'a[u8],matrix:[f32;16],p:[f64;3])->&'a[u8] {
    let p=project(matrix,p);let x=((p[0]+1.)*160.).floor()as usize;let y=((1.-p[1])*120.).floor()as usize;assert!(x<320&&y<240);&image[(y*320+x)*4..(y*320+x+1)*4]
}
fn depth(r:&super::Renderer,points:&[[u32;2]])->Vec<f32> {
    use wgpu::util::DeviceExt;
    let coords=points.iter().enumerate().map(|(i,p)|format!("values[{i}u]=textureLoad(depth,vec2<i32>({},{}),0);",p[0],p[1])).collect::<String>();
    let source=format!("@group(0) @binding(0) var depth:texture_depth_2d; @group(0) @binding(1) var<storage,read_write> values:array<f32>; @compute @workgroup_size(1) fn main(){{{coords}}}");
    let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("Actual contact color-pass depth"),source:wgpu::ShaderSource::Wgsl(source.into())});
    let pipeline=r.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:None,layout:None,module:&module,entry_point:Some("main"),compilation_options:Default::default(),cache:None});
    let output=r.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:None,contents:&vec![0u8;points.len()*4],usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC});
    let read=r.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:output.size(),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
    let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor{label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&r.depth)},wgpu::BindGroupEntry{binding:1,resource:output.as_entire_binding()}]});
    let mut encoder=r.device.create_command_encoder(&Default::default());{let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups(1,1,1);}encoder.copy_buffer_to_buffer(&output,0,&read,0,output.size());r.queue.submit([encoder.finish()]);
    let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let data=read.slice(..).get_mapped_range();let result=data.chunks_exact(4).map(|b|f32::from_ne_bytes(b.try_into().unwrap())).collect();drop(data);read.unmap();result
}

#[test]
fn original_contact_sources_and_complete_perimeters_are_retained() {
    for reverse in [false,true] {
        let(mut editor,source)=imported(reverse);assert_eq!(editor.scene.objects().len(),2);assert!(editor.scene.paths().is_empty(),"One derived mesh reuses its own border, without a duplicate path");
        let expected=&source.objects.as_ref().unwrap().elements[0];same_element(&retained(&editor),expected);let owner=editor.scene.objects()[0].id;let fill=&editor.scene.objects()[1];
        assert!(editor.scene.forest.derived(fill.id));assert_eq!(crate::feature_owner::display_owner(&editor.scene,fill.id),Some(owner));
        assert_eq!(fill.mesh.parts.faces.len(),1);assert_eq!(fill.mesh.parts.edges.len(),9,"The original nine-edge concave ring has no triangulation diagonals");
        assert!(fill.mesh.indices().len()>9);assert!(fill.mesh.normals().iter().all(|n|n[2].abs()>0.99&&n[0]==0.&&n[1]==0.));assert!(fill.mesh.alphas().iter().all(|v|(*v-1./255.).abs()<1e-7));
        let originals=crate::pick_sources::Sources::new(&editor.scene).unwrap();assert_eq!(originals.mesh_ink.len(),13);assert!(originals.mesh_ids.iter().all(|p|p[0]==owner.value()));
        let archive=crate::scene_archive::save(&editor.scene).unwrap();let bundle=crate::archive::Bundle::decode(&archive).unwrap();assert_eq!(bundle.documents.len(),1);let saved=session_rust::proto::Session::decode(bundle.documents[0].as_slice()).unwrap();let saved=saved.objects.unwrap();assert!(saved.meshes.is_empty(),"Derived contact display is excluded from the original archive");assert_eq!(saved.elements.len(),1);assert_eq!(saved.elements[0].features,expected.features);same_element(&saved.elements[0],expected);
        editor.apply(Action::ElementInteractions(Some(false))).unwrap();assert!(!editor.scene.visible(editor.scene.objects()[1].id));editor.apply(Action::ElementInteractions(Some(true))).unwrap();assert!(editor.scene.visible(editor.scene.objects()[1].id));same_element(&retained(&editor),expected);
    }
    for outline in [Polyline::new(vec![Point::new(0.,0.,0.),Point::new(1.,0.,0.),Point::new(2.,0.,0.)]),Polyline::new(vec![Point::new(1.,2.,3.),Point::new(1.,2.,3.)])] {assert!(crate::feature_display::contact_fill(&outline).unwrap().is_none());}
    if let Ok(root)=std::env::var("COURSE_CONTACT_EXPORT") {std::fs::create_dir_all(&root).unwrap();let(editor,source)=imported(false);std::fs::write(format!("{root}/original-contact.pb"),source.encode_to_vec()).unwrap();std::fs::write(format!("{root}/original-contact.session"),crate::scene_archive::save(&editor.scene).unwrap()).unwrap();}
}

#[test]
fn actual_contact_fill_and_original_owner_survive_presentation() {
    let(mut editor,_)=imported(false);let source=retained(&editor);let owner=editor.scene.objects()[0].id;let mut r=renderer(&editor.scene);r.set_opacity(1.);let uploads=r.stats()[0];
    for projection in [Projection::Orthographic,Projection::Perspective] {editor.camera.projection=projection;for density in [1.,2.] {r.set_density(density);let matrix=editor.camera.uniform();let before=pixels_at(&r,matrix);let inside=pixel(&before,matrix,[-350.,-150.,0.]);assert!((225..=232).contains(&inside[0])&&(23..=28).contains(&inside[1])&&(23..=28).contains(&inside[2]),"Actual unlit contact fill: {inside:?}");assert_eq!(pixel(&before,matrix,[0.,200.,0.]),&[51,102,153,255],"Concave notch retains host pixels");
        r.request_pick(project(matrix,[-350.,-150.,0.]),matrix).unwrap();wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"Contact is picked as its original Element, never a generated mesh owner");
        editor.apply(Action::Picked(Some(owner))).unwrap();r.select_objects(&editor.scene,&editor.highlighted_objects()).unwrap();let selected=pixels_at(&r,matrix);let p=pixel(&selected,matrix,[-350.,-150.,0.]);assert!(p[0]>240&&p[1]>240&&p[2]<20,"Actual original selection fills the contact yellow: {p:?}");
        editor.apply(Action::Picked(None)).unwrap();r.select_objects(&editor.scene,&editor.highlighted_objects()).unwrap();assert_eq!(pixels_at(&r,matrix),before);r.set_lighting(true,true);assert_eq!(pixel(&pixels_at(&r,matrix),matrix,[-350.,-150.,0.]),inside,"Contact remains unlit and independent of backface color");r.set_lighting(false,false);
        r.use_uniform_surface_records();assert_eq!(pixels_at(&r,matrix),before);r.set_scene(&editor.scene,None);assert_eq!(pixels_at(&r,matrix),before);assert_eq!(r.stats()[0],uploads);same_element(&retained(&editor),&source);
    }}
    editor.scene.clear();r.set_scene(&editor.scene,None);pixels_at(&r,editor.camera.uniform());assert_eq!(r.usage()[0],0);
}

#[test]
fn actual_contact_depth_is_nearer_without_crossing_an_occluder() {
    let(mut editor,_)=imported(false);let source=retained(&editor);let mut r=renderer(&editor.scene);r.set_opacity(1.);
    for projection in [Projection::Orthographic,Projection::Perspective] {editor.camera.projection=projection;let matrix=editor.camera.uniform();pixels_at(&r,matrix);
        let points=[[-350.,-150.,0.],[0.,200.,0.]].map(|p| {let p=project(matrix,p);[((p[0]+1.)*160.).floor()as u32,((1.-p[1])*120.).floor()as u32]});let z=depth(&r,&points);
        assert!(z[0]>z[1],"Actual contact color vertices pull toward the eye while the host notch stays on the source plane: {z:?}");assert!(z[0]<=z[1]*1.0001+1e-12,"Layer stays bounded: {z:?}");
        let mut blocked=editor.scene.clone();let mut roof=Mesh::from_vertices_and_faces(vec![Point::new(-600.,-400.,20.),Point::new(-100.,-400.,20.),Point::new(-100.,100.,20.),Point::new(-600.,100.,20.)],vec![vec![0,1,2,3]]);roof.set_objectcolor(session_rust::Color::new(0.1,0.8,0.2,1.));let blocker=blocked.insert(crate::prepared::PreparedMesh::new(roof).unwrap()).unwrap();r.set_scene(&blocked,None);let image=pixels_at(&r,matrix);let p=pixel(&image,matrix,[-350.,-150.,20.]);assert!(p[1]>190&&p[0]<35,"Real nearer geometry still occludes the bounded contact: {p:?}");r.request_pick(project(matrix,[-350.,-150.,20.]),matrix).unwrap();wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],blocker.value());
        r.set_scene(&editor.scene,None);pixels_at(&r,matrix);same_element(&retained(&editor),&source);
    }
}
