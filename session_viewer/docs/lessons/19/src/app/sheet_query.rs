// --8<-- [start:step-9a]
//! Answers what a click hit on a sheet, the flat drawing page laid over the model.
use serde::Deserialize;
use std::cell::Cell;
use std::rc::Rc;

/// Bytes of the table head: `SHM1` then a u32 record count.
pub const HEAD_BYTES: u64 = 8;

/// Bytes of one record: u64 offset, u64 length.
pub const RECORD_BYTES: u64 = 16;

/// Largest entity record accepted.
pub const MAX_BLOB: u64 = 64 * 1024;

/// One sheet entity's identity, from its JSON record.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EntityMeta {
    #[serde(default)]
    pub guid: String, // source object id
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub kind: String, // wall, door, ...
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub color: Vec<f32>,
}

/// The table head, cached per sheet.
#[derive(Clone, Debug)]
pub struct SheetTable {
    pub count: u32, // records in the table
    pub revision: Option<String>, // ETag every read must match
}

/// The record count from a table head.
pub fn table_count(raw: &[u8]) -> Result<u32, String> {
    if raw.len() != HEAD_BYTES as usize || &raw[..4] != b"SHM1" {
        return Err("Side table head is not SHM1".to_string());
    }

    Ok(u32::from_le_bytes(
        raw[4..8].try_into().expect("exact head checked above"),
    ))
}

/// One record as (offset, length).
pub fn record(raw: &[u8]) -> Result<(u64, u64), String> {
    if raw.len() != RECORD_BYTES as usize {
        return Err("Side table record must contain exactly 16 bytes".to_string());
    }

    Ok((
        u64::from_le_bytes(raw[..8].try_into().expect("exact record checked above")),
        u64::from_le_bytes(raw[8..].try_into().expect("exact record checked above")),
    ))
}

// --8<-- [end:step-9a]
// --8<-- [start:step-9b]
/// Byte position of record `id`; `record_at(count)` starts the blobs.
pub fn record_at(id: u32) -> u64 {
    HEAD_BYTES + RECORD_BYTES * u64::from(id)
}

/// An entity from its JSON; missing keys default.
pub fn entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw).map_err(|error| format!("Entity record is not valid JSON: {error}"))
}

/// One entity lookup in flight.
pub struct Query {
    pub id: u64, // lookup number
    pub row: u32, // the sheet's object row
    pub entity: u32, // entity index in the sheet
    pub cancelled: Rc<Cell<bool>>, // set when a newer lookup replaces this
}

impl Query {
    /// A new lookup.
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
    /// Cancel the lookup.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// The answer to one lookup.
pub struct Resolved {
    pub query: u64, // which lookup
    pub result: Result<(EntityMeta, SheetTable), String>,
}

// --8<-- [end:step-9b]
// --8<-- [start:step-9c]
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::loader::post;

    use crate::app::fetch::fetch_range as range;

    /// Start reading one entity; the answer arrives as a message.
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

    /// Read the entity and post the answer unless cancelled.
    async fn post_entity(
        query: u64,
        entity: u32,
        cancelled: Rc<Cell<bool>>,
        url: String,
        table: Option<SheetTable>,
        entities: u32,
        // --8<-- [end:step-9c]
    // --8<-- [start:step-9d]
    ) {
        let result = read_entity(&url, entity, table, entities, &cancelled).await;

        if !cancelled.get() {
            post(crate::Msg::SheetEntity(Resolved { query, result }));
        }
    }

    /// Read the head if unknown, then the record, then the blob.
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

    /// Build a side table from JSON blobs.
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

    /// Head, records and blobs parse; junk is refused.
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
// --8<-- [end:step-9d]
