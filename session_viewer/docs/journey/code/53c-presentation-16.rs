pub fn collect(scene: &Scene) -> Vec<Target> {
    let mut out = Vec::new();
    for row in scene.points().iter().filter(|r| scene.visible(r.id)) { let p = row.point(); point(&mut out,row.id,[p[0],p[1],p[2]],Kind::End); }
    for row in scene.lines().iter().filter(|r| scene.visible(r.id)) { let ends = row.endpoints().map(|p| [p[0],p[1],p[2]]); polyline(&mut out,row.id,&ends); }
    for row in scene.paths().iter().filter(|r| scene.visible(r.id)) {
        let points: Vec<_> = row.points().iter().map(|p| [p[0],p[1],p[2]]).collect();
        if let crate::chain::Source::Curve(curve, _) = &row.prepared.source {
