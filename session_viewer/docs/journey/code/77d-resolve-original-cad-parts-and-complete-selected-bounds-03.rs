        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let points=if let Some(row)=self.lines.iter().find(|row|row.id==id){Some(row.endpoints().map(|p|[p[0],p[1],p[2]]).to_vec())}
            else if let Some(row)=self.points.iter().find(|row|row.id==id){let p=row.model.transform_point(&row.prepared.source);Some(vec![[p[0],p[1],p[2]]])}
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
