    pub fn acquire(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<Rc<MeshRange>,&'static str>{
        self.prune();let key=Rc::as_ptr(source)as usize;
        if let Some(range)=self.leases.get(&key).and_then(Weak::upgrade){return Ok(range);}
        let range=Rc::new(self.upload(ctx,source)?);self.leases.insert(key,Rc::downgrade(&range));Ok(range)
    }
    pub fn prune(&mut self){
        let retired=self.leases.iter().filter(|(_,range)|range.strong_count()==0).map(|(&key,_)|key).collect::<Vec<_>>();
        for key in retired {self.leases.remove(&key);self.rows.retire(key);}
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
    }
    pub fn reset(&mut self)