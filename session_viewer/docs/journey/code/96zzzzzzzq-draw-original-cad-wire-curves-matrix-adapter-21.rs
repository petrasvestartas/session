            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.sheets.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.matrices.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }

    pub(crate) fn append_document(&mut self,source:&Scene,name:&str,place:&session_rust::Xform,point_px:f32)->Result<(),&'static str> {
