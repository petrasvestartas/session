        if let Some(arena)=&self.arena {arena.borrow_mut().prune();}
        if self.entries.is_empty(){self.arena=None;}