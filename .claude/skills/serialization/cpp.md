# Serialization - C++

## JSON (nlohmann/json)

```cpp
#include <nlohmann/json.hpp>
#include <fstream>

// Use ordered_json for alphabetical output
using ordered_json = nlohmann::ordered_json;

ordered_json ClassName::jsondump() const {
    ordered_json j;
    j["guid"] = guid;
    j["name"] = name;
    j["type"] = "ClassName";
    j["x"] = _x;
    j["y"] = _y;
    j["z"] = _z;
    return j;
}

ClassName ClassName::jsonload(const ordered_json& j) {
    ClassName obj;
    obj.guid = j.at("guid").get<std::string>();
    obj.name = j.at("name").get<std::string>();
    obj._x = j.at("x").get<double>();
    obj._y = j.at("y").get<double>();
    obj._z = j.at("z").get<double>();
    return obj;
}

void ClassName::json_dump(const std::string& filename) const {
    std::ofstream f(filename);
    f << jsondump().dump(2);
}

ClassName ClassName::json_load(const std::string& filename) {
    std::ifstream f(filename);
    ordered_json j;
    f >> j;
    return jsonload(j);
}
```

## Protobuf

```cpp
#include "proto/classname.pb.h"

std::string ClassName::to_proto() const {
    proto::ClassName msg;
    msg.set_guid(guid);
    msg.set_name(name);
    msg.set_x(_x);
    msg.set_y(_y);
    msg.set_z(_z);
    return msg.SerializeAsString();
}

ClassName ClassName::from_proto(const std::string& data) {
    proto::ClassName msg;
    msg.ParseFromString(data);
    ClassName obj;
    obj.guid = msg.guid();
    obj.name = msg.name();
    obj._x = msg.x();
    obj._y = msg.y();
    obj._z = msg.z();
    return obj;
}

void ClassName::protobuf_dump(const std::string& filename) const {
    std::ofstream f(filename, std::ios::binary);
    f << to_proto();
}

ClassName ClassName::protobuf_load(const std::string& filename) {
    std::ifstream f(filename, std::ios::binary);
    std::string data((std::istreambuf_iterator<char>(f)),
                      std::istreambuf_iterator<char>());
    return from_proto(data);
}
```
