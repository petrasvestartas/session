        self.sources.insert(url.into(),source);self.sizes.insert(url.into(),bytes.len());self.pending=true;Ok(true)
    }
    pub fn source_points(&self,url:&str)->u32 {self.sources.get(url).map_or(0,|scene|scene.streams().iter().map(|row|row.prepared.loaded.get()).sum())}
    pub fn streamed_points(&self)->u32 {self.sources.keys().fold(0u32,|sum,url|sum.saturating_add(self.source_points(url)))}
    pub fn stream(&mut self,url:&str,source:Rc<crate::range_cloud::Prepared>)->Result<bool,String> {
        let revision=source.source.fields.revision.clone();
        if self.cached(url)&&revision.is_some()&&self.etag(url)==revision {return Ok(false);}
        let mut scene=Scene::empty();scene.insert_stream(source.clone(),session_rust::Xform::identity(),0.).map_err(str::to_owned)?;
        // Range revisions identify remote originals; sampled page bytes cannot prove whole-file byte equality.
        self.revisions.changed(url,revision,source.source.guid.as_bytes());
        self.sources.insert(url.into(),scene);self.sizes.remove(url);self.pending=true;Ok(true)
    }
    pub fn replacement(&mut self,previous:&Scene)->Result<Option<Scene>,String> {
        if !self.pending {return Ok(None);}
