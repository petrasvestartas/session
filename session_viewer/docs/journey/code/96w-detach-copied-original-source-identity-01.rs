    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self.objects.iter_mut().find(|row| row.id == id) {
            let element = !instance && matches!(row.original, Some(session_rust::Geometry::Element(_)));
            if element {
                if let Some(session_rust::Geometry::Element(source)) = row.original.as_mut() {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element { Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone(); }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(