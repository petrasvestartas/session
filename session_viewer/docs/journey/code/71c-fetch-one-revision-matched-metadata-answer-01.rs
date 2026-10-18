};
use std::cell::Cell;
pub async fn read(
    url: &str,
    id: u32,
    entities: u32,
    cached: Option<SheetTable>,
    cancelled: &Cell<bool>,
    signal: &web_sys::AbortSignal,
) -> Result<(EntityMeta, SheetTable), String> {
    if cancelled.get() {
        return Err("Entity lookup cancelled".into());
    }
    let table = if let Some(table) = cached {
        table
    } else {
        let reply = crate::range_fetch::get_with_signal(
            url,
            &crate::range_fetch::GetOpts {
                range: Some((0, HEAD_BYTES)),
                revalidate: true,
            },
            Some(signal),
        )
        .await?;
        if reply.status != 206 {
            return Err("Entity metadata requires byte ranges".into());
        }
        let count = table_count(&reply.bytes)?;
        if count != entities {
            return Err("Side table record count differs from the original sheet".into());
        }
        if reply.etag.is_none() && !crate::manifest::immutable_key(url) {
            return Err("Metadata reads require a revision or immutable URL".into());
        }
        SheetTable {
            count,
            revision: reply.etag,
            total: reply.total.ok_or("Side table has no checked byte length")?,
        }
    };
    if table.revision.is_none() && !crate::manifest::immutable_key(url) {
        return Err("Metadata reads require a revision or immutable URL".into());
    }
    if table.count != entities || id >= table.count || cancelled.get() {
        return Err("Entity lookup is stale or outside the original table".into());
    }
    let record = reply(url, record_at(id), RECORD_BYTES, &table, cancelled, signal).await?;
    let total = record
        .total
        .ok_or("Side table has no checked byte length")?;
    let (at, length) = blob_range(table.count, id, &record.bytes, total)?;
    let blob = reply(url, at, length, &table, cancelled, signal).await?;
    if blob.total != Some(total) || total != table.total {
        return Err("Original side table length changed".into());
    }
    Ok((entity_from(&blob.bytes)?, table))
}
async fn reply(
    url: &str,
    at: u64,
    length: u64,
    table: &SheetTable,
    cancelled: &Cell<bool>,
    signal: &web_sys::AbortSignal,
) -> Result<crate::range_fetch::Reply, String> {
    if cancelled.get() {
        return Err("Entity lookup cancelled".into());
    }
    let reply = crate::range_fetch::get_with_signal(
        url,
        &crate::range_fetch::GetOpts {
            range: Some((at, length)),
            revalidate: false,
        },
        Some(signal),
    )
    .await?;
    if cancelled.get() || reply.status != 206 || reply.etag != table.revision {
        return Err("Original entity lookup was cancelled or its revision changed".into());
    }
    Ok(reply)
}
