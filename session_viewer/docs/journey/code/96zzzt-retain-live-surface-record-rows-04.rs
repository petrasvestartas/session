        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }