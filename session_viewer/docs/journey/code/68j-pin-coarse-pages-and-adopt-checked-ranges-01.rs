        Ok(Rc::new(Self {source,nodes:Rc::new(nodes),bounds:Bounds {min,max},pages:RefCell::new(Cache::new(budget)),root:RefCell::new(None),generation:RefCell::new(Default::default()),loaded:std::cell::Cell::new(0)}))
    }
    pub fn pin_root(&self,page:Page,charge:crate::page_cache::Charge)->Result<(),&'static str> {
        let first=self.source.lod.first[0] as u32;let end=first+self.source.lod.count[0] as u32;
        if page.rows!=(first..end) {return Err("Pinned root page does not match its original node");}
        let page=self.pages.borrow().hold(page,charge)?;self.root.replace(Some(page));Ok(())
    }
    pub fn page(&self,first:u32)->Option<Rc<Held<Page>>> {self.pages.borrow_mut().get(&first)}
    pub fn pages(&self)->Vec<Rc<Held<Page>>> {
        let mut cache=self.pages.borrow_mut();let keys=cache.keys();let mut pages=keys.iter().filter_map(|key|cache.get(key)).collect::<Vec<_>>();
        if let Some(root)=self.root.borrow().as_ref() {if !pages.iter().any(|page|page.value.rows.start<=root.value.rows.start&&page.value.rows.end>=root.value.rows.end) {pages.push(root.clone());}}pages
    }
    pub fn adopt(&self,page:Page,charge:crate::page_cache::Charge)->Result<Rc<Held<Page>>,&'static str> {
        if page.rows.start>=page.rows.end||page.rows.end>self.source.fields.count {return Err("Page is outside its original source");}
        let mut cache=self.pages.borrow_mut();let ids=page.ids.iter().copied().collect::<std::collections::HashSet<_>>();for key in cache.keys() {if key!=page.rows.start {if let Some(existing)=cache.get(&key) {
            if page.rows.start<existing.value.rows.end&&existing.value.rows.start<page.rows.end {return Err("Source page ranges overlap");}
            if existing.value.ids.iter().any(|id|ids.contains(id)) {return Err("Original point IDs repeat across retained pages");}
        }}}
        let end=page.rows.end;let result=cache.insert(page.rows.start,page,charge)?;self.loaded.set(self.loaded.get().max(end));Ok(result)
    }
    pub fn position(&self,id:u32)->Option<[f64;3]> {
        // Look only through the retained pages; on-demand source queries load missing rows separately.
        for page in self.pages() {if let Some(i)=page.value.ids.iter().position(|value|*value==id) {return page.value.coordinates[i*3..i*3+3].try_into().ok();}}None
    }
}

