# New Basic Class - C++ Template

## Header (src/name.h)

```cpp
#pragma once
#include <string>
#include <nlohmann/json.hpp>

namespace session_cpp {

class Name {
private:
    double _x = 0.0;
    double _y = 0.0;
    double _z = 0.0;

public:
    std::string guid;
    std::string name = "my_name";

    Name();
    Name(double x, double y, double z);
    Name(const Name& other);
    Name& operator=(const Name& other);

    double operator[](size_t i) const;
    double& operator[](size_t i);

    bool operator==(const Name& other) const;
    bool operator!=(const Name& other) const;

    std::string str() const;
    std::string repr() const;
    bool is_valid() const;

    nlohmann::ordered_json jsondump() const;
    static Name jsonload(const nlohmann::ordered_json& j);
    void json_dump(const std::string& filename) const;
    static Name json_load(const std::string& filename);

    std::string to_proto() const;
    static Name from_proto(const std::string& data);
    void protobuf_dump(const std::string& filename) const;
    static Name protobuf_load(const std::string& filename);
};

std::ostream& operator<<(std::ostream& os, const Name& obj);

} // namespace session_cpp
```

## Implementation (src/name.cpp)

```cpp
#include "name.h"
#include "guid.h"
#include <fmt/format.h>
#include <fstream>

namespace session_cpp {

Name::Name() : guid(generate_guid()) {}

Name::Name(double x, double y, double z)
    : guid(generate_guid()), _x(x), _y(y), _z(z) {}

Name::Name(const Name& other)
    : guid(generate_guid())
    , name(other.name)
    , _x(other._x), _y(other._y), _z(other._z) {}

Name& Name::operator=(const Name& other) {
    if (this != &other) {
        guid = generate_guid();
        name = other.name;
        _x = other._x; _y = other._y; _z = other._z;
    }
    return *this;
}

double Name::operator[](size_t i) const {
    switch (i) {
        case 0: return _x;
        case 1: return _y;
        case 2: return _z;
        default: throw std::out_of_range("Index out of bounds");
    }
}

double& Name::operator[](size_t i) {
    switch (i) {
        case 0: return _x;
        case 1: return _y;
        case 2: return _z;
        default: throw std::out_of_range("Index out of bounds");
    }
}

bool Name::operator==(const Name& other) const {
    return _x == other._x && _y == other._y && _z == other._z;
}

bool Name::operator!=(const Name& other) const {
    return !(*this == other);
}

std::string Name::str() const {
    return fmt::format("Name({}, {}, {})", _x, _y, _z);
}

std::string Name::repr() const {
    return fmt::format("Name(\n  name={},\n  x={},\n  y={},\n  z={}\n)",
                       name, _x, _y, _z);
}

bool Name::is_valid() const {
    return !std::isnan(_x) && !std::isnan(_y) && !std::isnan(_z);
}

std::ostream& operator<<(std::ostream& os, const Name& obj) {
    return os << obj.str();
}

} // namespace session_cpp
```

## CMakeLists.txt

Add to `MINITEST_SOURCES`:
```cmake
src/name_test.cpp
```
