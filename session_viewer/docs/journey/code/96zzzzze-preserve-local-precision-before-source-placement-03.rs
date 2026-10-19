        let [start,end]=self.endpoints_at(anchor);
        crate::stroke::Stroke {start:start.to_f32(),end:end.to_f32(),