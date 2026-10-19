];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
