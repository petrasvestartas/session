    pub fn usage(&self) -> [u64;4] { [self.vertices.size()+self.uniform.size(),self.tile.as_ref().map_or(0,|t|u64::from(t.size[0])*u64::from(t.size[1])*36),self.allocations,self.writes] }
    pub fn extent(&self) -> Option<[u32;2]> { self.tile.as_ref().map(|t|t.size) }

    // A small independent depth buffer lets handles overlap correctly while remaining visible over the entire scene.
    pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,layout:Option<crate::widget_layout::Layout>) {
        self.visible=false;
        let Some(layout)=layout else { self.clear(); return; };
        if self.tile.as_ref().is_none_or(|tile| tile.size!=layout.extent) { self.tile=Some(Tile::new(device,&self.texture_layout,layout.extent)); self.allocations+=1; }
        let bytes:Vec<u8>=layout.uniform.into_iter().flat_map(f32::to_ne_bytes).collect();
        queue.write_buffer(&self.uniform,0,&bytes); self.writes+=1; self.visible=true;
    }

}

impl Drop for Widget { fn drop(&mut self) {self.vertices.destroy();self.uniform.destroy();} }
