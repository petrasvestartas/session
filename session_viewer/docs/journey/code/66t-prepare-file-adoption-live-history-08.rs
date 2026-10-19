        assert_eq!(Intent::Move { id, offset: [0.25, 0.0, 0.0] }.replay(&mut e).unwrap(), Reply::Changed(Change::Scene));
        let Reply::Saved(bytes) = Intent::Save.replay(&mut e).unwrap() else { panic!("Save reply") };
        let bundle=crate::archive::Bundle::decode(&bytes).unwrap();
        let saved = session_rust::proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        assert_eq!(saved.objects.unwrap().meshes[0].vertices, original.vertices);
        e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects()[0].model.m, model);
