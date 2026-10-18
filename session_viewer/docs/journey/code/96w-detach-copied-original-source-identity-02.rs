if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.detach_copy_identity(copy);