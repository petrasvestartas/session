use crate::{production_mesh_ink as ink,scene::Scene,pick_sources::{Sources,Part,Target}};
use session_rust::{Mesh,Point,Color,Xform};
use std::rc::Rc;
fn panel(width:f64)->Mesh {
    let mut m=Mesh::new();for(key,p)in[11,700,90003,900004].into_iter().zip([[-80.,-60.,0.],[80.,-60.,0.],[80.,60.,0.],[-80.,60.,0.]]){m.add_vertex(Point::new(p[0],p[1],p[2]),Some(key));}
    m.add_face(vec![11,700,90003,900004],Some(37));m.set_objectcolor(Color::new(0.2,0.4,0.8,1.));m.set_linecolors(vec![],vec![width]);m
}

#[test]
fn production_mesh_pipe_rows_preserve_pens_facing_sparse_keys_and_compact_ownership(){
    let mut scene=Scene::empty();scene.next_id=16_777_217;let source=panel(7.);let _=source.guid();let original=source.to_proto();let prepared=crate::prepared::PreparedMesh::new(source).unwrap();
    let ink=&prepared.display.parts.ink;assert!(ink.active);assert_eq!(ink.segments.len(),4);assert_eq!(ink.ids,[0,1,2,3]);
    assert!(ink.segments.iter().all(|r|r.segment.radius==-3.5&&r.segment.facing!=u32::MAX&&r.previous==u32::MAX&&r.next==u32::MAX));
    let id=scene.insert(prepared).unwrap();let anchor=[1_000_000_000.,0.,0.];scene.place(id,Xform::translation(anchor[0]+0.125,0.,-500.)).unwrap();
    let sources=Sources::anchored(&scene,anchor).unwrap();let rows=crate::production_joined::rows(&scene,&sources,anchor,&[id],4096).unwrap();
    assert_eq!(rows.object_ids,[id]);assert_eq!(rows.objects.len(),96);assert_eq!(rows.mesh_from,0);assert_eq!(rows.segments.len(),4*52);assert_eq!(rows.runs[2],[0..24]);
    assert_eq!(f32::from_ne_bytes(rows.translations[..4].try_into().unwrap()),0.125);
    for(i,pixel)in rows.ids.iter().enumerate(){let(a,b)=scene.objects()[0].mesh.parts.edges[i];assert_eq!(sources.resolve(&scene,*pixel,crate::pick_sources::Mode::Edge),Some(Target{owner:id,part:Part::MeshEdge(a,b)}));}
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    assert!(sources.mesh_legacy.iter().all(|v|*v==0));assert_eq!(sources.mesh_ink.len(),4,"Original source-query fixture rows are retained");
    let default=ink::mesh(&panel(1.),2);assert!(default.segments.iter().all(|r|r.segment.radius==0.));
    for widths in[vec![0.],vec![0.;4]]{let mut hidden=panel(1.);hidden.set_linecolors(vec![],widths);let prepared=ink::mesh(&hidden,2);assert!(prepared.active&&prepared.segments.is_empty());}
}

#[test]
fn production_brep_pipe_rows_keep_only_original_edges_and_rigid_replay_normals(){
    let source=Rc::new(crate::cad_display::example());let _=source.guid();let original=source.to_proto();let mut cache=crate::cad_shape_cache::Cache::default();let(_,first)=cache.prepare(source.clone()).unwrap();let first=&first.display.parts.ink;
    assert!(first.active&&!first.segments.is_empty());assert_eq!(first.ids.len(),first.segments.len());assert_eq!(first.normals.len(),first.segments.len());
    assert!(first.segments.iter().any(|r|r.sag>0.),"Actual curved boundaries carry production chord slack");
    assert!(first.segments.iter().zip(&first.ids).all(|(r,id)|r.sag>=0.&&(*id as usize)<source.m_edges.len()),"CAD strokes belong only to original boundary edges");
    let motion=&Xform::translation(30.,-20.,10.)*&Xform::rotation_z(90.,true);let mut moved=source.transformed(&motion);moved.set_guid("rigid-pipe-copy".into());let moved=Rc::new(moved);let(_,replayed)=cache.prepare(moved.clone()).unwrap();let(_,independent)=crate::cad_display::prepare(moved).unwrap();
    let a=&replayed.display.parts.ink;let b=&independent.display.parts.ink;assert_eq!(cache.walks,1);assert_eq!(cache.replays,1);assert_eq!(a.ids.len(),b.ids.len());
    for(r,(n0,n1))in a.segments.iter().zip(&a.normals){assert_eq!(r.segment.facing,ink::pack_facing(n0.as_ref(),n1.as_ref()));}
    for(before,after)in first.segments.iter().zip(&a.segments){for(p,q)in[(before.segment.p0,after.segment.p0),(before.segment.p1,after.segment.p1)]{let p=motion.transform_point(&Point::new(p[0]as f64,p[1]as f64,p[2]as f64));for axis in 0..3{assert!((p[axis]-q[axis]as f64).abs()<1e-4);}}assert_eq!(before.sag.to_bits(),after.sag.to_bits());}
    assert_eq!(source.to_proto(),original);
}

#[test]
fn production_surface_boundary_pipes_join_without_inventing_source_edges(){
    let source=Rc::new(crate::surface::example());let prepared=crate::surface::Prepared::new(source).unwrap();let mesh=prepared.mesh().unwrap();let ink=&mesh.display.parts.ink;
    assert_eq!(ink.segments.len(),64);assert_eq!(ink.flags,1024|128|64|16);
    for edge in 0..4{assert_eq!(ink.ids.iter().filter(|id|**id==edge).count(),16);}
    assert!(ink.segments.iter().any(|r|r.next!=u32::MAX));assert!(ink.segments.iter().all(|r|r.segment.facing==u32::MAX&&r.sag==0.));
}

#[test]
fn production_mesh_edge_extract_and_complete_shader_hashes_match(){
    use sha2::{Digest,Sha256};let proof:serde_json::Value=serde_json::from_str(ink::PROVENANCE).unwrap();
    for(name,source)in[("production_mesh_ink.rs",include_str!("production_mesh_ink.rs")),("production_brep_ink.rs",include_str!("production_brep_ink.rs"))]{assert_eq!(format!("{:x}",Sha256::digest(source.as_bytes())),proof["extracted_modules"][name].as_str().unwrap());}
    assert_eq!(format!("{:x}",Sha256::digest(crate::production_joined::SOURCE.as_bytes())),proof["compiled_shader_sha256"].as_str().unwrap());
}

fn display_archive(scene:&Scene)->(Vec<session_rust::proto::Session>,serde_json::Value){
    use prost::Message;
    let bundle=crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();
    let documents=bundle.documents.iter().map(|bytes|{
        let mut document=session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
        // Placement snapshots create fresh transform IDs; their matrices and source-owner GUIDs remain exact.
        for entry in &mut document.xforms {if let Some(transform)=&mut entry.xform{transform.guid.clear();}}
        document
    }).collect();
    (documents,serde_json::to_value(bundle.metadata).unwrap())
}

#[test]
fn actual_mesh_edges_use_global_production_pen_aa_selection_and_family_switches(){
    use crate::cap_gpu_tests::{renderer,pixels_at,wait};let mut scene=Scene::empty();let mesh=panel(1.);let _=mesh.guid();let id=scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();scene.place(id,Xform::translation(0.,0.,-500.)).unwrap();
    let source=display_archive(&scene);let matrix=Xform::orthographic(-200.,200.,-150.,150.,1000.,0.).to_f32();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.set_lighting(false,false);gpu.set_ink_visibility(crate::ink_ranges::Visibility{edges:true,lines:false,points:false,vertices:false});
    for samples in[1,4]{gpu.set_samples(samples);for density in[1.,2.]{gpu.set_density(density);gpu.set_scene(&scene,None);gpu.set_display_settings(crate::display_settings::Settings{thickness:1.,..Default::default()});let thin=pixels_at(&gpu,matrix);
        gpu.set_display_settings(crate::display_settings::Settings{thickness:4.,..Default::default()});let thick=pixels_at(&gpu,matrix);
        let black=|image:&[u8]|image.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count();assert!(black(&thick)>black(&thin)+100,"Global production pen expands original mesh edges");
        gpu.set_display_settings(crate::display_settings::Settings{thickness:4.,feather:4.,..Default::default()});assert!(pixels_at(&gpu,matrix)==thick,"Production ribbon AA is analytic; the feather control applies to glyphs");
        gpu.set_pick_mode(crate::pick_sources::Mode::Edge);gpu.request_pick([0.,-0.4],matrix).unwrap();wait(&gpu);let pixel=gpu.take_pick(matrix).unwrap().unwrap().unwrap();let target=gpu.resolve_pick(&scene,pixel).unwrap();assert_eq!(target,Target{owner:id,part:Part::MeshEdge(11,700)});
        gpu.set_highlight(&scene,Some(target)).unwrap();let selected=pixels_at(&gpu,matrix);assert!(selected.chunks_exact(4).any(|p|p[0]>220&&p[1]>220&&p[2]<30));
        gpu.set_ink_visibility(crate::ink_ranges::Visibility{edges:false,lines:true,points:false,vertices:false});let off=pixels_at(&gpu,matrix);assert_eq!(black(&off),0,"Lines cannot re-enable mesh Edges");gpu.set_ink_visibility(crate::ink_ranges::Visibility{edges:true,lines:false,points:false,vertices:false});gpu.set_highlight(&scene,None).unwrap();assert_eq!(display_archive(&scene),source);
    }}
}
