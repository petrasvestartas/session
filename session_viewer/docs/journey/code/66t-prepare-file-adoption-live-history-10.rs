    let reply = Reply::new(e.reload_keys(), Some(Intent::Save), Ok(vec![bytes]));
    let Some(Edit::Saved(saved)) = reply.complete(&mut e).unwrap() else { panic!("Saved reply") };
    let bundle=crate::archive::Bundle::decode(&saved).unwrap();
    let message = session_rust::proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
    assert_eq!(message.objects.unwrap().meshes[0].vertices, original);
    assert_eq!(e.scene.objects()[0].model.m, moved);
