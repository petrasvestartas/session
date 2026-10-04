            && self.adapter.as_ref().is_none_or(|info| info.valid())
            && self.events.len() <= 24 && self.events.iter().all(event)