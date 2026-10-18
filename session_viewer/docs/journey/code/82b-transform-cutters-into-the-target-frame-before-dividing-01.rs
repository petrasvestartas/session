        Ok(())
    }
    pub fn prepare(&self, scene: &Scene) -> Result<(Vec<Geometry>, usize), String> {
        self.target.valid(scene).map_err(str::to_owned)?;
        let back = self
            .target
            .model
            .inverse()
            .ok_or("Target placement is singular")?;
        let mut tools = Vec::new();
        for owner in &self.cutters {
            owner.valid(scene).map_err(str::to_owned)?;
            if owner.model.inverse().is_none() {
                return Err("Cutter placement is singular".into());
            }
            let mut cutter = crate::split_geometry::curve(&owner.source)?;
            cutter.transform(&(&back * &owner.model));
            tools.push(cutter);
        }
        let (pieces, regions) =
            crate::split_geometry::divided(&self.target.source, self.face, &tools)?;
        Ok((
            crate::split_geometry::named(&self.target.source, pieces),
            regions,
        ))
    }
    pub fn prompt(&self) -> String {
        format!(
            "Split · {} cutter curves · Enter or Split confirms · Esc cancels",
            self.cutters.len()
        )
    }
}
