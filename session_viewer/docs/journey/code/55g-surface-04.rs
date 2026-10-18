use crate::surface::{Grid,validate};
use session_rust::{NurbsSurface,Point};

fn plane()->NurbsSurface {NurbsSurface::create(false,false,1,1,2,2,&[Point::new(-1.,-2.,0.),Point::new(-1.,2.,0.),Point::new(1.,-2.,0.),Point::new(1.,2.,0.)]).unwrap()}

#[test]
fn parametric_grid_retains_exact_uvs_analytic_normals_and_four_original_boundaries() {
    let source=plane();let before=source.to_proto();let grid=Grid::new(&source).unwrap();assert_eq!(grid.steps,[16,16]);assert_eq!(grid.samples.len(),289);assert_eq!(grid.indices.len(),16*16*6);
    assert_eq!(grid.samples[0].point,[-1.,-2.,0.]);assert_eq!(grid.samples[288].point,[1.,2.,0.]);assert_eq!(grid.samples[144].point,[0.;3]);assert_eq!(grid.samples[144].uv,[0.5,0.5]);assert_eq!(grid.normal(144).unwrap(),[0.,0.,1.]);
    assert_eq!(grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>(),vec![0,1,2,3]);assert!(grid.boundaries.iter().all(|b|b.segments.len()==16));assert_eq!(source.to_proto(),before);
    assert!(grid.indices.iter().all(|i|(*i as usize)<grid.samples.len()));
}

#[test]
fn rational_sampling_preserves_weights_and_original_nonunit_parameter_domains() {
    let mut source=plane();source.to_rational();assert!(source.set_cv_4d(1,1,2.,4.,0.,2.));source.m_nurbsknot[0]=vec![2.,9.];source.m_nurbsknot[1]=vec![-3.,5.];let before=source.to_proto();
    let grid=Grid::new(&source).unwrap();assert_eq!(grid.samples[0].uv,[2.,-3.]);assert_eq!(grid.samples.last().unwrap().uv,[9.,5.]);
    assert!((grid.samples[144].point[0]-0.2).abs()<1e-12&&(grid.samples[144].point[1]-0.4).abs()<1e-12);
    for sample in &grid.samples {let point=source.point_at(sample.uv[0],sample.uv[1]).unwrap();assert_eq!(sample.point,[point[0],point[1],point[2]]);}
    assert_eq!(source.get_cv_4d(1,1),Some((2.,4.,0.,2.)));assert_eq!(source.to_proto(),before);
}

#[test]
fn malformed_controls_weights_knots_and_overflow_are_refused_before_evaluation() {
    let mut source=plane();source.m_cv.pop();assert!(validate(&source).is_err());let mut source=plane();source.m_cv_stride=[usize::MAX,3];assert!(validate(&source).is_err());
    let mut source=plane();source.m_cv_stride=[3,3];assert!(validate(&source).is_err());
    let mut source=plane();source.m_cv[0]=f64::NAN;assert!(validate(&source).is_err());let mut source=plane();source.m_nurbsknot[0]=vec![2.,1.];assert!(validate(&source).is_err());
    let mut source=plane();source.m_nurbsknot[0]=vec![1.,1.];assert!(validate(&source).is_err());let mut source=plane();source.to_rational();source.m_cv[3]=0.;assert!(validate(&source).is_err());
    let mut source=plane();source.m_cv_count=[usize::MAX,usize::MAX];assert!(validate(&source).is_err());let mut source=plane();source.m_order[0]=33;assert!(validate(&source).is_err());
}

#[test]
fn source_owned_surface_draws_one_face_and_only_original_domain_edges() {
    let mut e=crate::editor::Editor::default();e.apply(crate::editor::Action::Close).unwrap();e.apply(crate::editor::Action::AddSurface).unwrap();let row=&e.scene.objects()[0];let id=row.id;let surface=row.surface.as_ref().unwrap();
    assert_eq!(row.metadata.source_guid,surface.source.guid());assert_eq!(row.mesh.parts.faces,vec![0]);assert!(row.mesh.parts.triangles.iter().all(|slot|*slot==0));assert_eq!(row.mesh.parts.endpoints.len(),64);assert_eq!(surface.grid.samples[144].uv,[0.5,0.5]);
    let sources=crate::pick_sources::Sources::new(&e.scene).unwrap();assert_eq!(sources.faces.len(),1);assert_eq!(sources.edges.len(),64);assert!(sources.edges.iter().all(|target|matches!(target.part,crate::pick_sources::Part::SurfaceEdge(0..=3))));
    assert!(crate::document::snapshot(&e.scene).unwrap_err().contains("CAD surface"));assert_eq!(crate::memory::surfaces([&e.scene])[..3],[1,1,1]);
    let before=e.scene.objects()[0].surface.as_ref().unwrap().source.to_proto();e.select_objects([id],false);e.apply(crate::editor::Action::Translate([0.123456789012345,0.,0.])).unwrap();assert_eq!(e.scene.objects()[0].surface.as_ref().unwrap().source.to_proto(),before);
    e.apply(crate::editor::Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,session_rust::Xform::identity().m);e.apply(crate::editor::Action::Undo).unwrap();assert!(e.scene.objects().is_empty());e.apply(crate::editor::Action::Redo).unwrap();assert_eq!(e.scene.objects()[0].id,id);
}

#[test]
fn metadata_and_display_owners_do_not_pin_original_surface_after_close() {
    let mut e=crate::editor::Editor::default();e.apply(crate::editor::Action::Close).unwrap();e.apply(crate::editor::Action::AddSurface).unwrap();let row=&e.scene.objects()[0];let weak=std::rc::Rc::downgrade(&row.surface.as_ref().unwrap().source);let metadata=row.metadata.clone();let display=row.mesh.clone();
    assert_eq!(crate::memory::surfaces(e.presentation_scenes())[1..3],[1,1]);e.apply(crate::editor::Action::Close).unwrap();assert!(weak.upgrade().is_none());assert_eq!(metadata.name,"Parametric roof");assert_eq!(display.indices().len(),16*16*6);assert_eq!(crate::memory::surfaces(e.presentation_scenes()),[0;5]);
}

#[test]
fn transposed_padded_control_layout_samples_without_rewriting_original_storage() {
    let source=plane();let mut transposed=source.clone();transposed.m_cv_stride.swap(0,1);let before=transposed.to_proto();let grid=Grid::new(&transposed).unwrap();
    assert_eq!(grid.samples[16].point,[1.,-2.,0.]);assert_eq!(grid.samples[272].point,[-1.,2.,0.]);assert_eq!(transposed.to_proto(),before);
    let mut padded=plane();padded.m_cv_stride=[12,6];padded.m_cv=vec![0.;21];for (i,j) in [(0,0),(0,1),(1,0),(1,1)] {assert!(padded.set_cv(i,j,&source.get_cv(i,j).unwrap()));}
    let before=padded.to_proto();assert_eq!(Grid::new(&padded).unwrap().samples[144].point,[0.;3]);assert_eq!(padded.to_proto(),before);
}
