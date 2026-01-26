# Mini Test Framework

Cross-language test comparison for Python, C++, and Rust implementations.

## Adding a New Test Suite (e.g., `bbox`)

### Step 1: minitest.sh
```bash
# Edit line 10
TEST_SUITES=(point color vector bbox)
```

### Step 2: Python
Create `session_py/src/session_py/bbox_test.py`:
```python
from .mini_test import MINI_TEST, MINI_CHECK, run_all

@MINI_TEST("BBox", "constructor")
def test_bbox_constructor():
    from session_py import BBox
    b = BBox(0, 0, 0, 1, 1, 1)
    MINI_CHECK(b.min[0] == 0 and b.max[0] == 1)

if __name__ == "__main__":
    run_all("python")
```

### Step 3: C++
Create `session_cpp/src/bbox_test.cpp`:
```cpp
#include "mini_test.h"
#include "bbox.h"
using namespace session_cpp::mini_test;

namespace session_cpp {
    MINI_TEST("BBox", "constructor") {
        BBox b(0, 0, 0, 1, 1, 1);
        MINI_CHECK(b.min()[0] == 0 && b.max()[0] == 1);
    }
}
```

Add to `session_cpp/CMakeLists.txt` MINITEST_SOURCES:
```cmake
set(MINITEST_SOURCES
    src/point_test.cpp
    src/color_test.cpp
    src/vector_test.cpp
    src/bbox_test.cpp      # ADD THIS
)
```

### Step 4: Rust
Create `session_rust/src/bbox_test.rs`:
```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;

pub fn run_bbox_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::BBox;
        let b = BBox::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        MINI_CHECK!(b.min()[0] == 0.0 && b.max()[0] == 1.0);
    })
}

REGISTER_MINI_TEST!("BBox", "constructor", crate::bbox_test::run_bbox_constructor);
```

Add to `session_rust/src/lib.rs`:
```rust
pub mod bbox_test;
```

### Step 5: Run
```bash
./bash/minitest.sh
```

## Local Development
```bash
./bash/minitest.sh              # All tests + Vue viewer
./bash/minitest.sh --py         # Python only
./bash/minitest.sh --fast       # Skip dependency installs
./bash/minitest.sh --no-web     # Skip Vue server
```
