
pub fn save(scene:&Scene)->Result<Vec<u8>,String> {
    if !scene.streams().is_empty() {return Err("This scene contains streamed sources. Open complete source documents before saving an editable session.".into());}
    let mut namespaces=HashSet::new();let mut documents=Vec::new();let mut bytes_used=0usize;
    let mut metadata=Metadata {created_doc:None,current_layer:None,documents:vec![],hidden:vec![],locked:vec![],colors:vec![],edge_colors:Some(vec![]),texts:vec![],groups:vec![]};
