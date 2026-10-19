        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (sources,bytes)={let arena=arena.borrow();(arena.rows.source_count(),arena.rows.vertices.len()+arena.rows.indices.len()*4)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=usize::from(arena.rows.source_count()!=sources);
        self.uploaded_bytes+=(arena.rows.vertices.len()+arena.rows.indices.len()*4-bytes)as u64;