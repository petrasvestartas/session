use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct Fault(Arc<Mutex<Option<String>>>);

impl Fault {
    pub fn remember(&self, message: String) -> bool {
        let mut first = self.0.lock().unwrap();
        if first.is_some() { return false; }
        *first = Some(message);
        true
    }

    pub fn message(&self) -> Option<String> { self.0.lock().unwrap().clone() }

    pub fn same_device(&self, other: &Self) -> bool { Arc::ptr_eq(&self.0, &other.0) }
}
