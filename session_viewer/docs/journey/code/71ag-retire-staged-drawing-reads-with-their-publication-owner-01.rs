struct State {
    active:bool,inflight:bool,again:bool,
    pending_sheets:Vec<crate::sheet_stream::Cursor>,sheets:Vec<Weak<crate::sheet_source::Prepared>>,
    pending_streams:Vec<crate::stream_load::Cursor>,streams:Vec<std::rc::Weak<crate::range_cloud::Prepared>>,cancelled:Rc<Cell<bool>>,
    gate:crate::read_gate::ReadGate,
