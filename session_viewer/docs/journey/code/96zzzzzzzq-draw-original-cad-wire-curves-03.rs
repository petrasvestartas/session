let mut data=crate::cad_face::prepare(&boundary)?;
    let mut wires=Vec::new();let mut seen=std::collections::HashSet::new();for(edge,chain)in boundary.chains.iter().enumerate(){let e=&source.m_edges[edge];if chain.is_some()||e.degenerated||e.curve_3d_index<0||!seen.insert(e.curve_3d_index){continue;}
        let points=crate::curve::sample(&source.m_curves_3d[e.curve_3d_index as usize])?;if data.vertices.len()+points.len()>1<<20{return Err("CAD wire sample budget exceeded")}
        for p in &points{data.points.push(session_rust::Point::new(p[0],p[1],p[2]));data.vertices.push([p[0]as f32,p[1]as f32,p[2]as f32,0.,0.,0.]);data.normals.push([0.,0.,1.]);}wires.push(Boundary{edge,points});
    }