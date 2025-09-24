# Session C++ API Documentation

Clean, comprehensive API documentation for the C++ implementation of the Session library.

## Quick Start

Generate the documentation:

```bash
doxygen
```

Then open `docs/html/index.html` in your browser to view the complete API reference.

## What's Documented

### Current API Classes
- **session_cpp::Point** - 3D geometric points with visual properties and JSON serialization
- **session_cpp::Color** - RGBA colors with utility methods and JSON serialization

*Note: The documentation is configured to only include Point and Color classes. As you add more classes, update the `INPUT` line in `Doxyfile` to include additional header files.*

### Features
- **Cross-language compatibility** - Full JSON serialization for interop with Python and Rust
- **Modern C++** - Clean, well-documented API with contemporary practices
- **Complete examples** - Usage patterns and code samples

## Documentation Features

The generated HTML documentation provides:

- **Main Page** - Project overview with quick start examples
- **Class Reference** - Detailed API for all classes and methods  
- **File Documentation** - Source code with inline comments
- **Cross-references** - Linked navigation between related items
- **Search** - Find any class, method, or concept quickly

## Example Usage

```cpp
#include "point.h"
#include "color.h"

using namespace session_cpp;

// Create a point with custom properties
Point point(10.0, 20.0, 30.0);
point.name = "my_point";
point.pointcolor = Color(255, 128, 0, 255); // Orange

// Save to JSON for cross-language usage
point.to_json("my_point.json");

// Load data created by Python or Rust
Point loaded = Point::from_json("external_point.json");
```

## Cross-Language Compatibility

This C++ implementation is fully compatible with the Rust and Python versions:

- **Consistent API** - Same method names and behavior across languages
- **JSON interop** - Objects created in any language work in all others  
- **Shared data** - Point and Color structures are identical across implementations

## Viewing the Documentation

After running `doxygen`, open `docs/html/index.html` to access:

- Complete API reference
- Usage examples  
- Cross-language compatibility guide
- Source code documentation

The documentation is generated directly from source code comments, ensuring it's always up-to-date with the implementation.
