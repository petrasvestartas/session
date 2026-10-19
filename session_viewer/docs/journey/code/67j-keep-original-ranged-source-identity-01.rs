    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

/// Original GUID and name precede the large coordinate array in the ranged layout.
pub fn cloud_identity(head:&[u8])->Option<(String,String)> {
    let mut at=0usize;let objects_end=descend_message(head,&mut at,None,3)?;let end=descend_message(head,&mut at,Some(objects_end),8)?;
    let mut guid=String::new();let mut name=String::new();
    while (at as u64)<end {
        let f=field_at(head.get(at..)?,at as u64,end)?;
        if f.field==3 {return (!guid.is_empty()).then_some((guid,name));}
        if f.wire==2&&(f.field==1||f.field==2)&&f.value<=NAME_BYTES {
            let value=std::str::from_utf8(head.get(f.body as usize..f.next as usize)?).ok()?.to_owned();
            if f.field==1 {if !guid.is_empty() {return None;}guid=value;} else {name=value;}
        }
        at=usize::try_from(f.next).ok()?;
    }None
}

