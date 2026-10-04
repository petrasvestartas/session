    adapter: Option<crate::adapter_info::AdapterInfo>,
    #[serde(default, skip_serializing_if = "std::collections::VecDeque::is_empty")]
    phases: std::collections::VecDeque<crate::load_phase::Phase>,
    #[serde(default, skip_serializing_if = "crate::load_phase::zero")]
    phases_dropped: u64,