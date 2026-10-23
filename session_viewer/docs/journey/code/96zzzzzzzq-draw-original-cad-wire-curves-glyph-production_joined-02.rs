        add(&mut out,row.id,&row.model,result.flags);
        for plane in &rows.rows {crate::production_plane::bytes(plane,&mut out.planes);}
    }
    crate::production_glyph::append(scene,&mut out,anchor,selected);
    for row in walk::joined_rows(
        &segments.ribbons,
        &segments.ribbon_chains,
