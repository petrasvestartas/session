            let ink_start=sources.mesh_ink.len()as u32;
            let line_frame=matches!(&row.original,Some(session_rust::Geometry::Plane(_)|session_rust::Geometry::OBB(_)));
            let derived=scene.forest.derived(row.id);