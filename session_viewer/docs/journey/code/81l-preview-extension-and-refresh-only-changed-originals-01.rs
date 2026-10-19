        best
    }
    pub fn follow(
        &mut self,
        scene: &Scene,
        screen: &crate::snap_math::Screen,
        at: [f64; 2],
    ) -> Result<(), String> {
        if !self.curves {
            return Ok(());
        }
        let best = self.nearest(screen, at);
        let Some((_, index, end)) = best else {
            self.hover = None;
            self.last = None;
            self.error.clear();
            return Ok(());
        };
        let owner = self.candidates[index].0.clone();
        if self.last == Some((owner.id, end))
            && owner.valid(scene).is_ok()
            && self
                .boundaries
                .iter()
                .all(|boundary| boundary.valid(scene).is_ok())
        {
            return Ok(());
        }
        self.last = Some((owner.id, end));
        self.hover = None;
        self.error.clear();
        let result = self
            .reach(scene, &owner, end)
            .and_then(|reach| extend_reach::extended(&owner.source, end, &reach));
        match result {
            Ok(grown) => {
                let tail = grown
                    .tail
                    .iter()
                    .map(|p| {
                        let p = p.transformed(&owner.model);
                        [p[0], p[1], p[2]]
                    })
                    .collect();
                self.hover = Some(Hover {
                    owner,
                    end,
                    geometry: grown.geometry,
                    tail,
                })
            }
            Err(error) => self.error = error,
        }
        Ok(())
    }
    pub fn refreshed(&mut self, scene: &Scene, id: ObjectId) -> Result<(), String> {
        let fresh = Owner::capture(scene, id).map_err(str::to_owned)?;
        let curve = source_cut::as_curve(&fresh.source).ok_or("Edited original is not a curve")?;
        let points = source_cut::samples(&curve, 32)
            .iter()
            .map(|p| {
                let p = p.transformed(&fresh.model);
                [p[0], p[1], p[2]]
            })
            .collect::<Vec<_>>();
        let bounds = bounds(&points);
        self.candidates.retain(|(owner, ..)| owner.id != id);
        self.candidates.push((fresh, points, bounds));
        self.hover = None;
        self.last = None;
        Ok(())
    }
    pub fn prompt(&self) -> String {
        if self.asking {
            "Extend · type a distance above zero · Esc cancels".into()
        } else if self.curves {
            if !self.error.is_empty() {
                self.error.clone()
            } else {
                "Extend · click near a curve end · Enter finishes · Esc cancels".into()
            }
        } else {
            "Extend · pick boundaries · Enter accepts · Esc cancels".into()
        }
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
