    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object { id, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
            model: session_rust::Xform::identity(), source: prepared.source });
        Ok(id)
    }
