# Session Multi-Language Project

Simple 3D Point and Color library with **JSON compatibility** between Rust, Python, and C++.

## Quick Examples

### Python
```python
from session_py.point import Point
point = Point(10.0, 20.0, 30.0)
point.to_json("point.json")
```

### Rust  
```rust
use session_rust::Point;
let point = Point::new(10.0, 20.0, 30.0);
point.to_json("point.json").unwrap();
```

### C++
```cpp
#include "point.h"
using namespace session_cpp;
Point point(10.0, 20.0, 30.0);
point.to_json("point.json");
```

## Project Structure

```
session/
├── session_py/     # Python implementation
├── session_rust/   # Rust implementation  
├── session_cpp/    # C++ implementation
└── docs/           # Documentation
```

## Documentation

View the complete documentation:

```bash
pip install mkdocs mkdocs-material mkdocstrings[python]
mkdocs serve
```

Then visit http://localhost:8000

## Key Features

- ✅ **Cross-Language** - Same data format across all three languages
- ✅ **JSON Compatible** - Save/load data between languages
- ✅ **Simple API** - Easy to use Point and Color classes
- ✅ **Well Documented** - Comprehensive documentation with examples

## Building

### Python
```bash
cd session_py
python main.py
```

### Rust
```bash
cd session_rust  
cargo run
```

### C++
```bash
cd session_cpp
mkdir build && cd build
cmake ..
make
./MyProject
```

All three will create compatible JSON files that can be used by any implementation!
