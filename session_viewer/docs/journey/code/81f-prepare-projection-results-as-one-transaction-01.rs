        Ok(plane)
    }
    pub fn prepare(
        &self,
        scene: &Scene,
        plane: &Plane,
    ) -> Result<(Vec<(ObjectId, Geometry)>, String), String> {
        let mut edits = Vec::new();
        let mut collapsed = 0;
        let mut unsupported = self.unsupported;
        for owner in &self.targets {
            owner.valid(scene).map_err(str::to_owned)?;
            match crate::project_geometry::projected(&owner.source, &owner.model, plane) {
                Ok(geometry) => edits.push((owner.id, geometry)),
                Err(crate::project_geometry::Skip::Collapsed) => collapsed += 1,
                Err(crate::project_geometry::Skip::Unsupported) => unsupported += 1,
            }
        }
        if edits.is_empty() {
            return Err(format!(
                "No objects projected · {collapsed} would collapse · {unsupported} unsupported"
            ));
        }
        let message = format!(
            "Projected {} · {collapsed} would collapse · {unsupported} unsupported",
            edits.len()
        );
        Ok((edits, message))
    }
    pub fn prompt(&self) -> String {
        match self.points.len() {
            0 => "Project To Plane · Plane origin · Esc cancels",
            1 => "Project To Plane · Point on the plane’s x axis · Esc cancels",
            _ => "Project To Plane · Third point on the plane · Esc cancels",
        }
        .into()
    }
}
