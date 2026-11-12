// Integration tests for the reconstructed course, copied to its generated tests/ directory.
use std::rc::Rc;
use viewer_journey::{cad_boundary, math_quality::{self, Quality}, surface};

#[test]
fn finite_surface_quality_refines_the_authored_roof_within_its_budget() {
    let source = surface::example();
    let default = surface::Grid::new(&source).unwrap();
    let coarse = math_quality::using(Quality(0.2), || surface::Grid::new(&source)).unwrap();
    let fine = math_quality::using(Quality(0.02), || surface::Grid::new(&source)).unwrap();
    assert_ne!(coarse.steps, default.steps, "Finite quality must use the adaptive path");
    assert!(fine.samples.len() > coarse.samples.len(), "Tighter error must refine this curved roof");
    assert!(fine.steps.iter().all(|&n| n <= math_quality::SURFACE_STEPS));
    assert!(fine.indices.iter().all(|&n| (n as usize) < fine.samples.len()));
    assert_eq!(fine.boundaries.len(), 4);
    for sample in &fine.samples {
        let original = source.point_at(sample.uv[0], sample.uv[1]).unwrap();
        for axis in 0..3 { assert!((original[axis] - sample.point[axis]).abs() < 1e-12); }
    }
    assert!(math_quality::current().0.is_infinite());
}

#[test]
fn finite_brep_quality_keeps_every_original_edge_addressable() {
    let source = Rc::new(session_rust::BRep::create_box(2., 3., 4.));
    let edges = source.m_edges.len();
    let prepared = math_quality::using(Quality(0.01), || cad_boundary::Prepared::new(source.clone())).unwrap();
    assert_eq!(prepared.faces.len(), source.m_faces.len());
    for edge in 0..edges {
        let (points, spans) = math_quality::using(Quality(0.01), || prepared.display(edge).ok_or("Missing edge")).unwrap();
        assert!(points.len() >= 2);
        assert_eq!(spans.len(), points.len() - 1);
        assert!(points.iter().flatten().all(|x| x.is_finite()));
    }
    assert_eq!(source.m_edges.len(), edges);
    assert!(math_quality::current().0.is_infinite());
}

#[test]
fn rejected_preparation_restores_the_callers_quality() {
    math_quality::using(Quality(0.1), || {
        assert_eq!(math_quality::using(Quality(0.01), || Err::<(), _>("Rejected")), Err("Rejected"));
        assert_eq!(math_quality::current(), Quality(0.1));
        Ok(())
    }).unwrap();
    assert!(math_quality::current().0.is_infinite());
}
