
/// The open number box: its title, unit and place.
#[cfg(target_arch = "wasm32")]
fn number_box(state: &State) -> serde_json::Value {
    serde_json::json!(state.number_prompt().map(|prompt| {
        serde_json::json!({"title": prompt.title, "unit": prompt.unit, "at": prompt.at})
    }))
}

/// A surface's degrees, control counts, closure and world middle; a BRep's faces and solidity.
#[cfg(target_arch = "wasm32")]
fn shape(state: &State, row: u32) -> serde_json::Value {
    let place = state.scene.placement_of(row).unwrap_or_default();

    match state.scene.geometry(row) {
        Some(session_rust::Geometry::NurbsSurface(surface)) => {
            let middle = surface
                .domain(0)
                .zip(surface.domain(1))
                .and_then(|(u, v)| surface.point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0));
            serde_json::json!({
                "kind": "NurbsSurface",
                "degree": [surface.degree(0), surface.degree(1)],
                "cv_count": [surface.cv_count(0), surface.cv_count(1)],
                "closed": [surface.is_closed(0), surface.is_closed(1)],
                "mid": middle.map(|p| {
                    let p = p.transformed(&place);
                    [p[0], p[1], p[2]]
                }),
            })
        }
        Some(session_rust::Geometry::BRep(brep)) => serde_json::json!({
            "kind": "BRep",
            "faces": brep.face_count(),
            "solid": brep.is_solid(),
        }),
        Some(geometry) => serde_json::json!({ "kind": kind(geometry) }),
        None => serde_json::Value::Null,
    }
}
