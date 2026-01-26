# Testing - Rust

## Test File Structure

```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;

pub fn run_classname_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::ClassName;

        let obj = ClassName::new(1.0, 2.0, 3.0);

        let cstr = obj.str();
        let crepr = obj.repr();

        let copy = obj.duplicate();

        MINI_CHECK!(obj.is_valid() == true);
        MINI_CHECK!(obj[0] == 1.0);
        MINI_CHECK!(obj.name == "my_classname");
        MINI_CHECK!(!obj.guid.is_empty());
        MINI_CHECK!(cstr == "ClassName(1, 2, 3)");
        MINI_CHECK!(crepr.contains("name=my_classname"));
        MINI_CHECK!(copy.guid != obj.guid);
        MINI_CHECK!(copy == obj);
    })
}

pub fn run_classname_json_roundtrip() -> TestResult {
    MINI_TEST!("json_roundtrip", {
        use crate::ClassName;

        let mut obj = ClassName::new(1.0, 2.0, 3.0);
        obj.name = "test_json".to_string();

        let path = "test_classname.json";
        obj.json_dump(path);
        let loaded = ClassName::json_load(path);

        MINI_CHECK!(loaded.name == obj.name);
        MINI_CHECK!(loaded[0] == obj[0]);
        MINI_CHECK!(loaded == obj);
    })
}

pub fn run_classname_protobuf_roundtrip() -> TestResult {
    MINI_TEST!("protobuf_roundtrip", {
        use crate::ClassName;

        let mut obj = ClassName::new(1.0, 2.0, 3.0);
        obj.name = "test_proto".to_string();

        let path = "test_classname.bin";
        obj.protobuf_dump(path);
        let loaded = ClassName::protobuf_load(path);

        MINI_CHECK!(loaded.name == obj.name);
        MINI_CHECK!(loaded[0] == obj[0]);
        MINI_CHECK!(loaded == obj);
    })
}

REGISTER_MINI_TEST!("ClassName", "constructor", crate::classname_test::run_classname_constructor);
REGISTER_MINI_TEST!("ClassName", "json_roundtrip", crate::classname_test::run_classname_json_roundtrip);
REGISTER_MINI_TEST!("ClassName", "protobuf_roundtrip", crate::classname_test::run_classname_protobuf_roundtrip);
```

## lib.rs Registration

```rust
pub mod classname;
pub mod classname_test;
pub use classname::ClassName;
```

## minitest.sh Entry

Add `"classname"` to `CLASS_NAMES` array in `bash/minitest.sh`
