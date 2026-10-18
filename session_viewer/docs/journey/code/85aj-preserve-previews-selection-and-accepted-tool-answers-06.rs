        if let Some((point,word))=label{overlay.marks.push(crate::marker::Marker{center:point.map(|v|v as f32),diameter:6.,colour:red});overlay.label=Some((point,word));}overlay
    }

    pub fn trimming_picks(&self)->bool{self.trim.as_ref().is_some_and(|t|t.picks())}
    pub fn trimming_parts(&self)->bool{self.trim.as_ref().is_some_and(|t|!t.picks())}
