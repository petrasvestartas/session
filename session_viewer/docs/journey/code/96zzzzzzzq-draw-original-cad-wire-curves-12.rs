        for row in scene.objects().iter().filter(|r|scene.visible(r.id)){
            let colour=scene.view.edges.get(&row.id).map(|c|[c[0]as f32/255.,c[1]as f32/255.,c[2]as f32/255.,1.]);
            let wires=crate::cad_display::wires(row,anchor,colour,0.);sources.path_ids.extend(std::iter::repeat_n(id(row.id,sub_id::OBJECT_INK),wires.len()));sources.wire_segments.extend(wires);
        }
        let vertex_end=