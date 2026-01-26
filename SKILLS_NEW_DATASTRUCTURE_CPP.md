# Skill: New Datastructure in C++

## Files to Create

1. `session_cpp/src/name.h` - Header with class declaration
2. `session_cpp/src/name.cpp` - Implementation
3. `session_cpp/src/name_test.cpp` - Minitest file

## Minimal Header (name.h)

```cpp
#pragma once
#include <string>

namespace session_cpp {

class Name {
public:
    std::string guid;
    std::string name = "my_name";

    Name();

    std::string str() const;
    std::string repr() const;
    bool is_valid() const;
};

} // namespace session_cpp
```

## Minimal Implementation (name.cpp)

```cpp
#include "name.h"
#include "guid.h"
#include <fmt/format.h>

namespace session_cpp {

Name::Name() : guid(generate_guid()) {}

std::string Name::str() const {
    return fmt::format("Name()");
}

std::string Name::repr() const {
    return fmt::format("Name(name={})", name);
}

bool Name::is_valid() const {
    return true;
}

} // namespace session_cpp
```

## Minimal Test (name_test.cpp)

```cpp
#include "mini_test.h"
#include "name.h"

using namespace session_cpp::mini_test;

namespace session_cpp {

MINI_TEST("Name", "constructor") {
    Name obj;

    std::string cstr = obj.str();
    std::string crepr = obj.repr();

    MINI_CHECK(obj.is_valid() == true);
    MINI_CHECK(obj.name == "my_name");
    MINI_CHECK(!obj.guid.empty());
    MINI_CHECK(cstr == "Name()");
    MINI_CHECK(crepr.find("name=my_name") != std::string::npos);
}

REGISTER_MINI_TEST("Name", run_name_tests);

} // namespace session_cpp
```

## Register in Build System

Add to `session_cpp/CMakeLists.txt` in `MINITEST_SOURCES`:

```cmake
src/name_test.cpp
```

## Build & Test

```bash
cd session_cpp
cmake --build build --config Release --target point_minitest
./build/Release/point_minitest.exe
```
