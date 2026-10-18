}
impl Draft {
    pub fn pick(&mut self, scene: &Scene, id: Option<ObjectId>) -> Result<(), String> {
        if self.curves {
            return Err("Click near a curve end".into());
        }
        let id = id.ok_or("Pick a boundary original")?;
        if let Some(at) = self.boundaries.iter().position(|o| o.id == id) {
            self.boundaries.remove(at);
            return Ok(());
        }
        let owner = Owner::capture(scene, id).map_err(str::to_owned)?;
        if source_cut::as_curve(&owner.source).is_none()
            && source_cut::as_plane(&owner.source).is_none()
            && !matches!(owner.source, Geometry::Mesh(_) | Geometry::BRep(_))
        {
            return Err("Stop at a line, polyline, curve, planar surface, mesh or BRep".into());
        }
        self.boundaries.push(owner);
        Ok(())
    }
    pub fn candidates(scene: &Scene) -> Vec<(Owner, Vec<[f64; 3]>, [[f64; 3]; 2])> {
        let mut candidates = Vec::new();
        let mut count = 0;
        for id in scene
            .lines()
            .iter()
            .map(|r| r.id)
            .chain(scene.paths().iter().map(|r| r.id))
        {
            let Ok(owner) = Owner::capture(scene, id) else {
                continue;
            };
            let Some(curve) = source_cut::as_curve(&owner.source) else {
                continue;
            };
            let points = source_cut::samples(&curve, 32)
                .iter()
                .map(|p| {
                    let p = p.transformed(&owner.model);
                    [p[0], p[1], p[2]]
                })
                .collect::<Vec<_>>();
            if count + points.len() > 200_000 {
                break;
            }
            count += points.len();
            let bounds = bounds(&points);
            candidates.push((owner, points, bounds));
        }
        candidates
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
