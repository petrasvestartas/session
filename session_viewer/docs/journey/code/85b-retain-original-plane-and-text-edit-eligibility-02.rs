        Ok(())
    }
}

pub fn editable(scene:&Scene,id:ObjectId)->Result<(),&'static str>{
    if scene.pick_id(id.value())!=Some(id){return Err("Pick a visible unlocked original")}
    if scene.objects().iter().any(|r|r.id==id&&r.original.is_none()&&r.geometry().is_none()){return Err("Reload an editable original source")}
    Ok(())
}
