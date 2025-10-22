#include "arrow.h"
#include <cmath>
#include <fstream>

namespace session_cpp {

Arrow::Arrow(const Line& line, double radius) 
    : radius(radius), line(line), mesh(create_arrow_mesh(line, radius)) {
}

std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> Arrow::unit_cylinder_geometry() {
    std::vector<Point> vertices = {
        Point(0.5, 0.0, -0.5),
        Point(0.404508, 0.293893, -0.5),
        Point(0.154508, 0.475528, -0.5),
        Point(-0.154508, 0.475528, -0.5),
        Point(-0.404508, 0.293893, -0.5),
        Point(-0.5, 0.0, -0.5),
        Point(-0.404508, -0.293893, -0.5),
        Point(-0.154508, -0.475528, -0.5),
        Point(0.154508, -0.475528, -0.5),
        Point(0.404508, -0.293893, -0.5),
        Point(0.5, 0.0, 0.5),
        Point(0.404508, 0.293893, 0.5),
        Point(0.154508, 0.475528, 0.5),
        Point(-0.154508, 0.475528, 0.5),
        Point(-0.404508, 0.293893, 0.5),
        Point(-0.5, 0.0, 0.5),
        Point(-0.404508, -0.293893, 0.5),
        Point(-0.154508, -0.475528, 0.5),
        Point(0.154508, -0.475528, 0.5),
        Point(0.404508, -0.293893, 0.5)
    };

    std::vector<std::array<size_t, 3>> triangles = {
        {0, 1, 11}, {0, 11, 10},
        {1, 2, 12}, {1, 12, 11},
        {2, 3, 13}, {2, 13, 12},
        {3, 4, 14}, {3, 14, 13},
        {4, 5, 15}, {4, 15, 14},
        {5, 6, 16}, {5, 16, 15},
        {6, 7, 17}, {6, 17, 16},
        {7, 8, 18}, {7, 18, 17},
        {8, 9, 19}, {8, 19, 18},
        {9, 0, 10}, {9, 10, 19}
    };

    return {vertices, triangles};
}

std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> Arrow::unit_cone_geometry() {
    std::vector<Point> vertices = {
        Point(0.0, 0.0, 0.5),
        Point(0.5, 0.0, -0.5),
        Point(0.353553, -0.353553, -0.5),
        Point(0.0, -0.5, -0.5),
        Point(-0.353553, -0.353553, -0.5),
        Point(-0.5, 0.0, -0.5),
        Point(-0.353553, 0.353553, -0.5),
        Point(0.0, 0.5, -0.5),
        Point(0.353553, 0.353553, -0.5)
    };

    std::vector<std::array<size_t, 3>> triangles = {
        {0, 2, 1},
        {0, 3, 2},
        {0, 4, 3},
        {0, 5, 4},
        {0, 6, 5},
        {0, 7, 6},
        {0, 8, 7},
        {0, 1, 8}
    };

    return {vertices, triangles};
}

Mesh Arrow::create_arrow_mesh(const Line& line, double radius) {
    Point start = line.start();
    Vector line_vec = line.to_vector();
    double length = line.length();

    Vector z_axis = line_vec;
    z_axis.normalize_self();
    Vector x_axis;
    if (std::abs(z_axis.z()) < 0.9) {
        x_axis = Vector(0.0, 0.0, 1.0).cross(z_axis);
        x_axis.normalize_self();
    } else {
        x_axis = Vector(1.0, 0.0, 0.0).cross(z_axis);
        x_axis.normalize_self();
    }
    Vector y_axis = z_axis.cross(x_axis);
    y_axis.normalize_self();

    double cone_length = length * 0.2;
    double body_length = length * 0.8;

    Point body_center(
        start.x() + line_vec.x() * 0.4,
        start.y() + line_vec.y() * 0.4,
        start.z() + line_vec.z() * 0.4
    );

    Point cone_base_center(
        start.x() + line_vec.x() * 0.9,
        start.y() + line_vec.y() * 0.9,
        start.z() + line_vec.z() * 0.9
    );

    Xform body_scale = Xform::scale_xyz(radius * 2.0, radius * 2.0, body_length);
    Point origin(0.0, 0.0, 0.0);
    Xform rotation = Xform::change_basis(origin, x_axis, y_axis, z_axis);
    Xform body_translation = Xform::translation(body_center.x(), body_center.y(), body_center.z());
    Xform body_xform = body_translation * rotation * body_scale;

    Xform cone_scale = Xform::scale_xyz(radius * 3.0, radius * 3.0, cone_length);
    Xform cone_translation = Xform::translation(cone_base_center.x(), cone_base_center.y(), cone_base_center.z());
    Xform cone_xform = cone_translation * rotation * cone_scale;

    auto body_geometry = unit_cylinder_geometry();
    auto cone_geometry = unit_cone_geometry();

    Mesh mesh;

    std::vector<size_t> body_vertex_map;
    for (const auto& v : body_geometry.first) {
        Point transformed = body_xform.transformed_point(v);
        body_vertex_map.push_back(mesh.add_vertex(transformed));
    }

    for (const auto& tri : body_geometry.second) {
        std::vector<size_t> face_vertices = {
            body_vertex_map[tri[0]],
            body_vertex_map[tri[1]],
            body_vertex_map[tri[2]]
        };
        mesh.add_face(face_vertices);
    }

    std::vector<size_t> cone_vertex_map;
    for (const auto& v : cone_geometry.first) {
        Point transformed = cone_xform.transformed_point(v);
        cone_vertex_map.push_back(mesh.add_vertex(transformed));
    }

    for (const auto& tri : cone_geometry.second) {
        std::vector<size_t> face_vertices = {
            cone_vertex_map[tri[0]],
            cone_vertex_map[tri[1]],
            cone_vertex_map[tri[2]]
        };
        mesh.add_face(face_vertices);
    }

    return mesh;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void Arrow::transform() {
  line.transform();
  xform = Xform::identity();
}

Arrow Arrow::transformed() const {
  Arrow result = *this;
  result.transform();
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Arrow::jsondump() const {
    nlohmann::ordered_json j;
    j["type"] = "Arrow";
    j["guid"] = guid;
    j["name"] = name;
    j["radius"] = radius;
    j["line"] = line.jsondump();
    j["mesh"] = mesh.jsondump();
    j["xform"] = xform.jsondump();
    return j;
}

Arrow Arrow::jsonload(const nlohmann::json& data) {
    Line line = Line::jsonload(data["line"]);
    double radius = data["radius"];
    Arrow arrow(line, radius);
    
    if (data.contains("guid")) {
        arrow.guid = data["guid"];
    }
    if (data.contains("name")) {
        arrow.name = data["name"];
    }
    if (data.contains("xform")) {
        arrow.xform = Xform::jsonload(data["xform"]);
    }
    
    return arrow;
}



} // namespace session_cpp
