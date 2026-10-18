}

#[cfg(target_arch = "wasm32")]
async fn envelope(
    url: &str,
    end: u64,
    total: u64,
    revision: &Option<String>,
    guid: &str,
) -> Result<(), String> {
    let length = total
        .checked_sub(end)
        .ok_or("Drawing exceeds its original file")?;
    if length > 1024 * 1024 {
        return Err("Original drawing envelope metadata exceeds its range limit".into());
    }
    if length > 0 {
        let tail = crate::stream::source_range(url, end, length, revision)
            .await
            .ok_or("Drawing envelope read failed")?;
        crate::range_shape::metadata(&tail, guid)?;
    }
    Ok(())
}
