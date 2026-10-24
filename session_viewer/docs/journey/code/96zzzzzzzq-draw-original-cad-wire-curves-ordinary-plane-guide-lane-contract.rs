fn ordinary_plane_uses_exact_compact_frame_and_keeps_clipping_plane_separate() {
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    let mut source = Plane::xy_plane();
    source.width = 7.0;
    let source = Rc::new(source);
    let _ = source.guid();
    let original = source.to_proto();
    let id = crate::plane_original::insert(&mut scene, source.clone()).unwrap();
    let anchor = [1_000_000_000.0, -1_000_000_000.0, 0.0];
    scene.place(id, Xform::translation(anchor[0] + 0.125, anchor[1], -500.0)).unwrap();
    let clip = crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy, &[[0.0, 0.0, -10.0]], 100.0).unwrap();
    let clip=Rc::new(clip);let _=clip.guid();let original_clip=clip.to_proto();
    let clipping = crate::plane_original::insert(&mut scene, clip.clone()).unwrap();
    assert!(scene.objects()[0].mesh.parts.edges.is_empty());
    assert_eq!(scene.objects()[1].mesh.parts.edges.len(), 7);
    assert_eq!(crate::clipping_rows::owners(&scene), [clipping]);
    let sources = crate::pick_sources::Sources::anchored(&scene, anchor).unwrap();
    match joined::rows(&scene,&sources,anchor,&[id],256) {Err(e)=>assert_eq!(e,"Production joined rows exceed the storage limit"),Ok(_)=>panic!("Seven visible guide ribbons must respect the 256-byte capacity")}
    let rows = joined::rows(&scene, &sources, anchor, &[id], 1024).unwrap();
    assert_eq!(rows.planes.len(), 48);
    assert_eq!(rows.objects.len(), 192);
    assert!(rows.heads.is_empty());assert_eq!(rows.segments.len(),7*52);
    assert_eq!(rows.owners,vec![clipping.value();7]);assert_eq!(rows.ids,vec![[clipping.value(),crate::sub_id::OBJECT_INK];7]);assert_eq!(rows.edges,vec![u32::MAX;7]);assert_eq!(rows.mesh_from,7);assert_eq!(rows.runs[1],[0..42]);
    assert_eq!(u32::from_ne_bytes(rows.objects[176..180].try_into().unwrap()),4096);
    for segment in rows.segments.chunks_exact(52) {assert_eq!(u32::from_ne_bytes(segment[28..32].try_into().unwrap()),1);for offset in[36,40,44]{assert_eq!(u32::from_ne_bytes(segment[offset..offset+4].try_into().unwrap()),u32::MAX);}}
    assert!(sources.mesh_legacy.iter().all(|kind|*kind==0),"Ordinary Plane has no legacy edges; guide query rows draw only in the production Line lane");
    assert_eq!(rows.object_ids, [id,clipping]);
    assert_eq!(rows.pick_owners, [id.value(),clipping.value()]);
    let words: Vec<u32> = rows.planes.chunks_exact(4).map(|b|u32::from_ne_bytes(b.try_into().unwrap())).collect();
    assert_eq!(words[3], 0);
    assert_eq!(words[4..8], [1.0f32.to_bits(), 0, 0, (-3.5f32).to_bits()]);
    assert_eq!(f32::from_ne_bytes(rows.translations[..4].try_into().unwrap()), 0.125);
    assert_eq!(u32::from_ne_bytes(rows.objects[80..84].try_into().unwrap()), 1);
    let mut output = plane::PlaneRows::default();
    let walked = plane::walk_plane(&mut output, &source, 13);
    assert_eq!(walked.flags, 0);
    assert_eq!(output.rows[0].instance_id, 13);
    assert_eq!(walked.bounds.min_point().to_f32(), [-100.0, -100.0, 0.0]);
    assert_eq!(walked.bounds.max_point().to_f32(), [100.0, 100.0, 100.0]);
    assert_eq!(source.to_proto(), original);
    assert_eq!(scene.objects()[0].model.m,Xform::translation(anchor[0]+0.125,anchor[1],-500.).m);
    assert_eq!(clip.to_proto(),original_clip);assert!(matches!(&scene.objects()[1].original,Some(session_rust::Geometry::Plane(actual))if Rc::ptr_eq(actual,&clip)));
}
