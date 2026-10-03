        let id = self.insert(Mesh::from_kernel(&source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
