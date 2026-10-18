        Ok(result)
    }
    pub fn text(&mut self, scene: &Scene, text: &str) -> Result<bool, String> {
        let words = text.split_whitespace().collect::<Vec<_>>();
        if words.is_empty() {
            if self.curves {
                return Ok(true);
            }
            self.start(scene)?;
            return Ok(false);
        }
        if words.len() == 1 && words[0].eq_ignore_ascii_case("Distance") {
            self.asking = true;
            return Ok(false);
        }
        let value = match words.as_slice() {
            [value] => *value,
            [name, value] if name.eq_ignore_ascii_case("Distance") => *value,
            _ => return Err("Extend Distance needs one positive length".into()),
        };
        let value = value
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && *v > 0.)
            .ok_or("The distance must be above zero")?;
        let mut next = self.clone();
        next.distance = Some(value);
        next.hover = None;
        next.error.clear();
        next.asking = false;
        next.start(scene)?;
        *self = next;
        Ok(false)
    }
    pub fn reach(&self, scene: &Scene, owner: &Owner, end: End) -> Result<Reach, String> {
        owner.valid(scene).map_err(str::to_owned)?;
        let back = owner
            .model
            .inverse()
            .ok_or("Source placement is singular")?;
        if let Some(distance) = self.distance {
            let (tip, direction) =
                extend_reach::tip(&owner.source, end).ok_or("This original has no free end")?;
            let from = tip.transformed(&owner.model);
            let mut along = &(&tip + &direction).transformed(&owner.model) - &from;
            if !along.normalize_self() {
                return Err("The end direction is singular".into());
            }
            let to = (&from + &(&along * distance)).transformed(&back);
            return Ok(Reach::Distance(to.distance(&tip, None)));
        }
        let mut cutters = Vec::new();
        let mut meshes = Vec::new();
        for (boundary, cutter) in &self.cutters {
            if boundary.id == owner.id {
                continue;
            }
            boundary.valid(scene).map_err(str::to_owned)?;
            if let Some(cutter) = cutter.moved(&back) {
                cutters.push(cutter)
            }
        }
        for (boundary, mesh) in &self.meshes {
            if boundary.id == owner.id {
                continue;
            }
            boundary.valid(scene).map_err(str::to_owned)?;
            let to = &boundary
                .model
                .inverse()
                .ok_or("Boundary placement is singular")?
                * &owner.model;
            meshes.push((mesh.clone(), to))
        }
        Ok(Reach::Boundaries { cutters, meshes })
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
