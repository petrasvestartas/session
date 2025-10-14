#include "objects.h"

namespace session_cpp {

std::string Objects::to_string() const {
  return fmt::format("Objects(name={}, guid={}, points={})", name, guid,
                     points->size());
}

nlohmann::ordered_json Objects::jsondump() const {
  // Build JSON arrays for all geometry types (in alphabetical order)
  std::vector<nlohmann::ordered_json> arrows_json;
  arrows_json.reserve(arrows->size());
  for (const auto &a : *arrows) {
    arrows_json.push_back(a->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> bboxes_json;
  bboxes_json.reserve(bboxes->size());
  for (const auto &b : *bboxes) {
    bboxes_json.push_back(b->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> cylinders_json;
  cylinders_json.reserve(cylinders->size());
  for (const auto &c : *cylinders) {
    cylinders_json.push_back(c->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> lines_json;
  lines_json.reserve(lines->size());
  for (const auto &l : *lines) {
    lines_json.push_back(l->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> meshes_json;
  meshes_json.reserve(meshes->size());
  for (const auto &m : *meshes) {
    meshes_json.push_back(m->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> planes_json;
  planes_json.reserve(planes->size());
  for (const auto &p : *planes) {
    planes_json.push_back(p->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> points_json;
  points_json.reserve(points->size());
  for (const auto &p : *points) {
    points_json.push_back(p->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> pointclouds_json;
  pointclouds_json.reserve(pointclouds->size());
  for (const auto &pc : *pointclouds) {
    pointclouds_json.push_back(pc->jsondump());
  }
  
  std::vector<nlohmann::ordered_json> polylines_json;
  polylines_json.reserve(polylines->size());
  for (const auto &pl : *polylines) {
    polylines_json.push_back(pl->jsondump());
  }

  return nlohmann::ordered_json{{"type", "Objects"},
                                {"guid", guid},
                                {"name", name},
                                {"arrows", arrows_json},
                                {"bboxes", bboxes_json},
                                {"cylinders", cylinders_json},
                                {"lines", lines_json},
                                {"meshes", meshes_json},
                                {"planes", planes_json},
                                {"points", points_json},
                                {"pointclouds", pointclouds_json},
                                {"polylines", polylines_json}};
}

Objects Objects::jsonload(const nlohmann::json &data) {
  // Create Objects instance
  Objects objects(data["name"].get<std::string>());
  
  // Load arrows
  if (data.contains("arrows")) {
    std::vector<std::shared_ptr<Arrow>> arrows;
    arrows.reserve(data["arrows"].size());
    for (const auto &arrow_data : data["arrows"])
      arrows.push_back(std::make_shared<Arrow>(Arrow::jsonload(arrow_data)));
    *objects.arrows = std::move(arrows);
  }
  
  // Load bboxes
  if (data.contains("bboxes")) {
    std::vector<std::shared_ptr<BoundingBox>> bboxes;
    bboxes.reserve(data["bboxes"].size());
    for (const auto &bbox_data : data["bboxes"])
      bboxes.push_back(std::make_shared<BoundingBox>(BoundingBox::jsonload(bbox_data)));
    *objects.bboxes = std::move(bboxes);
  }
  
  // Load cylinders
  if (data.contains("cylinders")) {
    std::vector<std::shared_ptr<Cylinder>> cylinders;
    cylinders.reserve(data["cylinders"].size());
    for (const auto &cylinder_data : data["cylinders"])
      cylinders.push_back(std::make_shared<Cylinder>(Cylinder::jsonload(cylinder_data)));
    *objects.cylinders = std::move(cylinders);
  }
  
  // Load lines
  if (data.contains("lines")) {
    std::vector<std::shared_ptr<Line>> lines;
    lines.reserve(data["lines"].size());
    for (const auto &line_data : data["lines"])
      lines.push_back(std::make_shared<Line>(Line::jsonload(line_data)));
    *objects.lines = std::move(lines);
  }
  
  // Load meshes
  if (data.contains("meshes")) {
    std::vector<std::shared_ptr<Mesh>> meshes;
    meshes.reserve(data["meshes"].size());
    for (const auto &mesh_data : data["meshes"])
      meshes.push_back(std::make_shared<Mesh>(Mesh::jsonload(mesh_data)));
    *objects.meshes = std::move(meshes);
  }
  
  // Load planes
  if (data.contains("planes")) {
    std::vector<std::shared_ptr<Plane>> planes;
    planes.reserve(data["planes"].size());
    for (const auto &plane_data : data["planes"])
      planes.push_back(std::make_shared<Plane>(Plane::jsonload(plane_data)));
    *objects.planes = std::move(planes);
  }
  
  // Load points
  if (data.contains("points")) {
    std::vector<std::shared_ptr<Point>> points;
    points.reserve(data["points"].size());
    for (const auto &point_data : data["points"])
      points.push_back(std::make_shared<Point>(Point::jsonload(point_data)));
    *objects.points = std::move(points);
  }
  
  // Load pointclouds
  if (data.contains("pointclouds")) {
    std::vector<std::shared_ptr<PointCloud>> pointclouds;
    pointclouds.reserve(data["pointclouds"].size());
    for (const auto &pointcloud_data : data["pointclouds"])
      pointclouds.push_back(std::make_shared<PointCloud>(PointCloud::jsonload(pointcloud_data)));
    *objects.pointclouds = std::move(pointclouds);
  }
  
  // Load polylines
  if (data.contains("polylines")) {
    std::vector<std::shared_ptr<Polyline>> polylines;
    polylines.reserve(data["polylines"].size());
    for (const auto &polyline_data : data["polylines"])
      polylines.push_back(std::make_shared<Polyline>(Polyline::jsonload(polyline_data)));
    *objects.polylines = std::move(polylines);
  }

  // Set guid if provided, otherwise generate a new one
  objects.guid =
      data.contains("guid") ? data["guid"].get<std::string>() : ::guid();

  return objects;
}



std::ostream &operator<<(std::ostream &os, const Objects &objects) {
  return os << objects.to_string();
}
} // namespace session_cpp
