    pub context: Context,
    events: std::collections::VecDeque<Event>,
    failure: Option<Event>,
}