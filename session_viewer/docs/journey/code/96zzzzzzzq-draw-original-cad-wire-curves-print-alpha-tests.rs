    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn shared_print_instances_compose_vertex_and_instance_alpha_without_changing_cap_tokens() {
    use session_rust::proto;
    let mut mesh=source(vec![0.]);mesh.set_objectcolor(session_rust::Color::new(1.,1.,1.,0.5));
    mesh.set_guid("6d0aaee0-0430-4000-8000-000000000711".into());
    let definition=mesh.to_proto();
    let colors=[[1.,0.,0.,0.25],[0.,0.,1.,0.75]];
    let document=proto::Session {
        definitions:Some(proto::Objects {meshes:vec![definition.clone()],..Default::default()}),
        objects:Some(proto::Objects {instances:colors.iter().enumerate().map(|(index,color)|proto::InstanceRef {
            guid:format!("printed-instance-{index}"),definition_guid:definition.guid.clone(),flags:session_rust::InstanceRef::FLAG_COLOR,
            color:Some(proto::Color {r:color[0],g:color[1],b:color[2],a:color[3],..Default::default()}),..Default::default()
        }).collect(),..Default::default()}),..Default::default()
    };
    let mut scene=crate::scene::Scene::empty();
    crate::scene_document::import(&mut scene,&prost::Message::encode_to_vec(&document),"",&session_rust::Xform::identity(),0.).unwrap();
    assert_eq!(scene.objects().len(),2);
    let first=scene.objects()[0].id;let last=scene.objects()[1].id;
    assert_eq!(scene.face_rgba(first),Some(colors[0]));assert_eq!(scene.face_rgba(last),Some(colors[1]));
    assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));
    let vertex_alpha=scene.objects()[0].mesh.alphas()[0];
    assert_eq!(vertex_alpha,1.,"The production kernel renders object-colour vertices opaque while retaining source alpha");
    let empty=crate::scene::Scene::empty();let mut gpu=renderer(&empty);
    gpu.set_grid(None).unwrap();gpu.set_lighting(false,false);gpu.set_outlines(false);
    let object=&scene.objects()[0];
    let mut row=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&crate::gpu_layout::uniform(&gpu.device,crate::object_record::BYTES as u64),object,false);
    let alpha=|row:&crate::gpu_mesh::GpuMesh|f32::from_ne_bytes(row.packed()[crate::object_record::PRINT_ALPHA..crate::object_record::BYTES].try_into().unwrap());
    assert_eq!(alpha(&row),1.);
    row.set_caps(&gpu.queue,crate::solid_orientation::CLOSED,0x1234);
    row.set_ambient_radius(&gpu.queue,0.125);
    row.update(&gpu.queue,object,crate::gpu_mesh::Style {source_faces:true,selected:false,owner:first.value(),colour:Some(colors[0])});
    assert_eq!(alpha(&row),0.25);
    assert_eq!(u32::from_ne_bytes(row.packed()[76..80].try_into().unwrap()),0x1234);
    assert_eq!(f32::from_ne_bytes(row.packed()[crate::object_record::RANGE+8..crate::object_record::PRINT_ALPHA].try_into().unwrap()),0.125);
    for samples in [1,4] {
        gpu.set_samples(samples);gpu.set_scene(&empty,None);let background=pixels(&gpu);
        gpu.set_scene(&scene,None);
        for selected in [false,true] {
            gpu.select_objects(&scene,if selected{std::slice::from_ref(&first)}else{&[]}).unwrap();
            let image=pixels(&gpu);let actual=at(&image,160,120);
            let mut expected=at(&background,160,120)[..3].iter().map(|v|*v as f32/255.).collect::<Vec<_>>();
            for (index,color) in colors.iter().enumerate() {
                let rgb=if selected&&index==0{crate::colour::selection()}else{*color};
                let opacity=vertex_alpha*color[3];
                for channel in 0..3 {expected[channel]=rgb[channel]*opacity+expected[channel]*(1.-opacity);}
            }
            for channel in 0..3 {assert!((actual[channel] as f32-expected[channel]*255.).abs()<=2.,"sample {samples}, selected {selected}, actual {actual:?}, expected {expected:?}");}
            let uploads=gpu.stats()[0];assert_eq!(pixels(&gpu),image);assert_eq!(gpu.stats()[0],uploads);
            gpu.request_pick([0.,0.],matrix()).unwrap();wait(&gpu);
            assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().map(|pixel|pixel[0]),Some(last.value()));
            assert_eq!(gpu.tile_usage()[3],0);
        }
    }
    for object in scene.objects(){assert_eq!(object.geometry().unwrap().to_proto(),definition);}
}
