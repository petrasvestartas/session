    failure: Option<Event>,
    // An optional field lets old reports decode unchanged; keeping identity outside the event queue preserves it when later events rotate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    adapter: Option<crate::adapter_info::AdapterInfo>,