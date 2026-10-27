            }
        }
    }
    fn update_inside(&self,queue:&wgpu::Queue,matrix:[f32;16]){
        let eye=session_rust::Xform::from_matrix(matrix.map(f64::from)).eye();
        let eye=std::array::from_fn::<_,3,_>(|i|eye[i]+self.anchor[i]);
        for((index,bounds),previous)in self.rows.inside.iter().zip(self.inside.borrow_mut().iter_mut()){
            let inside=(0..3).all(|i|eye[i]>=bounds.min[i]&&eye[i]<=bounds.max[i]);
            if inside!=*previous{let at=*index as usize*96+80;let flags=u32::from_ne_bytes(self.rows.objects[at..at+4].try_into().unwrap())|if inside{4}else{0};queue.write_buffer(&self.objects,at as u64,&flags.to_ne_bytes());*previous=inside;}
        }
    }
    pub fn glyph_usage(&self)->[u64;2]{let bytes=self.rows.dots.len()+self.rows.spheres.len();[(bytes/48)as u64,bytes as u64]}
    pub fn draw_spheres(&self,pass:&mut wgpu::RenderPass<'_>){if self.vertices_visible{if let Some(frame)=self.frame.borrow().as_ref(){self.spheres.draw(pass,&frame.groups,false);}}}
    pub fn draw_dots(&self,pass:&mut wgpu::RenderPass<'_>){if self.points_visible{if let Some(frame)=self.frame.borrow().as_ref(){self.dots.draw(pass,&frame.groups,false);}}}
    fn line_uniform(&self,matrix:[f32;16],size:[u32;2],density:f32,opacity:f32)->Vec<u8> {
        let mut bytes=self.settings.line(matrix,size,density,opacity,self.anchor);
        bytes[76..80].copy_from_slice(&self.plane_size.to_ne_bytes());bytes
    }
    pub fn plane_usage(&self)->[u64;2]{[(self.rows.planes.len()/48)as u64,self.rows.planes.len()as u64]}
