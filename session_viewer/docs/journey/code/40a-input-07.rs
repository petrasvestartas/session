    if !(0..=3).contains(&mesh.color_mode) { return Err("Unknown mesh colour mode"); }
    for colours in [&mesh.pointcolors_rgba, &mesh.facecolors_rgba, &mesh.linecolors_rgba] {
        if colours.len() % 4 != 0 || colours.len() > 160_000 || colours.iter().any(|v| !v.is_finite()) {
            return Err("Mesh colours need bounded complete finite RGBA records");
        }
    }
    if mesh.objectcolor.as_ref().is_some_and(|c| [c.r, c.g, c.b, c.a].iter().any(|v| !v.is_finite())) {
        return Err("Mesh object colour must be finite");
    }