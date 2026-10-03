        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        self.objects.push(Object { id, guid, metadata, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
