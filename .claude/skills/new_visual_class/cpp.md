# New Visual Class - C++ Template

Extends basic class with visual properties.

## Header Additions (src/name.h)

```cpp
#include "color.h"
#include "xform.h"

class Name {
    // ... basic fields ...

public:
    double width = 1.0;
    Color color = Color::red();
    Xform xform;

    // ... basic methods ...

    void transform();
    Name transformed() const;
};
```

## Implementation Additions (src/name.cpp)

```cpp
Name::Name(const Name& other)
    : guid(generate_guid())
    , name(other.name)
    , _x(other._x), _y(other._y), _z(other._z)
    , width(other.width)
    , color(other.color)
    , xform(other.xform) {}

void Name::transform() {
    double nx = xform[0]*_x + xform[1]*_y + xform[2]*_z + xform[3];
    double ny = xform[4]*_x + xform[5]*_y + xform[6]*_z + xform[7];
    double nz = xform[8]*_x + xform[9]*_y + xform[10]*_z + xform[11];

    _x = nx;
    _y = ny;
    _z = nz;
    xform = Xform();
}

Name Name::transformed() const {
    Name result = *this;
    result.transform();
    return result;
}
```

## JSON Additions

```cpp
nlohmann::ordered_json Name::jsondump() const {
    nlohmann::ordered_json j;
    j["color"] = color.jsondump();
    j["guid"] = guid;
    j["name"] = name;
    j["type"] = "Name";
    j["width"] = width;
    j["x"] = _x;
    j["xform"] = xform.jsondump();
    j["y"] = _y;
    j["z"] = _z;
    return j;
}
```

## Test Additions

```cpp
MINI_TEST("Name", "transformation") {
    Name obj(1.0, 2.0, 3.0);
    obj.xform = Xform::translation(10.0, 0.0, 0.0);

    Name copy = obj.transformed();
    MINI_CHECK(copy[0] == 11.0);
    MINI_CHECK(obj[0] == 1.0);  // Original unchanged

    obj.transform();
    MINI_CHECK(obj[0] == 11.0);
}
```
