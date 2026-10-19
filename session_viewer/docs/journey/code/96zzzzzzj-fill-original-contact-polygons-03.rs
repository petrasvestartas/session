fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    polygon_normal(vs.len(), |i| vpos[slots.slot(vs[i])])
}

/// Sort key of a face.
