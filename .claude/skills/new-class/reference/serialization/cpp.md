# Serialization - C++

## Header Declaration

```cpp
    // JSON Serialization
    nlohmann::ordered_json jsondump() const;
    static ClassName jsonload(const nlohmann::json& data);
    std::string json_dumps() const;
    static ClassName json_loads(const std::string& json_string);
    void json_dump(const std::string& filename) const;
    static ClassName json_load(const std::string& filename);

    // Protobuf Serialization
    std::string pb_dumps() const;
    static ClassName pb_loads(const std::string& data);
    void pb_dump(const std::string& filename) const;
    static ClassName pb_load(const std::string& filename);
```

## JSON Implementation

```cpp
#include <nlohmann/json.hpp>
#include <fstream>

// Core: object serialization (alphabetical field order)
nlohmann::ordered_json ClassName::jsondump() const {
    nlohmann::ordered_json j;
    j["guid"] = guid;
    j["name"] = name;
    j["type"] = "ClassName";
    j["x"] = _x;
    j["y"] = _y;
    j["z"] = _z;
    return j;
}

// Core: object deserialization
ClassName ClassName::jsonload(const nlohmann::json& j) {
    ClassName obj;
    obj.guid = j.value("guid", obj.guid);
    obj.name = j.value("name", obj.name);
    obj._x = j.value("x", 0.0);
    obj._y = j.value("y", 0.0);
    obj._z = j.value("z", 0.0);
    return obj;
}

// String wrappers
std::string ClassName::json_dumps() const {
    return jsondump().dump();
}

ClassName ClassName::json_loads(const std::string& json_string) {
    return jsonload(nlohmann::ordered_json::parse(json_string));
}

// File wrappers
void ClassName::json_dump(const std::string& filename) const {
    std::ofstream f(filename);
    f << jsondump().dump(4);
}

ClassName ClassName::json_load(const std::string& filename) {
    std::ifstream f(filename);
    nlohmann::json j;
    f >> j;
    return jsonload(j);
}
```

## Protobuf Implementation

```cpp
#include "proto/classname.pb.h"

// Core: binary serialization
std::string ClassName::pb_dumps() const {
    session_proto::ClassName msg;
    msg.set_guid(guid);
    msg.set_name(name);
    msg.set_x(_x);
    msg.set_y(_y);
    msg.set_z(_z);
    return msg.SerializeAsString();
}

// Core: binary deserialization
ClassName ClassName::pb_loads(const std::string& data) {
    session_proto::ClassName msg;
    msg.ParseFromString(data);
    ClassName obj;
    obj.guid = msg.guid();
    obj.name = msg.name();
    obj._x = msg.x();
    obj._y = msg.y();
    obj._z = msg.z();
    return obj;
}

// File wrappers
void ClassName::pb_dump(const std::string& filename) const {
    std::ofstream f(filename, std::ios::binary);
    f << pb_dumps();
}

ClassName ClassName::pb_load(const std::string& filename) {
    std::ifstream f(filename, std::ios::binary);
    std::string data((std::istreambuf_iterator<char>(f)),
                      std::istreambuf_iterator<char>());
    return pb_loads(data);
}
```

## Nested Objects

When a class contains other serializable objects (e.g. Color, Xform):

```cpp
// In jsondump():
j["linecolor"] = linecolor.jsondump();
j["xform"] = xform.jsondump();

// In jsonload():
if (j.contains("linecolor")) obj.linecolor = Color::jsonload(j["linecolor"]);
if (j.contains("xform")) obj.xform = Xform::jsonload(j["xform"]);

// In pb_dumps():
auto* color_proto = proto.mutable_linecolor();
color_proto->set_r(linecolor.r);
// ...

// In pb_loads():
if (proto.has_linecolor()) {
    const auto& c = proto.linecolor();
    obj.linecolor.r = c.r();
    // ...
}
```
