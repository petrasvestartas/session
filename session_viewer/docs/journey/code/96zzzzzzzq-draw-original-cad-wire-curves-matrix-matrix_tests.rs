use crate::{production_matrix as matrix,scene::Scene};
use session_rust::{Color,Matrix,Session,Xform};
use prost::Message;
use std::rc::Rc;

fn source()->Matrix {
    let mut m=Matrix::from_rows(&[vec![-2.0,1.2,5.0],vec![0.0,1.005,123456789.123]]);
    m.name="stiffness".into();m.cell=0.3;
    m.mincolor=Color::new(1.0,0.0,0.0,1.0);m.maxcolor=Color::new(0.0,1.0,0.0,1.0);m.zerocolor=Color::new(0.0,0.0,1.0,1.0);m
}
fn loaded(m:&Matrix)->Scene {
    let mut session=Session::new("matrix-only");session.add_matrix(m,None);
    session.set_xform(m.guid(),Xform::translation(-0.3,0.2,-0.4));
    let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&session.to_proto().encode_to_vec(),"matrix-only",&Xform::identity(),0.0).unwrap();scene
}
#[test]
fn matrix_packing_retains_integer_decimals_sign_and_original_cells() {
    let m=source();let component=m.to_component();matrix::validate(&component).unwrap();let mut rows=matrix::MatrixRows::default();
    crate::production_matrix_walk::walk(&mut rows,&component,19);
    assert_eq!(rows.words.len(),16+6*3);assert_eq!(&rows.words[..3],&[2,3,19]);
    assert_eq!(&rows.words[17..19],&[2,0x80000000]);assert_eq!(&rows.words[20..22],&[1,200]);
    assert_eq!(&rows.words[29..31],&[1,5]);assert_eq!(&rows.words[32..34],&[123456789,123]);
    let mut bad=component.clone();bad.extra.insert("cols".into(),serde_json::json!(4));assert!(matrix::validate(&bad).is_err());
    let mut scene=Scene::empty();let a=scene.insert_matrix(Rc::new(component.clone())).unwrap();let b=scene.insert_matrix(Rc::new(component)).unwrap();
    let packed=matrix::rows(&scene,[0.0;3],&[a],4096).unwrap();assert_eq!(packed.starts,[0,34]);assert_eq!(packed.words.len(),68*4);
    assert_eq!(u32::from_ne_bytes(packed.objects[(a.value() as usize-1)*96+80..(a.value() as usize-1)*96+84].try_into().unwrap()),1);
    assert_ne!(a,b);assert!(matrix::rows(&scene,[0.0;3],&[],32).is_err());
}
#[test]
fn matrix_only_archive_preserves_component_identity_data_and_placement_with_separate_title() {
    let m=source();let mut scene=loaded(&m);assert!(scene.objects().is_empty());assert_eq!(scene.matrices().len(),1);
    assert_eq!(scene.texts().len(),1);assert!(scene.texts()[0].prepared.source.label.bare);assert_ne!(scene.texts()[0].id,scene.matrices()[0].id);
    for _ in 0..2 {
        let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();
        let components=bundle.documents.iter().flat_map(|bytes|session_rust::proto::Session::decode(bytes.as_slice()).unwrap().objects.unwrap().components).collect::<Vec<_>>();
        assert_eq!(components.len(),1);assert_eq!(components[0].guid,m.guid());let restored=Matrix::from_component(&session_rust::Component::from_proto(components[0].clone())).unwrap();assert_eq!(restored,m);
        crate::scene_document::open(&mut scene,&saved).unwrap();assert_eq!(scene.matrices().len(),1);assert_eq!(scene.texts().len(),1);assert_eq!(scene.matrices()[0].model.m[12..15],[-0.3,0.2,-0.4]);
    }
    let id=scene.matrices()[0].id;scene.remove(id);assert!(scene.matrices().is_empty());assert_eq!(scene.texts().len(),1,"Production title has independent lifetime");
}
#[test]
fn matrix_shader_provenance_and_word_offsets_match_the_actual_program() {
    use sha2::{Digest,Sha256};
    let provenance:serde_json::Value=serde_json::from_str(matrix::PROVENANCE).unwrap();
    assert_eq!(format!("{:x}",Sha256::digest(matrix::SOURCE.as_bytes())),provenance["compiled_sha256"].as_str().unwrap());
    for (name,value) in [("HEADER_WORDS",16),("VALUE_WORDS",3),("ROWS",0),("COLS",1),("INSTANCE",2),("CELL",3),("LOW",4),("HIGH",5),("LINE",6),("MIN",8),("MAX",9),("ZERO",10),("TEXT",11)] {assert!(matrix::SOURCE.contains(&format!("const {name}: u32 = {value}u;")));}
    assert!(matrix::SOURCE.contains("fn value_coverage("));assert!(matrix::SOURCE.contains("fn clip_cut("));assert!(matrix::SOURCE.contains("fn ink_depth("));
}
#[test]
fn actual_matrix_colors_digits_selection_and_object_picks_survive_sample_returns() {
    use crate::cap_gpu_tests::{renderer,pixels_at,wait,at};
    let mut m=source();m.data[5]=5.0;let scene=loaded(&m);let id=scene.matrices()[0].id;let mut gpu=renderer(&scene);
    let transform=crate::cap_gpu_tests::matrix();
    let sample=|image:&[u8],x:f64,y:f64|at(image,((x+1.0)*160.0) as usize,((1.0-y)*120.0) as usize).to_vec();
    let mut first=None;
    for samples in [1,4,1] {
        gpu.set_samples(samples);let image=pixels_at(&gpu,transform);
        let low=sample(&image,-0.38,0.12);assert!(low[0]>240&&low[1]<15&&low[2]<15,"min cell: {low:?}");
        let high=sample(&image,0.22,0.12);assert!(high[1]>240&&high[0]<15&&high[2]<15,"max cell: {high:?}");
        let zero=sample(&image,-0.38,-0.18);assert!(zero[2]>240&&zero[0]<15&&zero[1]<15,"zero cell: {zero:?}");
        if let Ok(directory)=std::env::var("MATRIX_WITNESS_DIR") {
            let directory=std::path::Path::new(&directory);std::fs::create_dir_all(directory).unwrap();
            let mut ppm=b"P6\n320 240\n255\n".to_vec();for pixel in image.chunks_exact(4){ppm.extend_from_slice(&pixel[..3]);}
            std::fs::write(directory.join(format!("matrix-{samples}x.ppm")),ppm).unwrap();
        }
        // A 36-pixel cell gives the production 0.085-em stroke less than one pixel.
        // Verify the six lit strokes of "-2" and its unlit holes, rather than demand opaque white.
        for (name,x,y) in [("minus",106,95),("top",115,92),("upper right",119,93),("middle",115,95),("lower left",112,98),("bottom",115,99)] {
            let pixel=at(&image,x,y);
            assert!(pixel[0]>240&&pixel[1]>80&&pixel[2]>80&&pixel[1].abs_diff(pixel[2])<3,"{samples}x digit {name} must cover the red cell: {pixel:?}");
        }
        for (x,y) in [(115,94),(116,97),(110,92),(104,99)] {
            let pixel=at(&image,x,y);
            assert!(pixel[0]>240&&pixel[1]<15&&pixel[2]<15,"{samples}x digit hole ({x},{y}) must retain the red cell: {pixel:?}");
        }
        gpu.request_pick([-0.38,0.12],transform).unwrap();wait(&gpu);let hit=gpu.take_pick(transform).unwrap().unwrap().unwrap();assert_eq!(gpu.resolve_pick(&scene,hit).unwrap().owner,id);
        gpu.select_objects(&scene,&[id]).unwrap();let selected=pixels_at(&gpu,transform);
        if let Ok(directory)=std::env::var("MATRIX_WITNESS_DIR") {
            let mut ppm=b"P6\n320 240\n255\n".to_vec();for pixel in selected.chunks_exact(4){ppm.extend_from_slice(&pixel[..3]);}
            std::fs::write(std::path::Path::new(&directory).join(format!("matrix-selected-{samples}x.ppm")),ppm).unwrap();
        }
        let changed=image.chunks_exact(4).zip(selected.chunks_exact(4)).filter(|(a,b)|a!=b).count();
        assert!(changed>100,"Selected matrix must change its visible borders: {changed}");
        // Integer-aligned one-pixel edges cover their nearest pixel by one half.
        // Replacing black with yellow adds red and green equally, leaving blue intact.
        for (x,y) in [(88,96),(135,96),(112,78),(231,96),(88,132),(112,149)] {
            let before=at(&image,x,y);let after=at(&selected,x,y);
            let red=i16::from(after[0])-i16::from(before[0]);let green=i16::from(after[1])-i16::from(before[1]);
            assert!((110..=145).contains(&red)&&(110..=145).contains(&green)&&after[2].abs_diff(before[2])<3,
                "{samples}x yellow edge ({x},{y}) must blend over its original cell: {before:?} -> {after:?}");
        }
        for (x,y) in [(99,105),(195,105),(99,141),(115,94),(116,97)] {
            assert_eq!(at(&selected,x,y),at(&image,x,y),"{samples}x selected borders preserve cell interiors and glyph holes at ({x},{y})");
        }
        gpu.select_objects(&scene,&[]).unwrap();
        if samples==1 {if let Some(first)=first.as_ref(){assert_eq!(first,&image);}else{first=Some(image);}}
    }
}
#[test]
fn old_text_archives_default_to_plated_world_labels() {
    let mut value=serde_json::to_value(crate::text_example::label()).unwrap();value.as_object_mut().unwrap().remove("bare");
    let label:crate::text::TextLabel=serde_json::from_value(value).unwrap();assert!(!label.bare);
}
