            && self.adapter.as_ref().is_none_or(|info| info.valid())
            && self.phases.len() <= 256 && self.phases.iter().all(|phase| phase.valid())
            && self.fits()