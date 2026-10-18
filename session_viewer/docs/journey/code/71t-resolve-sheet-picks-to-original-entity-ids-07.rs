    if scene.instances.contains_key(&target.owner) {return Err("Detach the shared definition before editing its source geometry");}
    if !target.valid(scene) {return Err("Selected part is no longer editable");}
    if matches!(target.part,Part::SheetEntity(_)) {return Err("Sheet entities are display references; edit the original drawing");}
    if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
