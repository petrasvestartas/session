# Skill: New Datastructure in Rust

## Files to Create

1. `session_rust/src/name.rs` - Implementation
2. `session_rust/src/name_test.rs` - Minitest file

## Minimal Implementation (name.rs)

```rust
use uuid::Uuid;

#[derive(Clone)]
pub struct Name {
    pub guid: String,
    pub name: String,
}

impl Name {
    pub fn new() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_name".to_string(),
        }
    }

    pub fn str(&self) -> String {
        "Name()".to_string()
    }

    pub fn repr(&self) -> String {
        format!("Name(name={})", self.name)
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn duplicate(&self) -> Self {
        let mut copy = self.clone();
        copy.guid = Uuid::new_v4().to_string();
        copy
    }
}

impl Default for Name {
    fn default() -> Self {
        Self::new()
    }
}
```

## Minimal Test (name_test.rs)

```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;

pub fn run_name_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::Name;

        let obj = Name::new();

        let cstr = obj.str();
        let crepr = obj.repr();

        MINI_CHECK!(obj.is_valid() == true);
        MINI_CHECK!(obj.name == "my_name");
        MINI_CHECK!(!obj.guid.is_empty());
        MINI_CHECK!(cstr == "Name()");
        MINI_CHECK!(crepr.contains("name=my_name"));
    })
}

REGISTER_MINI_TEST!("Name", run_name_constructor);
```

## Register in lib.rs

Add to `session_rust/src/lib.rs`:

```rust
pub mod name;
pub mod name_test;
pub use name::Name;
```

## Register in minitest.sh

Add `"name"` to `CLASS_NAMES` array in `bash/minitest.sh`

## Build & Test

```bash
cd session_rust
cargo run --release --bin minitest
```
