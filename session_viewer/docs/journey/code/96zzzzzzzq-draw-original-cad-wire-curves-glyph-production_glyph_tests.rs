use crate::{production_glyph as glyph,production_joined as joined,scene::Scene};
use session_rust::{Point,Mesh,Color};
use std::rc::Rc;

#[test]
fn actual_glyph_programs_and_48_byte_rows_keep_the_production_algorithms(){
    use sha2::{Digest,Sha256};use wgpu::naga;
    let provenance:serde_json::Value=serde_json::from_str(glyph::PROVENANCE).unwrap();
    for(name,source)in[("glyph",glyph::GLYPH),("sphere",glyph::SPHERE)]{
        let p=&provenance[name];let adapter=&p["ownership_adapter"];
        assert_eq!(format!("{:x}",Sha256::digest(source.as_bytes())),p["compiled_sha256"].as_str().unwrap());
        let replacement=adapter["replacement"].as_str().unwrap();assert_eq!(source.matches(replacement).count(),1);
        let original=source.strip_suffix(adapter["declaration"].as_str().unwrap()).unwrap().replace(replacement,adapter["original"].as_str().unwrap());
        assert_eq!(format!("{:x}",Sha256::digest(original.as_bytes())),p["pristine_sha256"].as_str().unwrap());
        let module=naga::front::wgsl::parse_str(source).unwrap();
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&module).unwrap();
        let(_,row)=module.types.iter().find(|(_,t)|t.name.as_deref()==Some("GlyphPoint")).unwrap();
        let naga::TypeInner::Struct{members,span}=&row.inner else{panic!("GlyphPoint")};
        assert_eq!(*span as usize,std::mem::size_of::<glyph::GlyphPoint>());assert_eq!(*span,48);
        assert_eq!(members.iter().map(|m|m.offset).collect::<Vec<_>>(),[0,12,16,32,36,40]);
        assert!(source.contains("fn face_normal("));assert!(source.contains("fn physical_primitive("));
    }
}

#[test]
fn actual_point_walk_uses_physical_radius_and_compact_owners_without_mutating_source(){
    let mut scene=Scene::empty();scene.next_id=16_777_217;
    for(width,radius)in[(1.,-3.),(12.,-6.),(0.2,-0.1)]{
        let mut point=Point::new(10.,20.,-500.);point.width=width;point.pointcolor=Color::new(0.8,0.2,0.1,0.7);
        let point=Rc::new(point);let _=point.guid();let original=point.to_proto();
        let id=scene.insert_point(crate::marker::PreparedPoint::new(point.clone()).unwrap()).unwrap();
        let mut walked=glyph::GlyphRows::default();glyph::walk_point(&mut walked,&point,7);
        assert_eq!(walked.dots.len(),1);assert_eq!(walked.dots[0].radius,radius);assert_eq!(walked.dots[0].instance_id,7);
        assert_eq!(point.to_proto(),original);
        let sources=crate::pick_sources::Sources::new(&scene).unwrap();let rows=joined::rows(&scene,&sources,[0.;3],&[id],4096).unwrap();
        assert_eq!(rows.dots.len(),scene.points().len()*48);assert_eq!(rows.objects.len(),scene.points().len()*96);
        let index=scene.points().len()-1;assert_eq!(rows.pick_owners[index],id.value());
        assert_eq!(u32::from_ne_bytes(rows.dots[index*48+32..index*48+36].try_into().unwrap()),index as u32);
        assert_eq!(u32::from_ne_bytes(rows.objects[index*96+80..index*96+84].try_into().unwrap())&1,1);
    }
}

#[test]
fn mesh_marker_metadata_keeps_sparse_addresses_normals_and_widest_incident_width(){
    let mut mesh=Mesh::new();for(key,p)in[11,700,90003,900004].into_iter().zip([[0.,0.,0.],[100.,0.,0.],[100.,100.,0.],[0.,100.,0.]]){mesh.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![11,700,90003,900004],Some(33));mesh.set_linecolors(vec![],vec![2.,8.,0.,4.]);let _=mesh.guid();let original=mesh.to_proto();
    let parts=crate::mesh_parts::Parts::new(&mesh,2).unwrap();assert_eq!(parts.dots.len(),4);assert_eq!(parts.dot_styles.len(),4);
    let keys=mesh.vertices();let mut expected=vec![f64::NEG_INFINITY;keys.len()];
    for(i,&(a,b))in parts.edges.iter().enumerate(){let width=mesh.widths()[i];if width!=0.{for key in[a,b]{let slot=keys.iter().position(|k|*k==key).unwrap();expected[slot]=expected[slot].max(width);}}}
    for(style,width)in parts.dot_styles.iter().zip(expected){assert_eq!(style.radius,crate::production_ribbon::encode_width(width));assert_ne!(style.facing[0],u32::MAX);}
    assert_eq!(parts.glyph_flags,1024|16|128);assert_eq!(mesh.to_proto(),original);
    let prepared=Rc::new(crate::prepared::PreparedMesh::new(mesh).unwrap());let _=prepared;
}

#[test]
fn display_controls_follow_production_parser_and_leave_source_and_history_intact(){
    use crate::{display_settings::Settings,editor::{Editor,Action,Change},commands::{parse,Command}};
    let defaults=Settings::default();assert_eq!([defaults.thickness,defaults.feather,defaults.cloud_scale,defaults.edl,defaults.lod],[1.,1.,1.,0.25,0.]);
    let settings=Settings::from_values(|name|Some(match name{"thickness"=>"-5","aa"=>"10","cloud"=>"-2","edl"=>"-1",_=>"NaN"}.into()));
    assert_eq!([settings.thickness,settings.feather,settings.cloud_scale,settings.edl,settings.lod],[0.1,4.,-2.,-1.,0.]);
    let matrix=session_rust::Xform::identity().to_f32();let line=settings.line(matrix,[320,240],2.,1.,[0.;3]);
    assert_eq!(f32::from_ne_bytes(line[..4].try_into().unwrap()),0.2);assert_eq!(f32::from_ne_bytes(line[44..48].try_into().unwrap()),4.);
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();let history=editor.scenes().count();let source=crate::scene_archive::save(&editor.scene).unwrap();
    for(text,expected)in[("View Point Size 0.1",0.25),("View Point Size 20",8.),("View Point Size 2",2.)]{let Command::Editor(action)=parse(text).unwrap()else{panic!("editor")};assert_eq!(editor.apply(action).unwrap(),Change::View);assert_eq!(editor.display.cloud_scale,expected);}
    assert_eq!(editor.scenes().count(),history);assert_eq!(crate::scene_archive::save(&editor.scene).unwrap(),source);
    for bad in["View Point Size","View Point Size 0","View Point Size NaN","View Point Size -1","View Point Size 1 extra"]{assert!(parse(bad).is_err(),"{bad}");}
}

#[test]
fn production_point_pixels_keep_physical_diameter_and_selection_coverage_at_both_densities(){
    use crate::cap_gpu_tests::{renderer,pixels_at,wait};
    let mut scene=Scene::empty();let mut source=Point::new(0.,0.,-500.);source.width=12.;source.pointcolor=Color::new(0.8,0.1,0.1,1.);
    let id=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(source)).unwrap()).unwrap();
    let original=crate::scene_archive::save(&scene).unwrap();let matrix=session_rust::Xform::orthographic(-200.,200.,-150.,150.,1000.,0.).to_f32();
    let mut gpu=renderer(&scene);gpu.set_opacity(1.);
    for samples in[1,4]{gpu.set_samples(samples);for density in[1.,2.]{gpu.set_density(density);gpu.set_scene(&scene,None);
        let image=pixels_at(&gpu,matrix);let red:Vec<_>=image.chunks_exact(4).enumerate().filter(|(_,p)|i16::from(p[0])-i16::from(p[1])>60).map(|(i,_)|(i%320,i/320)).collect();assert!(!red.is_empty());
        for axis in[0,1]{let values:Vec<_>=red.iter().map(|&(x,y)|if axis==0{x}else{y}).collect();assert_eq!(values.iter().max().unwrap()-values.iter().min().unwrap()+1,12,"Production fixed radius is six physical pixels at DPR {density}, MSAA {samples}");}
        gpu.request_pick([0.,0.],matrix).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix).unwrap().unwrap().unwrap()[0],id.value());
        gpu.set_scene(&scene,Some(id));let selected=pixels_at(&gpu,matrix);assert_ne!(selected,image);
        let occupied=|p:&[u8]|p.chunks_exact(4).enumerate().filter(|(_,c)|c[..3]!=[255,255,255]).map(|(i,_)|i).collect::<Vec<_>>();assert_eq!(occupied(&selected),occupied(&image),"Production selection changes color without growing the disc");
        gpu.set_ink_visibility(crate::ink_ranges::Visibility{points:false,..Default::default()});let hidden=pixels_at(&gpu,matrix);assert!(hidden.chunks_exact(4).all(|p|p[..3]==[255,255,255]));gpu.set_ink_visibility(Default::default());
        assert_eq!(crate::scene_archive::save(&scene).unwrap(),original);
    }}
}
