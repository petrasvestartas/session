}

pub fn validate_document(bytes:&[u8])->Result<(),String> {
    let source=session_rust::proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&source)?;
    if let Some(definitions)=source.definitions.as_ref() {crate::source_validate::definitions(definitions,source.objects.as_ref().map_or(0,|objects|objects.instances.len()))?;}
    Ok(())
}

pub struct Bundle {pub documents:Vec<Vec<u8>>,pub metadata:Metadata}
impl Bundle {
    pub fn decode(bytes:&[u8])->Result<Self,String> {
        if bytes.len()>LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        let payload=bytes.strip_prefix(MAGIC).ok_or("Not a Session Viewer file")?;
        let archive=Archive::decode(payload).map_err(|e|e.to_string())?;
        let metadata:Metadata=serde_json::from_slice(&archive.metadata).map_err(|e|e.to_string())?;
        metadata.validate(archive.documents.len())?;
        for document in &archive.documents {validate_document(document)?;}
        Ok(Self {documents:archive.documents,metadata})
    }
    pub fn encode(&self)->Result<Vec<u8>,String> {
        self.metadata.validate(self.documents.len())?;
        let metadata=serde_json::to_vec(&self.metadata).map_err(|e|e.to_string())?;
        let size=self.documents.iter().try_fold(metadata.len(),|size,document|size.checked_add(document.len())).ok_or("Session size overflow")?;
        if size>LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        for document in &self.documents {validate_document(document)?;}
        let archive=Archive {documents:self.documents.clone(),metadata};
        let size=MAGIC.len().checked_add(archive.encoded_len()).ok_or("Session size overflow")?;
        if size>LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        let mut bytes=Vec::with_capacity(size);bytes.extend_from_slice(MAGIC);archive.encode(&mut bytes).map_err(|e|e.to_string())?;Ok(bytes)
    }
}