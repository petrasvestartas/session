# Common Fields - C++

## Header Declaration

```cpp
#pragma once
#include <string>
#include "color.h"
#include "xform.h"

namespace session_cpp {

class ClassName {
public:
    std::string guid;
    std::string name = "my_classname";

    // Visual classes only:
    double width = 1.0;
    Color color = Color::red();
    Xform xform;

    ClassName();
};

} // namespace session_cpp
```

## Implementation

```cpp
#include "classname.h"
#include "guid.h"

namespace session_cpp {

ClassName::ClassName() : guid(generate_guid()) {}

} // namespace session_cpp
```

## GUID Generation

```cpp
#include "guid.h"  // provides generate_guid()

// In constructor:
ClassName::ClassName() : guid(generate_guid()) {}

// In copy constructor - generate NEW guid:
ClassName::ClassName(const ClassName& other)
    : guid(generate_guid())  // NEW guid, not copied
    , name(other.name)
    , width(other.width)
    , color(other.color)
    , xform(other.xform)
{}
```
