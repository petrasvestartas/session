        for row in self.objects.iter().filter(|row| self.control_owners.contains(&row.id) && self.visible(row.id)) {
            let Some(source) = crate::source_edit::original(self, row.id) else { continue };
            for control in crate::source_controls::Controls::from_geometry(&source).points {
                let local = session_rust::Point::new(control.position[0], control.position[1], control.position[2]);
                let center = row.model.transform_point(&local).to_f32();
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let display = crate::marker::Marker { center, diameter: 6., colour: [0., 0., 0., 1.] };
                markers.push(ControlMarker { parent: row.id, part: crate::pick_sources::Part::SourceControl(control.id), display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }