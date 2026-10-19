let slots=SlotMap::new(&keys);
    let topology=mesh_topology(source,&keys,&points,&slots);
    let mut vertices=vec![u32::MAX;keys.len()];