//! Sheet entity lookup by bounded range reads of the side table: the 8-byte head once per
//! sheet, then one 16-byte record and one JSON blob per pick. The segment itself was picked
//! on the GPU; only its entity's identity comes off the wire.

use serde::Deserialize;
use std::cell::Cell;
use std::rc::Rc;

/// The side table's head: magic `SHM1`, u32 LE record count.
pub const HEAD_BYTES: u64 = 8;
/// One record: u64 LE offset, u64 LE length, relative to the byte after the table.
pub const RECORD_BYTES: u64 = 16;
/// A blob longer than this is not one entity's record.
pub const MAX_BLOB: u64 = 64 * 1024;

/// One entity's authored identity: the side table's JSON blob.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EntityMeta {
    #[serde(default)]
    pub guid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub color: Vec<f32>,
}

/// The validated head, cached per sheet with the revision every later read must match.
#[derive(Clone, Debug)]
pub struct SheetTable {
    pub count: u32,
    pub revision: Option<String>,
}

/// The record count behind a valid magic.
pub fn table_count(raw: &[u8]) -> Result<u32, String> {
    if raw.len() != HEAD_BYTES as usize || &raw[..4] != b"SHM1" {
        return Err("Side table head is not SHM1".to_string());
    }
    Ok(u32::from_le_bytes(
        raw[4..8].try_into().expect("exact head checked above"),
    ))
}

/// One record's (offset, length).
pub fn record(raw: &[u8]) -> Result<(u64, u64), String> {
    if raw.len() != RECORD_BYTES as usize {
        return Err("Side table record must contain exactly 16 bytes".to_string());
    }
    Ok((
        u64::from_le_bytes(raw[..8].try_into().expect("exact record checked above")),
        u64::from_le_bytes(raw[8..].try_into().expect("exact record checked above")),
    ))
}

/// Where record `id` sits; `record_at(count)` is where the blobs start.
pub fn record_at(id: u32) -> u64 {
    HEAD_BYTES + RECORD_BYTES * u64::from(id)
}

/// The JSON blob as an entity; unknown keys are ignored, missing ones default.
pub fn entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw).map_err(|error| format!("Entity record is not valid JSON: {error}"))
}

/// One entity lookup in flight. A superseding selection cancels the token even while HTTP awaits.
pub struct Query {
    pub id: u64,
    pub row: u32,
    pub entity: u32,
    pub cancelled: Rc<Cell<bool>>,
}

impl Query {
    /// A fresh token for entity `entity` of the sheet on `row`.
    pub fn new(id: u64, row: u32, entity: u32) -> Self {
        Self {
            id,
            row,
            entity,
            cancelled: Rc::new(Cell::new(false)),
        }
    }
}

impl Drop for Query {
    /// Retire the callback when the query completes, fails, or is replaced by new input.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// The answer for one query generation: the entity and the table head to cache.
pub struct Resolved {
    pub query: u64,
    pub result: Result<(EntityMeta, SheetTable), String>,
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::{
        fetch::{GetOpts, get},
        loader::post,
    };

    /// Validate the exact ranged body and any exposed revision; never consume a whole-file 200.
    async fn range(
        url: &str,
        at: u64,
        len: u64,
        revision: &Option<String>,
    ) -> Result<(Vec<u8>, Option<String>), String> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((at, len)),
                revalidate: true,
                ..Default::default()
            },
        )
        .await?;
        if reply.status != 206 || reply.bytes.len() as u64 != len {
            return Err(format!(
                "Side table range failed (HTTP {}, {} of {len} bytes)",
                reply.status,
                reply.bytes.len()
            ));
        }
        if revision.is_some() && revision != &reply.etag {
            return Err("Side table changed during selection; reload the sheet".to_string());
        }
        Ok((reply.bytes, reply.etag))
    }

    /// Schedule one entity read; the named task posts its completion to the event loop.
    pub fn fetch_entity(query: &Query, url: String, table: Option<SheetTable>, entities: u32) {
        wasm_bindgen_futures::spawn_local(post_entity(
            query.id,
            query.entity,
            query.cancelled.clone(),
            url,
            table,
            entities,
        ));
    }

    /// Complete or fail exactly one lookup, unless a newer selection retired its generation.
    async fn post_entity(
        query: u64,
        entity: u32,
        cancelled: Rc<Cell<bool>>,
        url: String,
        table: Option<SheetTable>,
        entities: u32,
    ) {
        let result = read_entity(&url, entity, table, entities, &cancelled).await;
        if !cancelled.get() {
            post(crate::Msg::SheetEntity(Resolved { query, result }));
        }
    }

    /// The head (once per sheet, checked against the sheet's `entity_count`), the record,
    /// then the blob, all under the side table's own revision.
    pub async fn read_entity(
        url: &str,
        id: u32,
        table: Option<SheetTable>,
        entities: u32,
        cancelled: &Cell<bool>,
    ) -> Result<(EntityMeta, SheetTable), String> {
        let table = match table {
            Some(table) => table,
            None => {
                let (raw, revision) = range(url, 0, HEAD_BYTES, &None).await?;
                let count = table_count(&raw)?;
                if count != entities {
                    return Err(format!(
                        "Side table holds {count} records; the sheet says {entities}"
                    ));
                }
                SheetTable { count, revision }
            }
        };
        if cancelled.get() {
            return Err("Entity lookup cancelled".to_string());
        }
        if id >= table.count {
            return Err(format!(
                "Entity {id} is outside the {}-record side table",
                table.count
            ));
        }
        let (raw, _) = range(url, record_at(id), RECORD_BYTES, &table.revision).await?;
        if cancelled.get() {
            return Err("Entity lookup cancelled".to_string());
        }
        let (offset, length) = record(&raw)?;
        if length > MAX_BLOB {
            return Err(format!(
                "Entity {id} record is {length} bytes, over the {MAX_BLOB} limit"
            ));
        }
        let at = record_at(table.count)
            .checked_add(offset)
            .ok_or("Entity record offset overflows")?;
        let (blob, _) = range(url, at, length, &table.revision).await?;
        Ok((entity_from(&blob)?, table))
    }
}
#[cfg(target_arch = "wasm32")]
pub use web::fetch_entity;

#[cfg(test)]
mod tests {
    use super::*;

    /// A side table with two blobs: head, records, then the blobs the records address.
    fn table(blobs: &[&str]) -> Vec<u8> {
        let mut out = b"SHM1".to_vec();
        out.extend((blobs.len() as u32).to_le_bytes());
        let mut offset = 0u64;
        for blob in blobs {
            out.extend(offset.to_le_bytes());
            out.extend((blob.len() as u64).to_le_bytes());
            offset += blob.len() as u64;
        }
        for blob in blobs {
            out.extend(blob.as_bytes());
        }
        out
    }

    /// The head, the records and the blobs parse from a hand-built buffer; junk is refused.
    #[test]
    fn side_table_head_records_and_blobs_parse_from_a_hand_built_buffer() {
        let raw = table(&[
            r#"{"guid":"g0","name":"Wall A","kind":"wall","width":0.35,"color":[0,0,0,255]}"#,
            r#"{"name":"Door","kind":"door","extra":true}"#,
        ]);
        let count = table_count(&raw[..HEAD_BYTES as usize]).unwrap();
        assert_eq!(count, 2);
        let blobs = record_at(count) as usize;
        let at = record_at(1) as usize;
        let (offset, length) = record(&raw[at..at + RECORD_BYTES as usize]).unwrap();
        let door = entity_from(&raw[blobs + offset as usize..][..length as usize]).unwrap();
        assert_eq!(door.name, "Door");
        assert_eq!(door.kind, "door");
        assert_eq!(door.guid, "");
        let at = record_at(0) as usize;
        let (offset, length) = record(&raw[at..at + RECORD_BYTES as usize]).unwrap();
        let wall = entity_from(&raw[blobs + offset as usize..][..length as usize]).unwrap();
        assert_eq!(wall.guid, "g0");
        assert_eq!(wall.width, 0.35);
        assert_eq!(wall.color, [0.0, 0.0, 0.0, 255.0]);
        assert!(table_count(b"SHM2\0\0\0\0").is_err());
        assert!(table_count(&raw[..7]).is_err());
        assert!(record(&raw[..15]).is_err());
        assert!(entity_from(b"{").is_err());
    }

    /// Dropping a query retires its token, so a late answer is ignored.
    #[test]
    fn dropping_a_query_cancels_its_token() {
        let query = Query::new(3, 1, 9);
        let token = query.cancelled.clone();
        drop(query);
        assert!(token.get());
    }
}
