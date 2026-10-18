        let vertex_count=u32::try_from(source.vertices().len()).map_err(|_|"Arena vertex count")?;
        let index_count=u32::try_from(source.indices().len()).map_err(|_|"Arena index count")?;
        let vertex_end=u32::try_from(self.vertices.len()/20).map_err(|_|"Arena vertex range")?;
        let index_end=u32::try_from(self.indices.len()).map_err(|_|"Arena index range")?;
        let vertices=crate::arena_spans::plan(&self.free_vertices,vertex_count,vertex_end).ok_or("Arena vertex range")?;
        let indices=crate::arena_spans::plan(&self.free_indices,index_count,index_end).ok_or("Arena index range")?;
        let byte_end=usize::try_from(u64::from(vertices.range.end)*20).map_err(|_|"Arena vertex bytes")?;
        let rebased=source.indices().iter().map(|i|i.checked_add(vertices.range.start).ok_or("Arena index address")).collect::<Result<Vec<_>,_>>()?;
        let bytes=crate::packed_vertex::bytes(source);
        let range=MeshRange {vertices:vertices.take(&mut self.free_vertices),indices:indices.take(&mut self.free_indices)};
        self.vertices.resize(self.vertices.len().max(byte_end),0);self.indices.resize(self.indices.len().max(range.indices.end as usize),0);
        self.vertices[range.vertices.start as usize*20..byte_end].copy_from_slice(&bytes);
        self.indices[range.indices.start as usize..range.indices.end as usize].copy_from_slice(&rebased);
        self.sources.insert(key,(Rc::downgrade(source),range.clone()));Ok(range)
    }
    pub(crate) fn retire(&mut self,key:usize)->bool {
        let Some((_,range))=self.sources.remove(&key)else{return false};
        let vertices=crate::arena_spans::retire(&mut self.free_vertices,range.vertices,(self.vertices.len()/20)as u32);
        let indices=crate::arena_spans::retire(&mut self.free_indices,range.indices,self.indices.len()as u32);
        self.vertices.truncate(vertices as usize*20);self.indices.truncate(indices as usize);true
    }
