use crate::{renderer::Renderer,scene::Scene,editor::Action,camera::Projection};
use session_rust::{Xform,Point,Mesh};
fn scene(angle:f64,projection:Projection,mirror:bool)->(crate::editor::Editor,crate::scene::ObjectId){
    let(mut editor,_)=super::contact_feature_tests::imported(false);editor.camera.projection=projection;
    let owner=editor.scene.objects().iter().find(|r|!editor.scene.forest.derived(r.id)).unwrap().id;
    editor.select_objects([owner],false);let model=&Xform::rotation_x(angle,true)*&Xform::scale_xyz(if mirror{-1.3}else{1.},0.8,1.7);
    editor.apply(Action::Transformation{kind:crate::transform_draft::Kind::Rotate,delta:model.clone()}).unwrap();editor.select_objects([],false);
    assert!(editor.scene.objects().iter().all(|r|r.model.m==model.m),"The actual parent and contact must both follow the edit");
    for i in 0..20{let x=10000.+i as f64*20.;let mesh=Mesh::from_vertices_and_faces(vec![Point::new(x,0.,-20.),Point::new(x+10.,0.,-20.),Point::new(x+10.,10.,-20.),Point::new(x,10.,-20.)],vec![vec![0,1,2,3]]);editor.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();}
    (editor,owner)
}
fn renderer(scene:&Scene,bounded:bool)->Renderer{
    let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let mut descriptor=wgpu::DeviceDescriptor::default();if bounded{descriptor.required_limits.max_storage_buffer_binding_size=2048;}
    let(device,queue)=pollster::block_on(adapter.request_device(&descriptor)).unwrap();device.on_uncaptured_error(std::sync::Arc::new(|e|panic!("Actual contact perimeter: {e}")));let mut renderer=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8Unorm,scene);renderer.resize(crate::viewport::Viewport{width:320,height:240});renderer.set_opacity(1.);renderer
}
#[test]
fn actual_contact_perimeters_survive_bounded_projection_fallback(){
    for projection in [Projection::Orthographic,Projection::Perspective]{for angle in [0.,60.,75.,85.]{for mirror in [false,true]{
        let(editor,owner)=scene(angle,projection,mirror);let matrix=editor.camera.uniform();let(mut full,mut bounded)=(renderer(&editor.scene,false),renderer(&editor.scene,true));
        for density in [1.,2.]{full.set_density(density);bounded.set_density(density);let a=crate::cap_gpu_tests::pixels_at(&full,matrix);let b=crate::cap_gpu_tests::pixels_at(&bounded,matrix);
            assert!(full.tile_usage()[0]>96);assert_eq!(&bounded.tile_usage()[..2],&[96,16],"Real storage limit exercises bounded fallback");
            let corners=[[-550.,-350.,0.],[550.,-350.,0.],[550.,350.,0.],[-550.,350.,0.]].map(|p| {
                let p=editor.scene.placement(owner).unwrap().transform_point(&Point::new(p[0],p[1],p[2]));
                let at=super::contact_feature_tests::project(matrix,[p[0],p[1],p[2]]);[(at[0]+1.)*160.,(1.-at[1])*120.]
            });
            let min=std::array::from_fn::<_,2,_>(|i|(corners.iter().map(|p|p[i]).fold(f32::INFINITY,f32::min)-2.*density).floor().max(0.) as usize);
            let max=std::array::from_fn::<_,2,_>(|i|(corners.iter().map(|p|p[i]).fold(f32::NEG_INFINITY,f32::max)+2.*density).ceil().min([320.,240.][i]) as usize);
            let differences=(min[1]..max[1]).flat_map(|y|(min[0]..max[0]).map(move|x|(x,y))).filter(|(x,y)|{let at=(y*320+x)*4;a[at..at+4]!=b[at..at+4]}).collect::<Vec<_>>();
            assert!(differences.is_empty(),"Every pixel of the contact, complete perimeter, notch and surrounding band matches exactly: {projection:?}, {angle}, mirror={mirror}, density={density}, different={differences:?}");
            let local=Point::new(-350.,-150.,0.);let p=editor.scene.placement(owner).unwrap().transform_point(&local);let at=super::contact_feature_tests::project(matrix,[p[0],p[1],p[2]]);
            for r in [&mut full,&mut bounded]{r.request_pick(at,matrix).unwrap();crate::cap_gpu_tests::wait(r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());}
        }
    }}}
}
#[test]
fn contact_metadata_follows_original_parent_placement(){
    for mirror in [false,true]{let(editor,owner)=scene(75.,Projection::Perspective,mirror);let sources=crate::pick_sources::Sources::new(&editor.scene).unwrap();assert_eq!(sources.mesh_metadata.len(),sources.mesh_ink.len()*2);let contact=editor.scene.objects().iter().find(|r|editor.scene.forest.derived(r.id)).unwrap();let normal=contact.mesh.normals()[0];let m=crate::normals::matrix(&contact.model).unwrap();let expected=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*normal[j]).sum()));let contacts=sources.mesh_metadata.chunks_exact(2).zip(&sources.mesh_ids).filter(|(m,_)|m[1]!=crate::packed_vertex::NO_NORMAL).collect::<Vec<_>>();assert_eq!(contacts.len(),9);assert!(contacts.iter().all(|(m,id)|*m==[0,expected]&&id[0]==owner.value()));
        let archive=crate::scene_archive::save(&editor.scene).unwrap();let mut restored=Scene::empty();crate::scene_document::open(&mut restored,&archive).unwrap();let again=crate::pick_sources::Sources::new(&restored).unwrap();assert_eq!(again.mesh_metadata,sources.mesh_metadata);assert!(restored.paths().is_empty());
    }
}
