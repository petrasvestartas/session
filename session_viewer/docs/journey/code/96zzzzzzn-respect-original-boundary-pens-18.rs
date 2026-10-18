            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
