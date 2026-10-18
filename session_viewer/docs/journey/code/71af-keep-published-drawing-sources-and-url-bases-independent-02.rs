        let source=match crate::publication::prepare(&Scene::empty(),bytes,true,Some(Rc::new(crate::reload_url::ReloadUrl::new(url.into())))) {Ok(source)=>source,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        self.sources.insert(url.into(),source);self.sizes.insert(url.into(),bytes.len());self.pending=true;Ok(true)
    }
    pub fn source_segments(&self,url:&str)->u32 {self.sources.get(url).map_or(0,|scene|scene.sheets().iter().filter_map(|row|row.prepared.streamed.as_ref().map(|stream|stream.loaded.get())).sum())}
    pub fn streamed_segments(&self)->u32 {self.sources.keys().fold(0u32,|sum,url|sum.saturating_add(self.source_segments(url)))}
    pub fn stream_sheet(&mut self,url:&str,source:Rc<crate::sheet_source::Prepared>)->Result<bool,String> {
        let revision=source.streamed.as_ref().ok_or("Missing original drawing stream")?.source.fields.revision.clone();if self.cached(url)&&revision.is_some()&&self.etag(url)==revision {return Ok(false);}let mut scene=Scene::empty();scene.insert_sheet(source.clone()).map_err(str::to_owned)?;self.revisions.changed(url,revision,source.original.guid.as_bytes());self.sources.insert(url.into(),scene);self.sizes.remove(url);self.pending=true;Ok(true)
    }
    pub fn source_points(&self,url:&str)->u32 {self.sources.get(url).map_or(0,|scene|scene.streams().iter().map(|row|row.prepared.loaded.get()).sum())}
