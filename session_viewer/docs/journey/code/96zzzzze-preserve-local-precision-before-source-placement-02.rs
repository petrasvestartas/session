        let line = &self.prepared.source;let model=crate::render_space::placed(&self.model,anchor);
        [model.transform_point(&line.start()),model.transform_point(&line.end())]