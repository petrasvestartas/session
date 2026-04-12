Full minitest rules, conventions, and language-specific templates for this project.

## Core Rules

1. Same test names across all 3 languages (C++, Python, Rust)
2. Same variable names, assertion order, similar line count
3. One test per API method; test names start with capital letter
4. Method order: constructors -> accessors -> mutators (*_self) -> operators -> utilities -> serialization -> str/repr
5. JSON fields alphabetically ordered across all languages
6. Every class needs: json_dump/json_load + to_proto/from_proto tests
7. Operators go inside constructor test, not separate tests
8. Collections: each object on separate line
9. Test output goes to `session_tests/session_{lang}/` as JSON for Vue viewer

## Constructor Test Must Include

- Default constructor + parameterized constructor
- Index operator []
- String representation (str, repr)
- Equality operators (==, !=)
- In-place operators (+=, -=, *=, /=)
- Copy operators (+, -, *, /)
- duplicate() with new GUID check

## Test File Imports

### Python (top of file)
```python
from session_py.mini_test import MINI_TEST
from session_py.mini_test import MINI_CHECK
from session_py.mini_test import run_all
from session_py.tolerance import TOLERANCE
from session_py.tolerance import PI
import math
```
Geometry imports (Point, NurbsCurve, etc.) go INSIDE each test function using flat imports:
`from session_py import ClassName` (not `from session_py.classname import ClassName`)

### C++ (top of file)
```cpp
#include "mini_test.h"
#include "classname.h"
#include "tolerance.h"
using namespace session_cpp::mini_test;
```
Never `#include "tolerance.h"` in production code (minitest only).
Use `std::cout << point` not manual coordinate printing.

### Rust (top of file)
```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;
use crate::tolerance::{TOLERANCE, PI};
```
Geometry imports go inside MINI_TEST blocks: `use crate::ClassName;`

## C++ Template

```cpp
#include "mini_test.h"
#include "classname.h"
#include "tolerance.h"

using namespace session_cpp::mini_test;

namespace session_cpp {

MINI_TEST("ClassName", "Constructor") {
    ClassName obj(1.0, 2.0, 3.0);

    std::string cstr = obj.str();
    std::string crepr = obj.repr();

    ClassName copy = obj;

    MINI_CHECK(obj.is_valid() == true);
    MINI_CHECK(obj[0] == 1.0);
    MINI_CHECK(obj.name == "my_classname");
    MINI_CHECK(!obj.guid.empty());
    MINI_CHECK(cstr == "ClassName(1, 2, 3)");
    MINI_CHECK(crepr.find("name=my_classname") != std::string::npos);
    MINI_CHECK(copy.guid != obj.guid);
    MINI_CHECK(copy == obj);
}

MINI_TEST("ClassName", "Json_roundtrip") {
    ClassName obj(1.0, 2.0, 3.0);
    obj.name = "test_json";

    std::string path = "test_classname.json";
    obj.json_dump(path);
    ClassName loaded = ClassName::json_load(path);

    MINI_CHECK(loaded.name == obj.name);
    MINI_CHECK(loaded[0] == obj[0]);
    MINI_CHECK(loaded == obj);
}

MINI_TEST("ClassName", "Protobuf_roundtrip") {
    ClassName obj(1.0, 2.0, 3.0);
    obj.name = "test_proto";

    std::string path = "test_classname.bin";
    obj.protobuf_dump(path);
    ClassName loaded = ClassName::protobuf_load(path);

    MINI_CHECK(loaded.name == obj.name);
    MINI_CHECK(loaded[0] == obj[0]);
    MINI_CHECK(loaded == obj);
}

REGISTER_MINI_TEST("ClassName", run_classname_tests);

} // namespace session_cpp
```

CMakeLists.txt: add `src/classname_test.cpp` to `MINITEST_SOURCES`.

## Python Template

```python
from session_py.mini_test import MINI_TEST
from session_py.mini_test import MINI_CHECK


@MINI_TEST("ClassName", "Constructor")
def test_classname_constructor():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)

    cstr = str(obj)
    crepr = repr(obj)

    copy = obj.duplicate()

    MINI_CHECK(obj.is_valid() == True)
    MINI_CHECK(obj[0] == 1.0)
    MINI_CHECK(obj.name == "my_classname")
    MINI_CHECK(obj.guid != "")
    MINI_CHECK(cstr == "ClassName(1, 2, 3)")
    MINI_CHECK("name=my_classname" in crepr)
    MINI_CHECK(copy.guid != obj.guid)
    MINI_CHECK(copy == obj)


@MINI_TEST("ClassName", "Json_roundtrip")
def test_classname_json_roundtrip():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)
    obj.name = "test_json"

    path = "test_classname.json"
    obj.json_dump(path)
    loaded = ClassName.json_load(path)

    MINI_CHECK(loaded.name == obj.name)
    MINI_CHECK(loaded[0] == obj[0])
    MINI_CHECK(loaded == obj)


@MINI_TEST("ClassName", "Protobuf_roundtrip")
def test_classname_protobuf_roundtrip():
    from session_py import ClassName

    obj = ClassName(1.0, 2.0, 3.0)
    obj.name = "test_proto"

    path = "test_classname.bin"
    obj.protobuf_dump(path)
    loaded = ClassName.protobuf_load(path)

    MINI_CHECK(loaded.name == obj.name)
    MINI_CHECK(loaded[0] == obj[0])
    MINI_CHECK(loaded == obj)


if __name__ == "__main__":
    test_classname_constructor()
    test_classname_json_roundtrip()
    test_classname_protobuf_roundtrip()
```

Add to `session_py/src/session_py/__init__.py`: `from session_py.classname import ClassName`
Add `"classname"` to `CLASS_NAMES` array in `bash/minitest.sh`.

## Rust Template

```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;

pub fn run_classname_constructor() -> TestResult {
    MINI_TEST!("Constructor", {
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
    MINI_TEST!("Json_roundtrip", {
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
    MINI_TEST!("Protobuf_roundtrip", {
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

REGISTER_MINI_TEST!("ClassName", "Constructor", crate::classname_test::run_classname_constructor);
REGISTER_MINI_TEST!("ClassName", "Json_roundtrip", crate::classname_test::run_classname_json_roundtrip);
REGISTER_MINI_TEST!("ClassName", "Protobuf_roundtrip", crate::classname_test::run_classname_protobuf_roundtrip);
```

Add to `lib.rs`: `pub mod classname_test;`
Add `"classname"` to `CLASS_NAMES` array in `bash/minitest.sh`.

## Verify
Run `./bash/minitest.sh` -- all tests must pass. Check JSON output in `session_tests/`.
