        triangles: Vec::new(),
        edges,
        endpoints,
        dots:Vec::new(),dot_styles:Vec::new(),glyph_flags:0,dot_colours:Vec::new(),pens:Vec::new(),
    };
    Ok(crate::prepared::PreparedMesh {
        geometry: Rc::new(mesh),
