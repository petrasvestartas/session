        }
        edges.push(Boundary {edge:chain.edge,points});
    }
    display.parts.ink=crate::production_mesh_ink::brep(&boundary);
    let prepared=Prepared {source,edges,samples:data.samples,face_count:boundary.faces.len(),edge_ids,wires};
    let mesh=crate::prepared::PreparedMesh {geometry:Rc::new(proxy),display,source:None,model:Xform::identity()};Ok((prepared,mesh))
}
