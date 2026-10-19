
#[derive(Clone, Copy)]
pub struct Request { pub window: Window, pub transform: [f32; 16], pub density: f32 }

pub(crate) struct Queued { request: Request, ticket: crate::pick_gate::Ticket }
struct Flight { ready: Arc<AtomicU8>, ticket: crate::pick_gate::Ticket }


pub struct Picker {
