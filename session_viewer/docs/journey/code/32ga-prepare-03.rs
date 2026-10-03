pub struct PreparedReload {
    pub key: ReloadKey,
    pub document: Rc<session_rust::Session>,
}

impl PreparedReload {
    pub fn new(key: ReloadKey, bytes: &[u8]) -> Result<Self, &'static str> {
        let document = crate::document::restore(bytes, &key.origin)?;
        Ok(Self { key, document })
    }

    pub fn geometry(&self, guid: &str) -> Option<&Rc<session_rust::Mesh>> {
        self.document.objects.meshes.iter().find(|mesh| mesh.guid() == guid)
    }

    pub fn validates(&self, row: &crate::scene::Object) -> bool {
        let Some(mesh) = self.geometry(&row.metadata.source_guid) else { return false; };
        let metadata = &row.metadata;
        metadata.name == mesh.name && metadata.is_visible == mesh.is_visible && metadata.is_locked == mesh.is_locked
    }
}

#[cfg(test)]
mod tests {
