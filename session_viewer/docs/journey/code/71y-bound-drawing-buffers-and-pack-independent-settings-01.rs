    }
}
pub fn chunks(count: u32, bytes: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
    let size = u32::try_from((bytes / 32).min(u64::from(u32::MAX)))
        .map_err(|_| "Drawing buffer range overflow")?;
    if size == 0 {
        return Err("Device cannot hold one drawing segment");
    }
    let mut out = Vec::new();
    let mut from = 0;
    while from < count {
        let to = from + size.min(count - from);
        out.push(from..to);
        from = to;
    }
    Ok(out)
}
fn settings(scene: &Scene, row: &crate::sheet_source::Row, base: u32, selected: bool) -> Vec<u8> {
    let mut bytes = row
        .model
        .m
        .iter()
        .flat_map(|v| (*v as f32).to_le_bytes())
        .collect::<Vec<_>>();
    bytes.extend(
        scene
            .face_rgba(row.id)
            .unwrap_or([0.; 4])
            .into_iter()
            .flat_map(f32::to_le_bytes),
    );
    bytes.extend(
        [
            scene.pick_id(row.id.value()).map_or(0, ObjectId::value),
            base,
            u32::from(selected),
            0,
        ]
        .into_iter()
        .flat_map(u32::to_le_bytes),
    );
    bytes
}
