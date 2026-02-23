# New Visual Class - C++ Template

Extends basic class with visual properties.

## Header Template (src/name.h)

Use `///` section separators and `///` docstrings on every public method.
Order: Factory → Constructors → Accessors → domain-specific → Transformation → JSON → Protobuf → String → private Internal Helpers.

```cpp
#pragma once

#include "point.h"
#include "xform.h"
#include "color.h"
#include "guid.h"
#include "json.h"
#include <vector>
#include <string>

namespace session_cpp {

/**
 * @class Name
 * @brief One-line summary of what this class represents.
 *
 * 2-3 sentences: what it stores, how it's defined, key design choices.
 */
class Name {
public:
    std::string guid = ::guid();
    std::string name = "my_name";
    double width = 1.0;
    Color surfacecolor = Color::black();  // or pointcolors/linecolors vectors
    Xform xform = Xform::identity();

    // Core data members
    // ...

public:
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Static Factory Methods
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Create a Name from parameters. One sentence on what it does.
    static Name create(/* params */);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Constructors & Destructor
    ///////////////////////////////////////////////////////////////////////////////////////////

    Name();
    Name(const Name& other);
    Name& operator=(const Name& other);
    bool operator==(const Name& other) const;
    bool operator!=(const Name& other) const;
    ~Name();

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Accessors
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Return true if internal state is consistent and non-empty.
    bool is_valid() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Evaluation
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Evaluate a 3D point at the given parameter(s).
    Point point_at(/* params */) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Meshing (if applicable)
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Generate a triangle mesh using adaptive subdivision.
    Mesh mesh() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Apply the stored xform to all geometry and reset xform to identity.
    void transform();

    /// Return a copy with the stored xform applied.
    Name transformed() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON Serialization
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON object with fields in alphabetical order.
    nlohmann::ordered_json jsondump() const;

    /// Construct from a JSON object.
    static Name jsonload(const nlohmann::json& data);

    /// Write JSON to a file.
    void json_dump(const std::string& filename) const;

    /// Read from a JSON file.
    static Name json_load(const std::string& filename);

    /// Serialize to a JSON string.
    std::string json_dumps() const;

    /// Deserialize from a JSON string.
    static Name json_loads(const std::string& json_string);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Protobuf Serialization
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serialize to a protobuf binary string.
    std::string pb_dumps() const;

    /// Deserialize from a protobuf binary string.
    static Name pb_loads(const std::string& data);

    /// Write protobuf to a file.
    void pb_dump(const std::string& filename) const;

    /// Read from a protobuf file.
    static Name pb_load(const std::string& filename);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // String Representation
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Simple string (type and key properties).
    std::string str() const;

    /// Detailed string with all internal state.
    std::string repr() const;

    /// Stream output operator (calls str()).
    friend std::ostream& operator<<(std::ostream& os, const Name& obj);

private:
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Internal Helpers
    ///////////////////////////////////////////////////////////////////////////////////////////

    void deep_copy_from(const Name& src);
};

} // namespace session_cpp
```

## Implementation (src/name.cpp)

```cpp
Name::Name(const Name& other)
    : guid(::guid())
    , name(other.name)
    , width(other.width)
    , surfacecolor(other.surfacecolor)
    , xform(other.xform) {}

void Name::transform() {
    // Apply xform to geometry, then reset
    xform = Xform::identity();
}

Name Name::transformed() const {
    Name result = *this;
    result.transform();
    return result;
}
```

## JSON (fields alphabetically ordered)

```cpp
nlohmann::ordered_json Name::jsondump() const {
    nlohmann::ordered_json j;
    j["guid"] = guid;
    j["name"] = name;
    j["surfacecolor"] = surfacecolor.jsondump();
    j["type"] = "Name";
    j["width"] = width;
    j["xform"] = xform.jsondump();
    // ... domain fields alphabetically ...
    return j;
}
```

## Test

```cpp
MINI_TEST("Name", "transformation") {
    Name obj = Name::create(/* params */);
    obj.xform = Xform::translation(10.0, 0.0, 0.0);

    Name copy = obj.transformed();
    MINI_CHECK(/* transformed value */);
    MINI_CHECK(/* original unchanged */);

    obj.transform();
    MINI_CHECK(/* now transformed */);
}
```
