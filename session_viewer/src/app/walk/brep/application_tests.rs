use super::*;
use crate::app::surface_preview::SurfacePreview;
use crate::app::walk::{Walk, WalkCx, walk_geometry};
use crate::engine::gpu::{
    Upload,
    patch::{Counts, Span},
};
use session_rust::{Geometry, Point};
use std::rc::Rc;
#[test]
fn exact_rational_border_samples_preserve_faces_source_ids_and_edit_preview() {
    let mut surface = BRep::create_cylinder(1., 2.).m_surfaces.remove(0);
    let before = surface.to_proto();
    let source = Geometry::NurbsSurface(Rc::new(surface.clone()));
    let mut upload = Upload::default();
    walk_geometry(
        &mut Walk::of(&mut upload),
        &WalkCx {
            quality: Default::default(),
            vert_base: 0,
            cloud_px: 0.,
            row: 7,
            interactions: false,
            attributes: false,
        },
        &source,
    );
    let nu = surface_parameters(&surface, 0, Default::default()).len() - 1;
    let nv = surface_parameters(&surface, 1, Default::default()).len() - 1;
    let face_vertices = (nu + 1) * (nv + 1);
    assert_eq!(upload.arena.idx.len(), nu * nv * 6);
    assert!(
        upload
            .arena
            .idx
            .iter()
            .all(|i| (*i as usize) < face_vertices)
    );
    assert_eq!(upload.arena.verts.len(), face_vertices);
    let used: std::collections::HashSet<_> = upload.arena.idx.iter().copied().collect();
    assert_eq!(
        used.len(),
        face_vertices,
        "every curve sample belongs to the shaded faces"
    );
    assert_eq!(upload.arena.surface_samples.len(), upload.arena.verts.len());
    assert_eq!(
        upload.arena.surface_boundaries.len(),
        upload.seg.pipes.len()
    );
    assert!(upload.seg.pipe_ids.iter().all(|id| *id == 2 || *id == 3));
    for &(pipe, ends) in &upload.arena.surface_boundaries {
        let p = &upload.seg.pipes[pipe as usize];
        assert_eq!(p.p0, upload.arena.verts[ends[0] as usize].position);
        assert_eq!(p.p1, upload.arena.verts[ends[1] as usize].position);
        assert_eq!(p.instance_id, 7);
        for point in [p.p0, p.p1] {
            assert!((point[0] * point[0] + point[1] * point[1] - 1.).abs() < 1e-6);
        }
    }
    assert_eq!(surface.to_proto(), before);
    let preview = SurfacePreview::capture(
        &upload,
        Span {
            start: Counts::default(),
            count: Counts::of(&upload),
        },
        Counts::default(),
        &source,
    )
    .expect("Every refined endpoint retains its own UV sample");
    let (_, original, _) = preview.evaluate(&source).unwrap();
    for (a, b) in original.pipes.iter().zip(&upload.seg.pipes) {
        assert_eq!(a.p0, b.p0);
        assert_eq!(a.p1, b.p1);
    }
    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            let p = surface.get_cv(i, j).unwrap();
            assert!(surface.set_cv(i, j, &Point::new(p[0], p[1], p[2] + 0.5)));
        }
    }
    let (_, moved, _) = preview
        .evaluate(&Geometry::NurbsSurface(Rc::new(surface)))
        .unwrap();
    assert_eq!(moved.pipes.len(), original.pipes.len());
    for (a, b) in original.pipes.iter().zip(&moved.pipes) {
        for (a, b) in [(a.p0, b.p0), (a.p1, b.p1)] {
            assert!((a[2] + 0.5 - b[2]).abs() < 1e-5);
            assert_eq!([a[0], a[1]], [b[0], b[1]]);
        }
    }
}
