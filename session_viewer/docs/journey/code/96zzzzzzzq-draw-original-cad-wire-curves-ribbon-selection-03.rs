        let mut flags = selected.to_vec();
        if let Some(owner) = self.highlighted { if !flags.contains(&owner) { flags.push(owner); } }
        let rows = ribbon::rows(scene, anchor, &flags, limit)?;