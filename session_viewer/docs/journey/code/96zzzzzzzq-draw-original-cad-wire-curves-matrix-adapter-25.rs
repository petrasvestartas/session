        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);self.sheets.retain(|row|row.id!=id);self.matrices.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }
