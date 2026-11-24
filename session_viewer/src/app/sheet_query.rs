pub use viewer_core::sheet_query::*;

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::fetch_range as range;
    use crate::app::loader::post;
    use std::cell::Cell;
    use std::rc::Rc;

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
