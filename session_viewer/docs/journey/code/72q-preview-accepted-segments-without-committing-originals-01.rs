    pub fn set(&mut self, device: &wgpu::Device, draft: Option<&Draft>) -> Result<(), &'static str> {
        let yellow = [1., 0.75, 0.05, 1.]; let mut strokes = Vec::new(); let mut markers = Vec::new();
        if let Some(draft) = draft {
            for pair in draft.points.windows(2) {if pair[0]!=pair[1] {strokes.push(crate::stroke::Stroke {start:pair[0].map(|v|v as f32),end:pair[1].map(|v|v as f32),width:2.,colour:yellow});}}
            if let Some(cursor) = draft.cursor {
            markers.push(crate::marker::Marker { center: cursor.map(|v| v as f32), diameter: 8., colour: yellow });
            if let Some(&start) = draft.points.last().filter(|&&p| p != cursor) {
