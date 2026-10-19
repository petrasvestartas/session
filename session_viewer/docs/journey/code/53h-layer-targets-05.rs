pub fn apply(scene:&mut Scene,key:&str,property:Property)->Result<(), &'static str> {
    let rows=hierarchy::rows(scene);let index=rows.iter().position(|row|row.key==key).ok_or("This layer row is gone")?;let row=&rows[index];
    if matches!(property,Property::Hidden(true)) && crate::layer_edit::holds_current(scene,key) {return Err("The current layer cannot be hidden");}
    let view=Rc::make_mut(&mut scene.view);
    match property {
