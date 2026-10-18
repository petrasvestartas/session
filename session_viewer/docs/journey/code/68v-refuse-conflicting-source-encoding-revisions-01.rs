    pub fn forget(&mut self,url:&str) {self.revisions.forget(url);}
    pub fn urls(&self)->Vec<String> {self.manifest.as_ref().map_or_else(Vec::new,|manifest|manifest.items.iter().map(|item|crate::publication_route::join(&self.base,&item.file)).collect())}
    pub fn encoding(&self,url:&str)->Result<crate::source_encoding::Encoding,String> {
        let manifest=self.manifest.as_ref().ok_or("Source manifest is missing")?;let mut encoding=None;
        for item in &manifest.items {if crate::publication_route::join(&self.base,&item.file)==url {
            let value=crate::source_encoding::Encoding::new(&item.encoding,item.size)?;
            if encoding.as_ref().is_some_and(|previous|previous!=&value) {return Err("Repeated source has conflicting encoding metadata".into());}encoding=Some(value);
        }}encoding.ok_or_else(||"Source URL is absent from its manifest".into())
    }
    pub fn manifest(&mut self,url:&str,etag:Option<String>,bytes:&[u8])->Result<bool,String> {
        if !self.revisions.changed(url,etag,bytes) {return Ok(false);}
        let manifest=match Manifest::parse(bytes) {Ok(value)=>value,Err(error)=>{self.revisions.forget(url);return Err(error);}};
        let mut formats=HashMap::new();
        for item in &manifest.items {
            let format=match crate::source_encoding::Encoding::new(&item.encoding,item.size) {Ok(format)=>format,Err(error)=>{self.revisions.forget(url);return Err(error);}};
            let key=crate::publication_route::join(&self.base,&item.file);
            if formats.insert(key,format.clone()).is_some_and(|previous|previous!=format) {self.revisions.forget(url);return Err("Repeated source has conflicting encoding metadata".into());}
        }
        self.manifest=Some(manifest);self.pending=true;let used=self.urls().into_iter().collect::<HashSet<_>>();self.sources.retain(|url,_|used.contains(url));self.sizes.retain(|url,_|used.contains(url));Ok(true)
    }
