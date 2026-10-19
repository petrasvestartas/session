    fn control_sources(&self) -> Vec<(ObjectId, bool, crate::source_controls::Controls)> {
        let curves = self.paths.iter().filter(|row| row.controls_visible).map(|row| (row.id, true));
        let originals = self.objects.iter().filter(|row| self.control_owners.contains(&row.id)).map(|row| (row.id, false));
        curves.chain(originals).filter(|(id, _)| self.visible(*id)).filter_map(|(id, legacy)| {
            let source = crate::source_edit::original(self, id)?;
            let model = self.placement(id)?;
            let mut controls = crate::source_controls::Controls::from_geometry(&source);
            for control in &mut controls.points {
                let p = model.transform_point(&session_rust::Point::new(control.position[0], control.position[1], control.position[2]));
                control.position = [p[0], p[1], p[2]];
            }
            Some((id, legacy, controls))
        }).collect()
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for (owner, legacy, controls) in self.control_sources() {
            for control in controls.points {
                let center = control.position.map(|v| v as f32);
                if center.iter().any(|v| !v.is_finite()) { continue; }
                let part = match (legacy, control.id) {
                    (true, crate::source_controls::ControlId::Curve { point, .. }) => crate::pick_sources::Part::Control(point),
                    (_, id) => crate::pick_sources::Part::SourceControl(id),
                };
                markers.push(ControlMarker::new(owner, part, center));
            }
        }
        markers
    }

    pub fn control_nets(&self) -> Vec<(ObjectId, crate::stroke::Stroke)> {
        let mut nets = Vec::new();
        for (owner, _, controls) in self.control_sources() {
            for [a, b] in controls.links {
                let Some((a, b)) = controls.points.get(a).zip(controls.points.get(b)) else { continue };
                let start = a.position.map(|v| v as f32);
                let end = b.position.map(|v| v as f32);
                if start.iter().chain(&end).any(|v| !v.is_finite()) { continue; }
                nets.push((owner, crate::stroke::Stroke { start, end, colour: [102./255., 136./255., 204./255., 1.], width: 1. }));
            }
        }
        nets
    }

