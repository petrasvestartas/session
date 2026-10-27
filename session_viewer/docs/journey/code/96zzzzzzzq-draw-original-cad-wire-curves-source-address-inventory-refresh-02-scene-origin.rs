let path_id = scene.paths()[0].id;
 let production_points = crate::production_joined::source_points(&far.paths()[0].prepared.source);
 let expected_path_ids = production_points.windows(2)
  .filter(|pair| pair[0] != pair[1] || matches!(&far.paths()[0].prepared.source, crate::chain::Source::Line(_)))
  .count();
 let unanchored = crate::pick_sources::Sources::new(&far).unwrap();
 assert_eq!(unanchored.path_ids.len(), expected_path_ids);
 assert_eq!(unanchored.path_ids, baseline.path_ids);
 for &address in &unanchored.path_ids {
  let target = unanchored.resolve(&far, address, crate::pick_sources::Mode::Edge).expect("Translated path address remains valid");
  assert_eq!(target.owner, path_id);
  assert!(matches!(target.part, crate::pick_sources::Part::PathEdge(_)));
 }
