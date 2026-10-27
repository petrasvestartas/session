        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.matrices.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let points=if let Some(row)=self.lines.iter().find(|row|row.id==id){Some(row.endpoints().map(|p|[p[0],p[1],p[2]]).to_vec())}
            else if let Some(row)=self.points.iter().find(|row|row.id==id){let p=row.model.transform_point(&row.prepared.source);Some(vec![[p[0],p[1],p[2]]])}
