        self.clock=self.clock.checked_add(1)?;let entry=self.entries.get_mut(key)?;entry.touch=self.clock;Some(entry.value.clone())
    }
    pub fn reserve(&mut self,bytes:usize)->Result<Charge,&'static str> {
        if bytes>self.budget.limit {return Err("One page exceeds its memory budget");}
        let mut victims=self.entries.iter().filter(|(_,entry)|Rc::strong_count(&entry.value)==1).map(|(key,entry)|(entry.touch,key.clone(),entry.value.bytes())).collect::<Vec<_>>();victims.sort_by_key(|v|v.0);
        let mut room=self.budget.available();let mut chosen=Vec::new();
        for (_,key,size) in victims {if room>=bytes {break;}room=room.checked_add(size).ok_or("Page memory accounting overflow")?;chosen.push(key);}
        if room<bytes {return Err("Page memory is still held by a renderer or pending reader");}
        for key in chosen {self.entries.remove(&key);}
        self.budget.reserve(bytes)
    }
    pub fn hold(&self,value:T,mut charge:Charge)->Result<Rc<Held<T>>,&'static str> {
        if !Rc::ptr_eq(&charge.budget,&self.budget) {return Err("Page charged to another cache budget");}
        charge.resize(value.bytes())?;Ok(Rc::new(Held {value,charge}))
    }
    pub fn insert(&mut self,key:K,value:T,charge:Charge)->Result<Rc<Held<T>>,&'static str> {
        let value=self.hold(value,charge)?;
        self.clock=self.clock.checked_add(1).ok_or("Page cache revision exhausted")?;
        self.entries.insert(key,Entry {value:value.clone(),touch:self.clock});Ok(value)
    }
    pub fn oldest(&self)->Option<K> {self.entries.iter().min_by_key(|(_,entry)|entry.touch).map(|(key,_)|key.clone())}
    pub fn remove(&mut self,key:&K)->bool {self.entries.remove(key).is_some()}
