    })
}
pub fn placed(source: &OBB, model: &Xform) -> crate::bounds::Bounds {
    let points = source.corners().map(|p| p.transformed(model));
    let p = &points[0];
    let mut result = crate::bounds::Bounds::point([p[0], p[1], p[2]]);
    for p in points {
        result.include([p[0], p[1], p[2]]);
    }
    result
}
pub fn insert(
    scene: &mut crate::scene::Scene,
    source: Rc<OBB>,
) -> Result<crate::scene::ObjectId, &'static str> {
    let id = scene.insert(prepared(&source)?)?;
    scene
        .objects_mut()
        .iter_mut()
        .find(|r| r.id == id)
        .unwrap()
        .original = Some(session_rust::Geometry::OBB(source));
    Ok(id)
}
pub fn box_geometry(bounds: &crate::bounds::Bounds) -> OBB {
    let c = bounds.centre();
    OBB::new(
        Point::new(c[0], c[1], c[2]),
        session_rust::Vector::x_axis(),
        session_rust::Vector::y_axis(),
        session_rust::Vector::z_axis(),
        session_rust::Vector::new(
            (bounds.max[0] - bounds.min[0]) * 0.5,
            (bounds.max[1] - bounds.min[1]) * 0.5,
            (bounds.max[2] - bounds.min[2]) * 0.5,
        ),
    )
}
