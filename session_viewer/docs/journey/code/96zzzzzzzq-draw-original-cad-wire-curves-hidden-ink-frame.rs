    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){
        let count=(self.data.len()as u64/self.stride)as u32;
        if count==0||self.ranges.visible(count).all(|range|range.is_empty()){return;}
        if let Some(ink)=&self.finite{ink.set(device,tables,depth)}
    }
