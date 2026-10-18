entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw).map_err(|error| format!("Entity record is not valid JSON: {error}"))
}

/// One entity lookup in flight.
pub struct Query {
    pub id: u64,                   // lookup number
    pub row: u32,                  // the sheet's object row
    pub entity: u32,               // entity index in the sheet
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
    pub query: u64,                                       // which lookup
    pub result: Result<(EntityMeta, SheetTable), String>, // the entity and the table head
}

