    for head in &heads.rows {
        head_bytes(head, &mut out.heads);
    }
    out.mesh_from=out.owners.len();
    crate::production_mesh_ink::append(scene,&mut out,sources,anchor,selected)?;
    for(index,segment)in out.segments.chunks_exact(52).enumerate(){
        let row=u32::from_ne_bytes(segment[28..32].try_into().unwrap())as usize;let flags=u32::from_ne_bytes(out.objects[row*96+80..row*96+84].try_into().unwrap());
        let kind=usize::from(index<out.mesh_from)+2*usize::from(flags&1!=0);let runs=&mut out.runs[kind];let start=index as u32*6;
        if let Some(last)=runs.last_mut().filter(|r|r.end==start){last.end+=6;}else{runs.push(start..start+6);}
    }
    if out.ids.len() != out.owners.len() {
        return Err("Joined source IDs do not match production rows");
    }
