Full minitest rules and conventions for this project.

## Test file imports

### Python (top of file)
```python
from .mini_test import MINI_TEST
from .mini_test import MINI_CHECK
from .mini_test import run_all
from .tolerance import TOLERANCE
from .tolerance import PI
import math
```
Geometry imports (Point, NurbsCurve, etc.) go INSIDE each test function.

### C++ (top of file)
```cpp
#include "tolerance.h"
```

### Rust (top of file)
```rust
use crate::tolerance::TOLERANCE;
use crate::tolerance::PI;
```
Per-function: `use crate::point::Point;` etc. inside MINI_TEST blocks.

## Test requirements
- One test per API method, identical across all 3 languages
- Constructor test groups: default ctor, overloads, [], ==, !=, str(), repr()
- API method order: constructors, accessors, mutators (*_self), copy operators (+,-,*,/), utilities, serialization, str/repr
- JSON fields alphabetically ordered (C++: ordered_json, Python: dict keys sorted, Rust: serde_json default)
- Protobuf: to_proto/from_proto required on all geometry classes
- Test output goes to `session_tests/session_{lang}/` as JSON for Vue viewer
- Check JSON contents for failures — the viewer shows serialized output

## Code style
- Python: one import per line, never `from session_py import Mesh, Point`
- C++: never `#include "tolerance.h"` in production code (minitest only)
- C++: use `std::cout << point` not manual coordinate printing

## Verify
Run `./bash/minitest.sh` — all tests must pass. Check JSON output.
