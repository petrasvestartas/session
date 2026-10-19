        Ok(result)
    }
    fn open_names(&self) -> String {
        match self.open.len() {
            0..=3 => self.open.join(", "),
            count => format!("{} and {} more", self.open[..3].join(", "), count - 3),
        }
    }
    pub fn answer(
        &self,
        kind: Kind,
        unit: Unit,
        centre: [f64; 3],
    ) -> Result<(Mark, String), String> {
        let text = crate::measure_math::to_text;
        let (name, power, one) = match kind {
            Kind::Length => ("Length", 1, "curve"),
            Kind::Area => ("Area", 2, "object"),
            Kind::Volume => ("Volume", 3, "object"),
            Kind::BoundingLengths => unreachable!(),
        };
        if self.count == 0 {
            return Err(if self.open.is_empty() {
                format!("{name}: no supported original is selected")
            } else {
                format!(
                    "Volume needs closed objects; open or inconsistently wound: {}",
                    self.open_names()
                )
            });
        }
        let value = format!("{} {}", text(self.total), unit.suffix(power));
        let refused = if self.open.is_empty() {
            String::new()
        } else {
            format!(" · open, not measured: {}", self.open_names())
        };
        let message = format!(
            "{name} {value} · {}{refused}{}",
            crate::measure_math::plural(self.count, one),
            crate::measure_math::skipped_text(self.skipped)
        );
        Ok((
            Mark {
                points: vec![centre],
                label: value,
            },
            message,
        ))
    }
}
pub fn selection(
    scene: &Scene,
    ids: &[ObjectId],
    face: Option<(ObjectId, usize)>,
    kind: Kind,
    unit: Unit,
) -> Result<(Mark, String), String> {
    for id in ids {
        if scene
            .objects()
            .iter()
            .any(|r| r.id == *id && r.geometry().is_none())
        {
            return Err("Reload editable originals before measuring".into());
        }
    }
    let bounds = selected_bounds(scene, ids)?;
    let text = crate::measure_math::to_text;
    if matches!(kind, Kind::BoundingLengths) {
        let d = std::array::from_fn::<_, 3, _>(|i| bounds.max[i] - bounds.min[i]);
        let value = format!(
            "X {} · Y {} · Z {} {}",
            text(d[0]),
            text(d[1]),
            text(d[2]),
            unit.suffix(1)
        );
        return Ok((
            Mark {
                points: vec![bounds.centre()],
                label: value.clone(),
            },
            format!("Length BoundingBox · {value}"),
        ));
    }
    Total::collect(scene, ids, face, kind)?.answer(kind, unit, bounds.centre())
}
