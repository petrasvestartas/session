    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    pub fn set_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();let lane=lane.get_or_insert_with(||crate::text_atlas::Lane::new(&self.device,&self.queue,self.format));lane.document.set_labels(labels)
    }
    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{let [rasters,bytes]=lane.raster_usage();[lane.document.shape_count,lane.preparations,lane.skips,rasters as u64,bytes as u64,lane.resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

