    pub(super) cloud_query: Option<crate::app::cloud_query::Query>, // a point-cloud pick in flight; register:cloud_query
    #[cfg(target_arch = "wasm32")] // register:cloud_query
    pub(super) query_generation: u64, // counts cloud queries, old answers dropped; register:cloud_query
