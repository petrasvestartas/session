        Self {document:TextDocument::new(),cache,atlas,viewport,raster:SwashCache::new(),renderer,prepared:None,preparations:0,skips:0,resets:0,format}
    }
    pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2],density:f32)->anyhow::Result<()> {
        anyhow::ensure!(size.iter().all(|v|*v>0)&&density.is_finite()&&density>0.,"Invalid text viewport");
        let key=(self.document.revision,self.document.font_revision,size,density.to_bits());if self.prepared==Some(key) {self.skips+=1;return Ok(());}
        if self.prepared.is_some_and(|old|old.1!=key.1) {
            self.atlas=TextAtlas::with_color_mode(device,queue,&self.cache,self.format,glyphon::ColorMode::Accurate);self.renderer=TextRenderer::new(&mut self.atlas,device,Default::default(),None);self.raster=SwashCache::new();self.resets+=1;
        }
        self.viewport.update(queue,Resolution {width:size[0],height:size[1]});
        let areas=self.document.runs.iter().filter_map(|run| {
            let TextPlacement::Screen {left,top}=run.label.placement else {return None;};let bounds=run.label.clip.map_or([0.,0.,size[0] as f32/density,size[1] as f32/density],|clip|clip);
            Some(TextArea {buffer:&run.buffer,left:left*density,top:top*density,scale:density,bounds:TextBounds {left:(bounds[0]*density).floor() as i32,top:(bounds[1]*density).floor() as i32,right:(bounds[2]*density).ceil() as i32,bottom:(bounds[3]*density).ceil() as i32},default_color:Color::rgba(run.label.ink_color()[0],run.label.ink_color()[1],run.label.ink_color()[2],run.label.ink_color()[3]),custom_glyphs:&[]})
        });
        self.renderer.prepare(device,queue,&mut self.document.fonts,&mut self.atlas,&self.viewport,areas,&mut self.raster)?;self.prepared=Some(key);self.preparations+=1;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>)->anyhow::Result<()> {self.renderer.render(&self.atlas,&self.viewport,pass)?;Ok(())}
    pub fn raster_usage(&self)->[usize;2] {self.raster.image_cache.values().flatten().fold([0,0],|[n,bytes],image|[n+1,bytes+image.data.capacity()])}
}
