# Common Methods - C++

## Header Declaration

```cpp
class ClassName {
public:
    std::string str() const;
    std::string repr() const;
    bool is_valid() const;
    // Copy via assignment operator generates new GUID
};
```

## Implementation

```cpp
#include <fmt/format.h>

std::string ClassName::str() const {
    return fmt::format("ClassName(x={}, y={})", x, y);
}

std::string ClassName::repr() const {
    return fmt::format(
        "ClassName(\n"
        "  name={},\n"
        "  x={},\n"
        "  y={}\n"
        ")", name, x, y);
}

bool ClassName::is_valid() const {
    return !std::isnan(x) && !std::isnan(y);
}
```

## Copy Constructor (New GUID)

```cpp
// In header:
ClassName(const ClassName& other);
ClassName& operator=(const ClassName& other);

// In implementation:
ClassName::ClassName(const ClassName& other)
    : guid(generate_guid())  // NEW guid
    , name(other.name)
    , _x(other._x)
    , _y(other._y)
{}

ClassName& ClassName::operator=(const ClassName& other) {
    if (this != &other) {
        guid = generate_guid();  // NEW guid
        name = other.name;
        _x = other._x;
        _y = other._y;
    }
    return *this;
}
```

## Stream Operator

```cpp
// In header (outside class, inside namespace):
std::ostream& operator<<(std::ostream& os, const ClassName& obj);

// In implementation:
std::ostream& operator<<(std::ostream& os, const ClassName& obj) {
    return os << obj.str();
}
```
