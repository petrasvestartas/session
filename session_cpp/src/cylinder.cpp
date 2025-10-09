#include "cylinder.h"
#include <cmath>
#include <fstream>

namespace session_cpp {

Cylinder::Cylinder(const Line& line, float radius) 
    : radius(radius), line(line), mesh(create_cylinder_mesh(line, radius)) {
}

std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> Cylinder::unit_cylinder_geometry() {
    std::vector<Point> vertices = {
        Point(0.5f, 0.0f, -0.5f),
        Point(0.404508f, 0.293893f, -0.5f),
        Point(0.154508f, 0.475528f, -0.5f),
        Point(-0.154508f, 0.475528f, -0.5f),
        Point(-0.404508f, 0.293893f, -0.5f),
        Point(-0.5f, 0.0f, -0.5f),
        Point(-0.404508f, -0.293893f, -0.5f),
        Point(-0.154508f, -0.475528f, -0.5f),
        Point(0.154508f, -0.475528f, -0.5f),
        Point(0.404508f, -0.293893f, -0.5f),
        Point(0.5f, 0.0f, 0.5f),
        Point(0.404508f, 0.293893f, 0.5f),
        Point(0.154508f, 0.475528f, 0.5f),
        Point(-0.154508f, 0.475528f, 0.5f),
        Point(-0.404508f, 0.293893f, 0.5f),
        Point(-0.5f, 0.0f, 0.5f),
        Point(-0.404508f, -0.293893f, 0.5f),
        Point(-0.154508f, -0.475528f, 0.5f),
        Point(0.154508f, -0.475528f, 0.5f),
        Point(0.404508f, -0.293893f, 0.5f),
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
        {9, 0, 10}, {9, 10, 19},
    };

    return {vertices, triangles};
}

Xform Cylinder::line_to_cylinder_transform(const Line& line, float radius) {
    Point start = line.start();
    Point end = line.end();
    Vector line_vec = line.to_vector();
    float length = line.length();

    Vector z_axis = line_vec;
    z_axis.normalize_self();
    
    Vector x_axis;
    if (std::abs(z_axis.z()) < 0.9f) {
        x_axis = Vector(0.0f, 0.0f, 1.0f).cross(z_axis);
        x_axis.normalize_self();
    } else {
        x_axis = Vector(1.0f, 0.0f, 0.0f).cross(z_axis);
        x_axis.normalize_self();
    }
    
    Vector y_axis = z_axis.cross(x_axis);
    y_axis.normalize_self();

    Xform scale = Xform::scale_xyz(radius * 2.0f, radius * 2.0f, length);
    
    // Create rotation matrix from column vectors
    Xform rotation;
    rotation.m[0] = x_axis.x();
    rotation.m[1] = x_axis.y();
    rotation.m[2] = x_axis.z();
    rotation.m[4] = y_axis.x();
    rotation.m[5] = y_axis.y();
    rotation.m[6] = y_axis.z();
    rotation.m[8] = z_axis.x();
    rotation.m[9] = z_axis.y();
    rotation.m[10] = z_axis.z();
    
    Point center(
        (start.x() + end.x()) * 0.5f,
        (start.y() + end.y()) * 0.5f,
        (start.z() + end.z()) * 0.5f
    );
    Xform translation = Xform::translation(center.x(), center.y(), center.z());

    return translation * rotation * scale;
}

Mesh Cylinder::transform_geometry(
    const std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>>& geometry,
    const Xform& xform
) {
    const auto& [vertices, triangles] = geometry;
    Mesh mesh;

    std::vector<size_t> vertex_keys;
    vertex_keys.reserve(vertices.size());
    for (const auto& v : vertices) {
        Point transformed = xform.transformed_point(v);
        vertex_keys.push_back(mesh.add_vertex(transformed));
    }

    for (const auto& tri : triangles) {
        std::vector<size_t> face_vertices = {
            vertex_keys[tri[0]],
            vertex_keys[tri[1]],
            vertex_keys[tri[2]]
        };
        mesh.add_face(face_vertices);
    }

    return mesh;
}

Mesh Cylinder::create_cylinder_mesh(const Line& line, float radius) {
    auto unit_cylinder = unit_cylinder_geometry();
    Xform xform = line_to_cylinder_transform(line, radius);
    return transform_geometry(unit_cylinder, xform);
}

nlohmann::ordered_json Cylinder::to_json_data() const {
    nlohmann::ordered_json data;
    data["type"] = "Cylinder";
    data["guid"] = guid;
    data["name"] = name;
    data["radius"] = radius;
    data["line"] = line.to_json_data();
    data["mesh"] = mesh.to_json_data();
    return data;
}

Cylinder Cylinder::from_json_data(const nlohmann::json& data) {
    Line line = Line::from_json_data(data["line"]);
    float radius = data["radius"];
    Cylinder cylinder(line, radius);
    
    if (data.contains("guid")) {
        cylinder.guid = data["guid"];
    }
    if (data.contains("name")) {
        cylinder.name = data["name"];
    }
    
    return cylinder;
}

void Cylinder::to_json(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << to_json_data().dump(4);
}

Cylinder Cylinder::from_json(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    return from_json_data(data);
}

} // namespace session_cpp
