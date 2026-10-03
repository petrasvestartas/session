use crate::rehydrate::ReloadKey;

pub struct Request {
    pub ticket: u64,
    pub urls: Vec<String>,
}

struct Pending {
    ticket: u64,
    keys: Vec<ReloadKey>,
}

#[derive(Default)]
pub struct ReloadJob {
    issued: u64,
    pending: Option<Pending>,
}

impl ReloadJob {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.cancel();
        if keys.is_empty() { return Err("No released sources to reload"); }
        let mut origins = std::collections::HashSet::new();
        let mut urls = Vec::new();
        for key in &keys {
            if !origins.insert(key.origin.id) { return Err("Duplicate reload origin"); }
            let location = key.origin.location.as_ref().ok_or("Source has no reload location")?;
            urls.push(location.value().to_owned());
        }
        let ticket = self.issued.checked_add(1).ok_or("Reload tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(Pending { ticket, keys });
        Ok(Request { ticket, urls })
    }

    pub fn pending(&self) -> Option<u64> { self.pending.as_ref().map(|pending| pending.ticket) }

    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| pending.keys)
    }

    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}
