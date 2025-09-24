#include "objects.h"

namespace session_cpp {

std::string Objects::to_string() const {
  return fmt::format("Objects(name={}, guid={}, points={})", name, guid,
                     points->size());
}

nlohmann::ordered_json Objects::to_json_data() const {
  // Build JSON array of points
  std::vector<nlohmann::ordered_json> points_json;
  points_json.reserve(points->size());
  for (const auto &p : *points) {
    points_json.push_back(p->to_json_data());
  }

  return nlohmann::ordered_json{{"type", "Objects"},
                                {"guid", guid},
                                {"name", name},
                                {"points", points_json}};
}

Objects Objects::from_json_data(const nlohmann::json &data) {
  // Construct vector of shared_ptr<Point> from JSON data
  std::vector<std::shared_ptr<Point>> points;

  points.reserve(data["points"].size());
  for (const auto &point_data : data["points"])
    points.push_back(
        std::make_shared<Point>(Point::from_json_data(point_data)));

  // Create shared_ptr for the points vector
  auto points_ptr =
      std::make_shared<std::vector<std::shared_ptr<Point>>>(std::move(points));

  // Create Objects instance
  Objects objects(data["name"].get<std::string>(), points_ptr);

  // Set guid if provided, otherwise generate a new one
  objects.guid =
      data.contains("guid") ? data["guid"].get<std::string>() : ::guid();

  return objects;
}

void Objects::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << this->to_json_data().dump(4);
}

Objects Objects::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return Objects::from_json_data(data);
}

std::ostream &operator<<(std::ostream &os, const Objects &objects) {
  return os << objects.to_string();
}
} // namespace session_cpp
