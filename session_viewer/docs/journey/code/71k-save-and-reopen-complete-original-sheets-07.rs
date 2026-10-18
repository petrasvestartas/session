pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if scene.objects().iter().any(|row|row.surface.is_some()||row.cad.is_some()) {return Err("This checkpoint cannot save CAD surface objects yet");}
    if !scene.sheets().is_empty() {return Err("Use the viewer archive to save original sheets");}
    if !scene.clouds().is_empty() {return Err("This checkpoint cannot save cloud objects yet");}
    if !scene.texts().is_empty() {return Err("This checkpoint cannot save scene text objects yet");}
