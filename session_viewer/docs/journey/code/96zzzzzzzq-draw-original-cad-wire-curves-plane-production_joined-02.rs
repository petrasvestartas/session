            }
        }
    }
    for row in scene.objects().iter().filter(|r|scene.visible(r.id)) {
        let Some(session_rust::Geometry::Plane(source))=&row.original else {continue;};
        if crate::clipping_geometry::is_clipping(source) {continue;}
        let index=out.object_ids.len() as u32;
        let mut rows=crate::production_plane::PlaneRows::default();
        let result=crate::production_plane::walk_plane(&mut rows,source,index);
        add(&mut out,row.id,&row.model,result.flags);
        for plane in &rows.rows {crate::production_plane::bytes(plane,&mut out.planes);}
    }
    for row in walk::joined_rows(
        &segments.ribbons,
        &segments.ribbon_chains,
