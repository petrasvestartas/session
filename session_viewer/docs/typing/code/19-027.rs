    pub(super) sheet_query: Option<crate::app::sheet_query::Query>, // a sheet pick in flight; register:sheets
    pub(super) sheet_generation: u64, // counts sheet queries, old answers dropped; register:sheets
