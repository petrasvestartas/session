#include "mesh.h"
#include <fstream>
#include <algorithm>
#include <cmath>

namespace session_cpp {

Mesh::Mesh() {
    default_vertex_attributes["x"] = 0.0f;
    default_vertex_attributes["y"] = 0.0f;
    default_vertex_attributes["z"] = 0.0f;
}

size_t Mesh::number_of_edges() const {
    std::set<std::pair<size_t, size_t>> seen;
    size_t count = 0;
    
    for (const auto& [u, neighbors] : halfedge) {
        for (const auto& [v, _] : neighbors) {
            auto edge = std::minmax(u, v);
            if (seen.insert(edge).second) {
                count++;
            }
        }
    }
    
    return count;
}

int Mesh::euler() const {
    return static_cast<int>(number_of_vertices()) - 
           static_cast<int>(number_of_edges()) + 
           static_cast<int>(number_of_faces());
}

void Mesh::clear() {
    halfedge.clear();
    vertex.clear();
    face.clear();
    facedata.clear();
    edgedata.clear();
    triangulation.clear();
    max_vertex = 0;
    max_face = 0;
}

size_t Mesh::add_vertex(const Point& position, std::optional<size_t> vkey) {
    size_t vertex_key = vkey.value_or(max_vertex + 1);
    
    if (vertex_key >= max_vertex) {
        max_vertex = vertex_key + 1;
    }
    
    vertex[vertex_key] = VertexData(position);
    halfedge[vertex_key] = {};
    
    return vertex_key;
}

std::optional<size_t> Mesh::add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey) {
    if (vertices.size() < 3) {
        return std::nullopt;
    }
    
    for (size_t v : vertices) {
        if (vertex.find(v) == vertex.end()) {
            return std::nullopt;
        }
    }
    
    std::set<size_t> unique_vertices(vertices.begin(), vertices.end());
    if (unique_vertices.size() != vertices.size()) {
        return std::nullopt;
    }
    
    size_t face_key = fkey.value_or(max_face + 1);
    
    if (face_key >= max_face) {
        max_face = face_key + 1;
    }
    
    face[face_key] = vertices;
    triangulation.erase(face_key);
    
    for (size_t i = 0; i < vertices.size(); ++i) {
        size_t u = vertices[i];
        size_t v = vertices[(i + 1) % vertices.size()];
        
        halfedge[u][v] = face_key;
        
        if (halfedge[v].find(u) == halfedge[v].end()) {
            halfedge[v][u] = std::nullopt;
        }
    }
    
    return face_key;
}

std::optional<Point> Mesh::vertex_position(size_t vertex_key) const {
    auto it = vertex.find(vertex_key);
    if (it == vertex.end()) {
        return std::nullopt;
    }
    return it->second.position();
}

std::optional<std::vector<size_t>> Mesh::face_vertices(size_t face_key) const {
    auto it = face.find(face_key);
    if (it == face.end()) {
        return std::nullopt;
    }
    return it->second;
}

std::vector<size_t> Mesh::vertex_neighbors(size_t vertex_key) const {
    std::vector<size_t> neighbors;
    auto it = halfedge.find(vertex_key);
    if (it != halfedge.end()) {
        for (const auto& [v, _] : it->second) {
            neighbors.push_back(v);
        }
    }
    return neighbors;
}

std::vector<size_t> Mesh::vertex_faces(size_t vertex_key) const {
    std::vector<size_t> faces;
    for (const auto& [face_key, face_vertices] : face) {
        if (std::find(face_vertices.begin(), face_vertices.end(), vertex_key) != face_vertices.end()) {
            faces.push_back(face_key);
        }
    }
    return faces;
}

bool Mesh::is_vertex_on_boundary(size_t vertex_key) const {
    auto it = halfedge.find(vertex_key);
    if (it == halfedge.end()) {
        return false;
    }
    
    for (const auto& [v, face_opt] : it->second) {
        if (!face_opt.has_value()) {
            return true;
        }
    }
    
    for (const auto& [u, neighbors] : halfedge) {
        auto neighbor_it = neighbors.find(vertex_key);
        if (neighbor_it != neighbors.end() && !neighbor_it->second.has_value()) {
            return true;
        }
    }
    
    return false;
}

std::optional<Vector> Mesh::face_normal(size_t face_key) const {
    auto vertices_opt = face_vertices(face_key);
    if (!vertices_opt.has_value() || vertices_opt->size() < 3) {
        return std::nullopt;
    }
    
    const auto& vertices = *vertices_opt;
    auto p0_opt = vertex_position(vertices[0]);
    auto p1_opt = vertex_position(vertices[1]);
    auto p2_opt = vertex_position(vertices[2]);
    
    if (!p0_opt || !p1_opt || !p2_opt) {
        return std::nullopt;
    }
    
    const auto& p0 = *p0_opt;
    const auto& p1 = *p1_opt;
    const auto& p2 = *p2_opt;
    
    Vector u(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z());
    Vector v(p2.x() - p0.x(), p2.y() - p0.y(), p2.z() - p0.z());
    
    Vector normal = u.cross(v);
    float len = normal.magnitude();
    
    if (len > Tolerance::ZERO_TOLERANCE) {
        return Vector(normal.x() / len, normal.y() / len, normal.z() / len);
    }
    
    return std::nullopt;
}

std::optional<Vector> Mesh::vertex_normal(size_t vertex_key) const {
    return vertex_normal_weighted(vertex_key, NormalWeighting::Area);
}

std::optional<Vector> Mesh::vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting) const {
    auto faces = vertex_faces(vertex_key);
    if (faces.empty()) {
        return std::nullopt;
    }
    
    Vector normal_acc(0.0f, 0.0f, 0.0f);
    
    for (size_t face_key : faces) {
        auto face_normal_opt = face_normal(face_key);
        if (!face_normal_opt) continue;
        
        const auto& fn = *face_normal_opt;
        float weight = 1.0f;
        
        switch (weighting) {
            case NormalWeighting::Area:
                weight = face_area(face_key).value_or(1.0f);
                break;
            case NormalWeighting::Angle:
                weight = vertex_angle_in_face(vertex_key, face_key).value_or(1.0f);
                break;
            case NormalWeighting::Uniform:
                weight = 1.0f;
                break;
        }
        
        normal_acc.set_x(normal_acc.x() + fn.x() * weight);
        normal_acc.set_y(normal_acc.y() + fn.y() * weight);
        normal_acc.set_z(normal_acc.z() + fn.z() * weight);
    }
    
    float len = normal_acc.magnitude();
    if (len > Tolerance::ZERO_TOLERANCE) {
        return Vector(normal_acc.x() / len, normal_acc.y() / len, normal_acc.z() / len);
    }
    
    return std::nullopt;
}

std::optional<float> Mesh::face_area(size_t face_key) const {
    auto vertices_opt = face_vertices(face_key);
    if (!vertices_opt.has_value() || vertices_opt->size() < 3) {
        return 0.0f;
    }
    
    const auto& vertices = *vertices_opt;
    float area = 0.0f;
    auto p0_opt = vertex_position(vertices[0]);
    if (!p0_opt) return std::nullopt;
    const auto& p0 = *p0_opt;
    
    for (size_t i = 1; i < vertices.size() - 1; ++i) {
        auto p1_opt = vertex_position(vertices[i]);
        auto p2_opt = vertex_position(vertices[i + 1]);
        if (!p1_opt || !p2_opt) return std::nullopt;
        
        const auto& p1 = *p1_opt;
        const auto& p2 = *p2_opt;
        
        Vector u(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z());
        Vector v(p2.x() - p0.x(), p2.y() - p0.y(), p2.z() - p0.z());
        
        area += u.cross(v).magnitude() * 0.5f;
    }
    
    return area;
}

std::optional<float> Mesh::vertex_angle_in_face(size_t vertex_key, size_t face_key) const {
    auto vertices_opt = face_vertices(face_key);
    if (!vertices_opt) return std::nullopt;
    
    const auto& vertices = *vertices_opt;
    auto it = std::find(vertices.begin(), vertices.end(), vertex_key);
    if (it == vertices.end()) return std::nullopt;
    
    size_t vertex_index = std::distance(vertices.begin(), it);
    size_t n = vertices.size();
    size_t prev_vertex = vertices[(vertex_index + n - 1) % n];
    size_t next_vertex = vertices[(vertex_index + 1) % n];
    
    auto center_opt = vertex_position(vertex_key);
    auto prev_opt = vertex_position(prev_vertex);
    auto next_opt = vertex_position(next_vertex);
    
    if (!center_opt || !prev_opt || !next_opt) return std::nullopt;
    
    const auto& center = *center_opt;
    const auto& prev_pos = *prev_opt;
    const auto& next_pos = *next_opt;
    
    Vector u(prev_pos.x() - center.x(), prev_pos.y() - center.y(), prev_pos.z() - center.z());
    Vector v(next_pos.x() - center.x(), next_pos.y() - center.y(), next_pos.z() - center.z());
    
    float u_len = u.magnitude();
    float v_len = v.magnitude();
    
    if (u_len < Tolerance::ZERO_TOLERANCE || v_len < Tolerance::ZERO_TOLERANCE) {
        return 0.0f;
    }
    
    float cos_angle = u.dot(v) / (u_len * v_len);
    cos_angle = std::clamp(cos_angle, -1.0f, 1.0f);
    return std::acos(cos_angle);
}

std::map<size_t, Vector> Mesh::face_normals() const {
    std::map<size_t, Vector> normals;
    for (const auto& [face_key, _] : face) {
        auto normal_opt = face_normal(face_key);
        if (normal_opt) {
            normals[face_key] = *normal_opt;
        }
    }
    return normals;
}

std::map<size_t, Vector> Mesh::vertex_normals() const {
    return vertex_normals_weighted(NormalWeighting::Area);
}

std::map<size_t, Vector> Mesh::vertex_normals_weighted(NormalWeighting weighting) const {
    std::map<size_t, Vector> normals;
    for (const auto& [vertex_key, _] : vertex) {
        auto normal_opt = vertex_normal_weighted(vertex_key, weighting);
        if (normal_opt) {
            normals[vertex_key] = *normal_opt;
        }
    }
    return normals;
}

Mesh Mesh::from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<float> precision) {
    Mesh mesh;
    
    std::map<std::tuple<int64_t, int64_t, int64_t>, size_t> map_eps;
    std::map<std::tuple<uint64_t, uint64_t, uint64_t>, size_t> map_exact;
    
    auto get_vkey = [&](const Point& p) -> size_t {
        if (precision.has_value()) {
            float eps = *precision;
            int64_t kx = static_cast<int64_t>(std::round(p.x() / eps));
            int64_t ky = static_cast<int64_t>(std::round(p.y() / eps));
            int64_t kz = static_cast<int64_t>(std::round(p.z() / eps));
            auto key = std::make_tuple(kx, ky, kz);
            
            auto it = map_eps.find(key);
            if (it != map_eps.end()) {
                return it->second;
            }
            size_t vk = mesh.add_vertex(p);
            map_eps[key] = vk;
            return vk;
        } else {
            union { float f; uint32_t i; } ux, uy, uz;
            ux.f = p.x(); uy.f = p.y(); uz.f = p.z();
            auto key = std::make_tuple(
                static_cast<uint64_t>(ux.i),
                static_cast<uint64_t>(uy.i),
                static_cast<uint64_t>(uz.i)
            );
            
            auto it = map_exact.find(key);
            if (it != map_exact.end()) {
                return it->second;
            }
            size_t vk = mesh.add_vertex(p);
            map_exact[key] = vk;
            return vk;
        }
    };
    
    for (const auto& poly : polygons) {
        if (poly.size() < 3) continue;
        
        std::vector<size_t> vkeys;
        vkeys.reserve(poly.size());
        for (const auto& p : poly) {
            vkeys.push_back(get_vkey(p));
        }
        mesh.add_face(vkeys);
    }
    
    return mesh;
}

nlohmann::ordered_json Mesh::to_json_data() const {
    nlohmann::ordered_json data;
    data["type"] = "Mesh";
    data["guid"] = guid;
    data["name"] = name;
    
    nlohmann::ordered_json vertex_data;
    for (const auto& [key, vdata] : vertex) {
        nlohmann::ordered_json v;
        v["x"] = vdata.x;
        v["y"] = vdata.y;
        v["z"] = vdata.z;
        v["attributes"] = vdata.attributes;
        vertex_data[std::to_string(key)] = v;
    }
    data["vertex"] = vertex_data;
    
    nlohmann::ordered_json face_data;
    for (const auto& [key, vertices] : face) {
        face_data[std::to_string(key)] = vertices;
    }
    data["face"] = face_data;
    
    data["default_vertex_attributes"] = default_vertex_attributes;
    data["default_face_attributes"] = default_face_attributes;
    data["default_edge_attributes"] = default_edge_attributes;
    
    return data;
}

Mesh Mesh::from_json_data(const nlohmann::json& data) {
    Mesh mesh;
    
    if (data.contains("guid")) mesh.guid = data["guid"];
    if (data.contains("name")) mesh.name = data["name"];
    
    if (data.contains("vertex")) {
        for (const auto& [key_str, vdata] : data["vertex"].items()) {
            size_t key = std::stoull(key_str);
            VertexData vertex_data;
            vertex_data.x = vdata["x"];
            vertex_data.y = vdata["y"];
            vertex_data.z = vdata["z"];
            if (vdata.contains("attributes")) {
                vertex_data.attributes = vdata["attributes"].get<std::map<std::string, float>>();
            }
            mesh.vertex[key] = vertex_data;
            mesh.halfedge[key] = {};
            if (key >= mesh.max_vertex) mesh.max_vertex = key + 1;
        }
    }
    
    if (data.contains("face")) {
        for (const auto& [key_str, vertices] : data["face"].items()) {
            size_t key = std::stoull(key_str);
            mesh.add_face(vertices.get<std::vector<size_t>>(), key);
        }
    }
    
    if (data.contains("default_vertex_attributes")) {
        mesh.default_vertex_attributes = data["default_vertex_attributes"];
    }
    if (data.contains("default_face_attributes")) {
        mesh.default_face_attributes = data["default_face_attributes"];
    }
    if (data.contains("default_edge_attributes")) {
        mesh.default_edge_attributes = data["default_edge_attributes"];
    }
    
    return mesh;
}

void Mesh::to_json(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << to_json_data().dump(2);
}

Mesh Mesh::from_json(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    return from_json_data(data);
}

} // namespace session_cpp
