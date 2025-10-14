#include "vertex.h"

namespace session_cpp {

    std::string Vertex::to_string() const {
    return fmt::format("Vertex({}, {}, {}, {})", guid, name, attribute, index);
    }

    nlohmann::ordered_json Vertex::jsondump() const {
    return nlohmann::ordered_json{{"type", "Vertex"},
                                    {"name", name},
                                    {"guid", guid},
                                    {"attribute", attribute},
                                    {"index", index}};
    }

    Vertex Vertex::jsonload(const nlohmann::json &data) {
    Vertex vertex(data["name"], data["attribute"]);
    vertex.index = data["index"];
    vertex.guid = data["guid"];
    return vertex;
    }

}
