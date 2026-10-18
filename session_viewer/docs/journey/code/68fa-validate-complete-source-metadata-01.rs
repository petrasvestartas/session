pub fn single_cloud(head:&[u8])->bool {
    let mut at=0u64;
    let objects=loop {
        let Some(raw)=head.get(at as usize..) else {return false;};let Some(field)=crate::stream::field_at(raw,at,u64::MAX) else {return false;};
        if field.field==3&&field.wire==2 {break field;}
        if field.field>2||field.wire!=2 {return false;}at=field.next;
    };
    at=objects.body;
    loop {
        let Some(raw)=head.get(at as usize..) else {return false;};let Some(field)=crate::stream::field_at(raw,at,objects.next) else {return false;};
        if field.field==8&&field.wire==2 {return field.next==objects.next;}
        if field.field>2||field.wire!=2 {return false;}at=field.next;
    }
}

pub fn metadata(raw:&[u8],guid:&str)->Result<(),String> {
    use prost::Message;
    let tail=session_rust::proto::Session::decode(raw).map_err(|_|"Invalid original source metadata")?;
    if tail.objects.is_some()||tail.definitions.is_some()||!tail.interactions.is_empty()||!tail.xforms.is_empty() {return Err("Placed or extended originals require complete document loading".into());}
    if let Some(root)=tail.tree.as_ref().and_then(|tree|tree.root.as_ref()) {let guids=[guid].into_iter().collect();crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    Ok(())
}
