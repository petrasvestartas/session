        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert((source,0),Batch {geometry,count:records.len()as u32,first_slot});
        }
