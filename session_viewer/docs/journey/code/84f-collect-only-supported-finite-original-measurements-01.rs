    BoundingLengths,
}
pub struct Total {
    total: f64,
    count: usize,
    skipped: usize,
    open: Vec<String>,
}
impl Total {
    pub fn collect(
        scene: &Scene,
        ids: &[ObjectId],
        face: Option<(ObjectId, usize)>,
        kind: Kind,
    ) -> Result<Self, String> {
        let mut result = Self {
            total: 0.,
            count: 0,
            skipped: 0,
            open: Vec::new(),
        };
        for id in ids {
            let Some(source) = crate::source_edit::original(scene, *id) else {
                if scene
                    .objects()
                    .iter()
                    .any(|r| r.id == *id && r.geometry().is_none())
                {
                    return Err("Reload editable originals before measuring".into());
                }
                result.skipped += 1;
                continue;
            };
            let model = scene
                .placement(*id)
                .ok_or("Selected placement is missing")?;
            let value = match kind {
                Kind::Length => crate::measure_length::compute_length(&source, &model),
                Kind::Area => crate::measure_area::compute_area(
                    &source,
                    &model,
                    face.filter(|(owner, _)| owner == id).map(|(_, face)| face),
                ),
                Kind::Volume => match crate::measure_volume::compute_volume(&source, &model) {
                    Some(Ok(value)) => Some(value),
                    Some(Err(_)) => {
                        result.open.push(source.name().to_owned());
                        continue;
                    }
                    None => None,
                },
                Kind::BoundingLengths => unreachable!(),
            };
            if let Some(value) = value {
                if !value.is_finite() {
                    return Err("Measurement exceeds numerical range".into());
                }
                result.total += value;
                result.count += 1;
            } else {
                result.skipped += 1;
            }
        }
        Ok(result)
    }
}
