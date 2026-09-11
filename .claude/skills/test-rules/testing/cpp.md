# Testing - C++

## Test File Structure

```cpp
#include "mini_test.h"
#include "classname.h"
#include "tolerance.h"

using namespace session_cpp::mini_test;

namespace session_cpp {

MINI_TEST("ClassName", "constructor") {
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

MINI_TEST("ClassName", "json_roundtrip") {
    ClassName obj(1.0, 2.0, 3.0);
    obj.name = "test_json";

    std::string path = "test_classname.json";
    obj.json_dump(path);
    ClassName loaded = ClassName::json_load(path);

    MINI_CHECK(loaded.name == obj.name);
    MINI_CHECK(loaded[0] == obj[0]);
    MINI_CHECK(loaded == obj);
}

MINI_TEST("ClassName", "protobuf_roundtrip") {
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

## CMakeLists.txt Entry

```cmake
set(MINITEST_SOURCES
    # ... existing ...
    src/classname_test.cpp
)
```
