use std::rc::Rc;
use crate::cloud::{Prepared,Cloud,LodNode};
use session_rust::{PointCloud,Xform};

fn source()->PointCloud {
    let coords=(0..121).flat_map(|i|[(i%11) as f64*0.1,(i/11) as f64*0.1,0.5]).collect();
    PointCloud::from_coords(coords,vec![],vec![])
}
#[test]
fn cloud_lod_preparation_keeps_original_coordinates_ids_and_source_lifetime() {
    let mut source=source();let original=source.coords().chunks_exact(3).map(|p|[p[0],p[1],p[2]]).collect::<Vec<_>>();source.build_lod(0.25,8);let source=Rc::new(source);let _=source.guid();let before=source.to_proto();
    let prepared=Prepared::new(source.clone()).unwrap();assert!(!prepared.nodes.is_empty());
    for (i,id) in source.point_ids().iter().enumerate() {assert_eq!(&source.coords()[i*3..i*3+3],&original[*id as usize]);}
    assert_eq!(source.to_proto(),before);assert_eq!(prepared.bounds.min,[0.,0.,0.5]);assert_eq!(prepared.bounds.max,[1.,1.,0.5]);let weak=Rc::downgrade(&source);drop(source);assert!(weak.upgrade().is_some());drop(prepared);assert!(weak.upgrade().is_none());
}
#[test]
fn cloud_rejects_malformed_octree_columns_repeated_owners_and_source_ids() {
    let mut source=source();source.build_lod(0.25,8);let baseline=source.to_proto();assert!(baseline.lod_size.len()>1);
    let mut bad=baseline.clone();bad.lod_children.pop();assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
    let mut bad=baseline.clone();bad.lod_children[0]=0;assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
    let mut bad=baseline.clone();bad.lod_first[1]=bad.lod_first[0];bad.lod_count[1]=1;assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
    let mut bad=baseline.clone();bad.point_ids[1]=bad.point_ids[0];assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
    let mut bad=baseline.clone();bad.lod_min[0]+=100.;assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
    let mut bad=baseline;bad.coords[0]=f64::NAN;assert!(Prepared::new(Rc::new(PointCloud::from_proto(bad))).is_err());
}
#[test]
fn current_cloud_lod_walk_obeys_screen_spacing_small_clouds_and_resident_prefixes() {
    use crate::cloud_lod::{LodWalk,Projection};
    let nodes=[LodNode {center:[0.;3],size:1.,spacing:1.,first:0,count:1,children:[1,-1,-1,-1,-1,-1,-1,-1]},LodNode {center:[0.;3],size:0.5,spacing:0.1,first:1,count:9,children:[-1;8]}];let mut walk=LodWalk::default();let mut cloud=Cloud {spacing:1.,node_first:0,node_count:2,resident:2_000_000};let mut view=Projection {eye:[0.,0.,3.],ortho_h:100.,height_px:200,lod_px:2.,nodes:&nodes};let model=Xform::identity().to_f32();
    walk.select(&view,&cloud,&model);assert_eq!(walk.ranges.iter().map(|r|(r.first,r.count)).collect::<Vec<_>>(),[(0,1)]);
    view.ortho_h=1.;walk.select(&view,&cloud,&model);assert_eq!(walk.ranges.iter().map(|r|(r.first,r.count)).collect::<Vec<_>>(),[(0,1),(1,9)]);assert!(walk.ranges.iter().all(|r|r.spacing==0.1&&r.tile));
    cloud.resident=7;walk.select(&view,&cloud,&model);assert_eq!((walk.ranges[0].first,walk.ranges[0].count,walk.ranges[0].tile),(0,7,false));
    cloud.resident=2_000_000;view.lod_px=0.;walk.select(&view,&cloud,&model);assert_eq!(walk.ranges[0].count,2_000_000);
}
#[test]
fn current_cloud_radius_factor_keeps_tile_spacing_and_object_scale() {
    use crate::cloud_lod::{Range,radius_factor};let run=Range {first:0,count:7,spacing:12.,tile:false};let tile=Range {tile:true,..run};
    assert!((radius_factor(&run,6.,2.,100.)-0.12).abs()<1e-6);assert!((radius_factor(&tile,1.,2.,100.)-0.06).abs()<1e-6);assert!(radius_factor(&run,6.,2.,0.)>radius_factor(&run,6.,1.,0.));
}
#[test]
fn cloud_row_copy_layers_placement_delete_undo_and_close_share_one_precise_source() {
    use crate::editor::{Editor,Action};let mut editor=Editor::default();editor.apply(Action::Close).unwrap();let source=Rc::new(source());let id=editor.scene.insert_cloud(Prepared::new(source.clone()).unwrap()).unwrap();let weak=Rc::downgrade(&source);drop(source);
    editor.apply(Action::Picked(Some(id))).unwrap();let original=editor.scene.clouds()[0].prepared.source.coords().to_vec();let mut history=crate::history::History::default();history.try_edit(&mut editor.scene,|scene|scene.translate(id,[1_000_000_000.125,0.,0.])).unwrap();assert_eq!(editor.scene.clouds()[0].model.m[12],1_000_000_000.125);history.undo(&mut editor.scene);assert_eq!(editor.scene.clouds()[0].model.m[12],0.);history.redo(&mut editor.scene);
    let copy=editor.scene.copy_row(id,false).unwrap();assert_ne!(copy,id);assert!(Rc::ptr_eq(&editor.scene.clouds()[0].prepared.source,&editor.scene.clouds()[1].prepared.source));assert_eq!(editor.scene.clouds()[0].prepared.source.coords(),original);assert!(crate::document::snapshot(&editor.scene).is_err());
    editor.apply(Action::SelectAll).unwrap();assert_eq!(editor.selected_objects().len(),2);editor.apply(Action::Delete).unwrap();assert!(editor.scene.clouds().is_empty());editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),2);editor.apply(Action::Close).unwrap();drop(history);assert!(weak.upgrade().is_none());
}
