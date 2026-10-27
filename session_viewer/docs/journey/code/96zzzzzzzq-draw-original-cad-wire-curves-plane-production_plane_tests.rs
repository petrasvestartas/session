use crate::{production_joined as joined, production_plane as plane, scene::Scene};
use session_rust::{Color, Plane, Xform};
use std::rc::Rc;
use wgpu::naga;

#[test]
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
    let clipping = crate::plane_original::insert(&mut scene, Rc::new(clip)).unwrap();
    assert!(scene.objects()[0].mesh.parts.edges.is_empty());
    assert_eq!(scene.objects()[1].mesh.parts.edges.len(), 7);
    assert_eq!(crate::clipping_rows::owners(&scene), [clipping]);
    let sources = crate::pick_sources::Sources::anchored(&scene, anchor).unwrap();
    let rows = joined::rows(&scene, &sources, anchor, &[id], 256).unwrap();
    assert_eq!(rows.planes.len(), 48);
    assert_eq!(rows.objects.len(), 96);
    assert!(rows.segments.is_empty() && rows.heads.is_empty());
    assert_eq!(rows.object_ids, [id]);
    assert_eq!(rows.pick_owners, [id.value()]);
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
}

#[test]
fn plane_complete_shader_and_row_layout_keep_the_original_algorithms() {
    use sha2::{Digest, Sha256};
    use std::mem::{offset_of, size_of};
    let provenance: serde_json::Value = serde_json::from_str(plane::PROVENANCE).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(plane::SOURCE.as_bytes())), provenance["compiled_sha256"].as_str().unwrap());
    let adapter = &provenance["ownership_adapter"];
    let replacement = adapter["replacement"].as_str().unwrap();
    assert_eq!(plane::SOURCE.matches(replacement).count(), 1);
    let pristine = plane::SOURCE.strip_suffix(adapter["declaration"].as_str().unwrap()).unwrap().replace(replacement, adapter["original"].as_str().unwrap());
    assert_eq!(format!("{:x}", Sha256::digest(pristine.as_bytes())), provenance["pristine_sha256"].as_str().unwrap());
    let module = naga::front::wgsl::parse_str(plane::SOURCE).unwrap();
    let (_, row) = module.types.iter().find(|(_,t)|t.name.as_deref()==Some("PlaneRow")).unwrap();
    let naga::TypeInner::Struct{members,span}=&row.inner else{panic!("PlaneRow struct")};
    assert_eq!(*span as usize, size_of::<plane::PlaneRow>());
    assert_eq!(*span, 48);
    let offsets = [offset_of!(plane::PlaneRow,origin),offset_of!(plane::PlaneRow,instance_id),offset_of!(plane::PlaneRow,x),offset_of!(plane::PlaneRow,radius),offset_of!(plane::PlaneRow,y),offset_of!(plane::PlaneRow,pad)];
    assert_eq!(members.iter().map(|m|m.offset as usize).collect::<Vec<_>>(), offsets);
    assert!(plane::SOURCE.contains("const PLANE_LINES: u32 = 22u;"));
    assert!(plane::SOURCE.contains("fn face_normal("));
    assert!(plane::SOURCE.contains("fn physical_primitive("));
}

#[test]
fn plane_size_command_is_presentation_only_and_validates_its_argument() {
    use crate::{commands::{parse,Command}, editor::{Action,Change,Editor}};
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    let source = Rc::new(Plane::xy_plane());
    let _ = source.guid();
    let id = crate::plane_original::insert(&mut editor.scene, source.clone()).unwrap();
    editor.apply(Action::Picked(Some(id))).unwrap();
    let original = source.to_proto();
    let history = editor.scenes().count();
    let Command::Editor(action) = parse("View Plane Size 250").unwrap() else{panic!("editor command")};
    assert_eq!(editor.apply(action).unwrap(), Change::View);
    assert_eq!(editor.plane_size, 250.0);
    assert_eq!(editor.selected, Some(id));
    assert_eq!(editor.scenes().count(), history);
    assert_eq!(source.to_proto(), original);
    for invalid in ["View Plane Size", "View Plane Size 0", "View Plane Size -5", "View Plane Size NaN", "View Plane Size 5 extra"] {
        assert!(parse(invalid).is_err(), "{invalid}");
    }
}

fn dump(name:&str,pixels:&[u8]) {
    if let Some(directory)=std::env::var_os("PLANE_WITNESS_DIR") {
        let directory=std::path::Path::new(&directory);std::fs::create_dir_all(directory).unwrap();
        let mut ppm=b"P6\n320 240\n255\n".to_vec();for pixel in pixels.chunks_exact(4){ppm.extend_from_slice(&pixel[..3]);}
        std::fs::write(directory.join(format!("{name}.ppm")),ppm).unwrap();
    }
}

#[test]
fn actual_plane_grid_arrows_size_visibility_selection_and_owner_picking() {
    use crate::cap_gpu_tests::{pixels_at,renderer,wait};
    let mut scene=Scene::empty();scene.next_id=16_777_217;
    let source=Rc::new(Plane::xy_plane());let _=source.guid();let original=source.to_proto();
    let id=crate::plane_original::insert(&mut scene,source.clone()).unwrap();
    scene.place(id,Xform::translation(0.0,0.0,-500.0)).unwrap();
    let transform=Xform::orthographic(-250.0,250.0,-187.5,187.5,1000.0,0.0).to_f32();
    let mut gpu=renderer(&scene);gpu.set_opacity(1.0);
    let grey=|p:&[u8]|p[0].abs_diff(p[1])<6&&p[1].abs_diff(p[2])<6&&p[0]<235;
    let pink=|p:&[u8]|i32::from(p[0])-i32::from(p[1])>25;
    let green=|p:&[u8]|i32::from(p[1])-i32::from(p[2])>25&&p[1]>p[0];
    let count=|image:&[u8],kind:fn(&[u8])->bool|image.chunks_exact(4).filter(|p|kind(p)).count();
    let mut covered=scene.clone();
    let mut lid=session_rust::Mesh::create_box(400.0,400.0,20.0);lid.set_objectcolor(Color::new(0.2,0.4,0.8,1.0));lid.set_linecolors(vec![],vec![1.0]);
    let lid=covered.insert(crate::prepared::PreparedMesh::new(lid).unwrap()).unwrap();covered.place(lid,Xform::translation(0.0,0.0,-450.0)).unwrap();
    for samples in [1,4] {
        gpu.set_samples(samples);gpu.set_scene(&scene,None);gpu.set_plane_size(100.0).unwrap();
        let open=pixels_at(&gpu,transform);dump(&format!("plane-{samples}x"),&open);
        assert_eq!(gpu.plane_usage(),[1,48]);assert!(count(&open,grey)>200);
        assert!(count(&open,pink)>20&&count(&open,green)>20);
        gpu.request_pick([-0.32,40.0/187.5],transform).unwrap();wait(&gpu);
        let hit=gpu.take_pick(transform).unwrap().unwrap().unwrap();assert_eq!(gpu.resolve_pick(&scene,hit).unwrap().owner,id);
        gpu.select_objects(&scene,&[id]).unwrap();let selected=pixels_at(&gpu,transform);
        assert!(open.chunks_exact(4).zip(selected.chunks_exact(4)).filter(|(a,b)|a!=b).count()>100);
        gpu.select_objects(&scene,&[]).unwrap();
        let usage=gpu.plane_usage();gpu.set_plane_size(50.0).unwrap();let small=pixels_at(&gpu,transform);
        let columns=|image:&[u8]|(0..320).filter(|x|(0..240).any(|y|grey(crate::cap_gpu_tests::at(image,*x,y)))).count();
        let wide=columns(&open);let narrow=columns(&small);assert!(narrow*10<wide*6&&narrow*10>wide*4,"Half-size plane: {narrow}/{wide}");assert_eq!(gpu.plane_usage(),usage);
        gpu.set_plane_size(100.0).unwrap();assert_eq!(pixels_at(&gpu,transform),open);
        let mut visibility=crate::ink_ranges::Visibility::default();visibility.lines=false;gpu.set_ink_visibility(visibility);
        assert!(pixels_at(&gpu,transform).chunks_exact(4).all(|p|p[..3]==[255,255,255]));
        gpu.set_ink_visibility(crate::ink_ranges::Visibility{edges:false,points:false,lines:true,vertices:false});
        gpu.set_scene(&covered,None);let under=pixels_at(&gpu,transform);dump(&format!("plane-covered-{samples}x"),&under);
        assert!(count(&under,grey)*10<count(&open,grey),"Opaque lid hides the grid");
        assert!(count(&under,pink)*10>=count(&open,pink)*8&&count(&under,green)*10>=count(&open,green)*8,"Production colored arrows stay over the lid");
    }
    assert_eq!(source.to_proto(),original);
}
