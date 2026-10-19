use crate::{bench_samples::summarize,bench_config::Conditions,bench_compare::{Run,compare}};
fn run()->Run{Run{conditions:Conditions{machine:"Lenovo fixture".into(),browser:"Chrome fixture".into(),adapter:"recorded adapter".into(),scene_sha256:"a".repeat(64),viewport:[1280,720],dpr:1.,camera:"saved pose and orbit path".into(),settings:"same opacity, AO, outlines and MSAA".into(),measurement:"presented browser frames".into()},revision:"before".into(),warmup:2,frame_ms:vec![500.,200.,16.,17.,18.,20.],gpu_buffers:100,gpu_textures:200,original_bytes:300}}
#[test]
fn warmup_and_slow_tail_are_reported_separately(){let values:Vec<f64>=std::iter::once(5000.).chain((1..=100).map(|v|v as f64)).collect();let s=summarize(&values,1).unwrap();assert_eq!(s.samples,100);assert_eq!(s.median_ms,50.5);assert_eq!(s.p95_ms,95.);assert_eq!(s.max_ms,100.);}
#[test]
fn missing_or_invalid_timings_are_never_performance_evidence(){for(values,warmup)in[(vec![],0),(vec![16.],1),(vec![f64::NAN],0),(vec![0.],0),(vec![f64::INFINITY],0)]{assert!(summarize(&values,warmup).is_err());}}
#[test]
fn comparisons_reject_changed_scene_hardware_and_quality(){let a=run();let mut b=run();b.revision="after".into();b.frame_ms=vec![400.,100.,10.,11.,12.,13.];let result=compare(&a,&b).unwrap();assert_eq!(result.median_change_ms,-6.);assert_eq!(result.p95_change_ms,-7.);b.conditions.adapter="different GPU".into();assert!(compare(&a,&b).is_err());b.conditions=a.conditions.clone();b.conditions.scene_sha256="b".repeat(64);assert!(compare(&a,&b).is_err());b.conditions=a.conditions.clone();b.conditions.settings="AO disabled".into();assert!(compare(&a,&b).is_err());b.conditions=a.conditions.clone();b.original_bytes=0;assert!(compare(&a,&b).is_err());}
#[test]
fn records_round_trip_conditions_without_invented_measurements(){let a=run();let bytes=serde_json::to_vec(&a).unwrap();let b:Run=serde_json::from_slice(&bytes).unwrap();assert_eq!(b.conditions,a.conditions);assert_eq!(b.frame_ms,a.frame_ms);assert!(compare(&a,&b).is_ok());}
#[test]
fn tree_cache_reuses_camera_frames_but_refreshes_edits_and_history(){
    use crate::editor::{Editor,Action};
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();let id=e.scene.objects()[0].id;let original=crate::source_edit::original(&e.scene,id).unwrap();
    let mut panel=crate::tree_panel::TreePanel::default();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert_eq!(panel.refreshes,1);
    let before=panel.rows.iter().map(|r|r.label.clone()).collect::<Vec<_>>();
    for _ in 0..20{e.apply(Action::Orbit(0.01,0.)).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);}assert_eq!(panel.refreshes,1);assert!(crate::source_edit::same(&original,&crate::source_edit::original(&e.scene,id).unwrap()));
    e.apply(Action::SelectAll).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert_eq!(panel.refreshes,2);
    e.apply(Action::Hide).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert_eq!(panel.refreshes,3);
    e.apply(Action::Undo).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert_eq!(panel.refreshes,4);assert!(panel.rows.is_empty());
    e.apply(Action::Redo).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert_eq!(panel.refreshes,5);assert_eq!(panel.rows.iter().map(|r|r.label.clone()).collect::<Vec<_>>(),before);
    e.apply(Action::Close).unwrap();panel.refresh_cached(&e.scene,e.selected_objects(),e.interaction_epoch);assert!(panel.rows.is_empty());assert!(panel.graph.is_empty());
}
#[test]
fn graph_rows_keep_one_original_edge_and_document_labels(){
    use crate::editor::{Editor,Action};
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();e.apply(Action::AddBox).unwrap();e.apply(Action::SelectAll).unwrap();e.apply(Action::AddEdge).unwrap();
    let before=e.scene.graphs.iter().map(|(key,graph)|(*key,graph.to_proto())).collect::<Vec<_>>();
    let rows=crate::graph_edit::rows(&e.scene);assert_eq!(rows.len(),1);assert!(!rows[0].0.is_empty());assert!(!rows[0].1.is_empty());assert!(!rows[0].2.is_empty());assert_eq!(rows[0].3,"");
    assert_eq!(e.scene.graphs.iter().map(|(key,graph)|(*key,graph.to_proto())).collect::<Vec<_>>(),before);
    e.apply(Action::Undo).unwrap();assert!(crate::graph_edit::rows(&e.scene).is_empty());e.apply(Action::Redo).unwrap();assert_eq!(crate::graph_edit::rows(&e.scene),rows);
}
