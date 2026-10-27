 assert_eq!(scene.all_ids(),before);assert_eq!(prepare(&scene,&[255],true,None).err().unwrap(),"failed to decode Protobuf message: invalid varint");assert_eq!(scene.all_ids(),before);
