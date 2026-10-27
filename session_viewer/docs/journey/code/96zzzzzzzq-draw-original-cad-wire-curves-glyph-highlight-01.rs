        self.strokes.set(device, scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|crate::render_space::placed(&r.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}})))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len())).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device,std::iter::empty())?;
        self.markers.set_points(device,std::iter::empty())?;
        Ok(())
    }

