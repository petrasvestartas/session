    Some((start, bytes))
}
#[cfg(target_arch="wasm32")]
impl Source {
    pub async fn from_probe(
        url: String,
        probe: &crate::range_fetch::Reply,
    ) -> Result<Self, String> {
        if probe.status != 206 || probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err(
                "Ranged drawings require byte ranges and a revision or immutable URL".into(),
            );
        }
        let (guid, name) =
            identity(&probe.bytes).ok_or("Ranged drawing must contain one original sheet")?;
        let (coords_at, coords_len, end) =
            crate::stream::sheet_layout(&probe.bytes).ok_or("Missing sheet coordinates")?;
        if coords_len == 0
            || coords_len % 48 != 0
            || end > probe.total.ok_or("Missing original drawing length")?
        {
            return Err("Invalid original sheet coordinate interval".into());
        }
        let mut fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: u32::try_from(coords_len / 48).map_err(|_| "Too many drawing segments")?,
            revision: probe.etag.clone(),
            ..Default::default()
        };
        let mut at = coords_at + coords_len;
        let mut window = crate::stream::MetadataWindow::default();
        let mut seen = std::collections::HashSet::new();
        while at < end {
            let header = window
                .read(&url, at, 64.min(end - at), end, &fields.revision)
                .await
                .ok_or("Drawing metadata range failed")?;
            let field = field_at(header, at, end).ok_or("Invalid drawing field")?;
            if !seen.insert(field.field) {
                return Err("Repeated original drawing field".into());
            }
            let body = if (field.field, field.wire) == (8, 2) {
                if field.value > 256 {
                    return Err("Original metadata URL is too long".into());
                }
                window
                    .read(&url, field.body, field.value, end, &fields.revision)
                    .await
                    .ok_or("Drawing metadata URL range failed")?
            } else {
                &[]
            };
            if !fields.set(&field, body) {
                return Err("Invalid original drawing array or count".into());
            }
            at = field.next;
        }
        if !seen.contains(&6) {
            return Err("Original drawing has no declared segment count".into());
        }
        envelope(&url, end, probe.total.unwrap(), &fields.revision, &guid).await?;
        Ok(Self {
            guid,
            name,
            url,
            fields,
        })
    }
}
#[cfg(test)]
mod tests {
