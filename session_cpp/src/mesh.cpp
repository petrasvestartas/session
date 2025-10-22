#include "mesh.h"
#include <fstream>
#include <algorithm>
#include <cmath>
#include <unordered_map>
#include <limits>
#include <thread>

namespace session_cpp {

Mesh::Mesh() {
    xform = Xform::identity();
    default_vertex_attributes["x"] = 0.0;
    default_vertex_attributes["y"] = 0.0;
    default_vertex_attributes["z"] = 0.0;
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
    pointcolors.clear();
    facecolors.clear();
    linecolors.clear();
    widths.clear();
    triangle_bvh_built = false;
    triangle_bvh.reset();
    triangle_boxes_cache.clear();
    triangle_aabbs_cache.clear();
    triangle_indices_cache.clear();
    triangle_face_subidx_cache.clear();
    vertices_cache.clear();
}

size_t Mesh::add_vertex(const Point& position, std::optional<size_t> vkey) {
    size_t vertex_key = vkey.value_or(max_vertex + 1);
    
    if (vertex_key >= max_vertex) {
        max_vertex = vertex_key + 1;
    }
    
    vertex[vertex_key] = VertexData(position);
    halfedge[vertex_key] = {};
    pointcolors.push_back(Color::white());
    triangle_bvh_built = false;
    triangle_bvh.reset();
    triangle_boxes_cache.clear();
    triangle_aabbs_cache.clear();
    triangle_indices_cache.clear();
    triangle_face_subidx_cache.clear();
    vertices_cache.clear();
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
    facecolors.push_back(Color::white());
    
    for (size_t i = 0; i < vertices.size(); ++i) {
        size_t u = vertices[i];
        size_t v = vertices[(i + 1) % vertices.size()];
        
        bool is_new_edge = (halfedge[v].find(u) == halfedge[v].end());
        
        halfedge[u][v] = face_key;
        
        if (is_new_edge) {
            halfedge[v][u] = std::nullopt;
            linecolors.push_back(Color::white());
            widths.push_back(1.0);
        }
    }
    triangle_bvh_built = false;
    triangle_bvh.reset();
    triangle_boxes_cache.clear();
    triangle_aabbs_cache.clear();
    triangle_indices_cache.clear();
    triangle_face_subidx_cache.clear();
    vertices_cache.clear();
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
    double len = normal.magnitude();
    
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
    
    Vector normal_acc(0.0, 0.0, 0.0);
    
    for (size_t face_key : faces) {
        auto face_normal_opt = face_normal(face_key);
        if (!face_normal_opt) continue;
        
        const auto& fn = *face_normal_opt;
        double weight = 1.0;
        
        switch (weighting) {
            case NormalWeighting::Area:
                weight = face_area(face_key).value_or(1.0);
                break;
            case NormalWeighting::Angle:
                weight = vertex_angle_in_face(vertex_key, face_key).value_or(1.0);
                break;
            case NormalWeighting::Uniform:
                weight = 1.0;
                break;
        }
        
        normal_acc.set_x(normal_acc.x() + fn.x() * weight);
        normal_acc.set_y(normal_acc.y() + fn.y() * weight);
        normal_acc.set_z(normal_acc.z() + fn.z() * weight);
    }
    
    double len = normal_acc.magnitude();
    if (len > Tolerance::ZERO_TOLERANCE) {
        return Vector(normal_acc.x() / len, normal_acc.y() / len, normal_acc.z() / len);
    }
    return std::nullopt;
}

std::optional<double> Mesh::face_area(size_t face_key) const {
    auto vertices_opt = face_vertices(face_key);
    if (!vertices_opt.has_value() || vertices_opt->size() < 3) {
        return 0.0;
    }
    
    const auto& vertices = *vertices_opt;
    double area = 0.0;
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
        
        area += u.cross(v).magnitude() * 0.5;
    }
    return area;
}

std::optional<double> Mesh::vertex_angle_in_face(size_t vertex_key, size_t face_key) const {
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
    
    double u_len = u.magnitude();
    double v_len = v.magnitude();
    
    if (u_len < Tolerance::ZERO_TOLERANCE || v_len < Tolerance::ZERO_TOLERANCE) {
        return 0.0;
    }
    
    double cos_angle = u.dot(v) / (u_len * v_len);
    cos_angle = std::clamp(cos_angle, -1.0, 1.0);
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

Mesh Mesh::from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision) {
    Mesh mesh;
    
    std::map<std::tuple<int64_t, int64_t, int64_t>, size_t> map_eps;
    std::map<std::tuple<uint64_t, uint64_t, uint64_t>, size_t> map_exact;
    
    auto get_vkey = [&](const Point& p) -> size_t {
        if (precision.has_value()) {
            double eps = *precision;
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
            union { double f; uint64_t i; } ux, uy, uz;
            ux.f = p.x(); uy.f = p.y(); uz.f = p.z();
            auto key = std::make_tuple(ux.i, uy.i, uz.i);
            
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

std::map<size_t, size_t> Mesh::vertex_index() const {
    // Collect and sort keys to ensure consistent ordering
    std::vector<size_t> keys;
    keys.reserve(vertex.size());
    for (const auto& [key, _] : vertex) {
        keys.push_back(key);
    }
    std::sort(keys.begin(), keys.end());
    
    // Create index mapping
    std::map<size_t, size_t> index_map;
    for (size_t index = 0; index < keys.size(); ++index) {
        index_map[keys[index]] = index;
    }
    return index_map;
}

std::pair<std::vector<Point>, std::vector<std::vector<size_t>>> Mesh::to_vertices_and_faces() const {
    auto vertex_idx = vertex_index();
    std::vector<Point> vertices(vertex.size());
    
    for (const auto& [key, vdata] : vertex) {
        size_t idx = vertex_idx[key];
        vertices[idx] = vdata.position();
    }
    
    // Sort face keys to ensure consistent ordering
    std::vector<size_t> face_keys;
    face_keys.reserve(face.size());
    for (const auto& [key, _] : face) {
        face_keys.push_back(key);
    }
    std::sort(face_keys.begin(), face_keys.end());
    
    std::vector<std::vector<size_t>> faces;
    for (size_t face_key : face_keys) {
        const auto& face_vertices = face.at(face_key);
        std::vector<size_t> remapped;
        remapped.reserve(face_vertices.size());
        for (size_t v : face_vertices) {
            remapped.push_back(vertex_idx.at(v));
        }
        faces.push_back(remapped);
    }
    return {vertices, faces};
}

void Mesh::transform() {
  for (auto& [idx, vdata] : vertex) {
    Point pt(vdata.x, vdata.y, vdata.z);
    xform.transform_point(pt);
    vdata.x = pt.x();
    vdata.y = pt.y();
    vdata.z = pt.z();
  }
  xform = Xform::identity();
  triangle_bvh_built = false;
  triangle_bvh.reset();
  triangle_boxes_cache.clear();
  triangle_aabbs_cache.clear();
  triangle_indices_cache.clear();
  triangle_face_subidx_cache.clear();
  vertices_cache.clear();
}

Mesh Mesh::transformed() const {
  Mesh result = *this;
  result.transform();
  return result;
}

nlohmann::ordered_json Mesh::jsondump() const {
    nlohmann::ordered_json data;
    data["type"] = "Mesh";
    data["guid"] = guid;
    data["name"] = name;
    
    // Halfedge connectivity
    nlohmann::ordered_json halfedge_data;
    for (const auto& [u, neighbors] : halfedge) {
        nlohmann::ordered_json neighbor_data;
        for (const auto& [v, face_opt] : neighbors) {
            neighbor_data[std::to_string(v)] = face_opt.has_value() ? nlohmann::json(face_opt.value()) : nlohmann::json(nullptr);
        }
        halfedge_data[std::to_string(u)] = neighbor_data;
    }
    data["halfedge"] = halfedge_data;
    
    // Vertex data
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
    
    // Face data
    nlohmann::ordered_json face_data;
    for (const auto& [key, vertices] : face) {
        face_data[std::to_string(key)] = vertices;
    }
    data["face"] = face_data;
    
    // Face attributes
    nlohmann::ordered_json facedata_json;
    for (const auto& [key, attrs] : facedata) {
        facedata_json[std::to_string(key)] = attrs;
    }
    data["facedata"] = facedata_json;
    
    // Edge attributes
    nlohmann::ordered_json edgedata_json;
    for (const auto& [edge, attrs] : edgedata) {
        std::string edge_key = std::to_string(edge.first) + "," + std::to_string(edge.second);
        edgedata_json[edge_key] = attrs;
    }
    data["edgedata"] = edgedata_json;
    
    data["default_vertex_attributes"] = default_vertex_attributes;
    data["default_face_attributes"] = default_face_attributes;
    data["default_edge_attributes"] = default_edge_attributes;
    data["max_vertex"] = max_vertex;
    data["max_face"] = max_face;

    // Persist triangle BVH caches (optional)
    data["triangle_bvh_built"] = triangle_bvh_built;
    nlohmann::ordered_json tri_boxes = nlohmann::ordered_json::array();
    for (const auto& box : triangle_boxes_cache) {
        tri_boxes.push_back(box.jsondump());
    }
    data["triangle_boxes_cache"] = tri_boxes;

    // Deprecated: triangle_data_cache removed in favor of index-based caches
    return data;
}

Mesh Mesh::jsonload(const nlohmann::json& data) {
    Mesh mesh;
    
    if (data.contains("guid")) mesh.guid = data["guid"];
    if (data.contains("name")) mesh.name = data["name"];
    
    // Load halfedge connectivity
    if (data.contains("halfedge")) {
        for (const auto& [u_str, neighbors] : data["halfedge"].items()) {
            size_t u = std::stoull(u_str);
            mesh.halfedge[u] = {};
            for (const auto& [v_str, face_val] : neighbors.items()) {
                size_t v = std::stoull(v_str);
                if (face_val.is_null()) {
                    mesh.halfedge[u][v] = std::nullopt;
                } else {
                    mesh.halfedge[u][v] = face_val.get<size_t>();
                }
            }
        }
    }
    
    // Load vertex data
    if (data.contains("vertex")) {
        for (const auto& [key_str, vdata] : data["vertex"].items()) {
            size_t key = std::stoull(key_str);
            VertexData vertex_data;
            vertex_data.x = vdata["x"];
            vertex_data.y = vdata["y"];
            vertex_data.z = vdata["z"];
            if (vdata.contains("attributes")) {
                vertex_data.attributes = vdata["attributes"].get<std::map<std::string, double>>();
            }
            mesh.vertex[key] = vertex_data;
            if (!data.contains("halfedge")) {
                mesh.halfedge[key] = {};
            }
            if (key >= mesh.max_vertex) mesh.max_vertex = key + 1;
        }
    }
    
    // Load face data
    if (data.contains("face")) {
        for (const auto& [key_str, vertices] : data["face"].items()) {
            size_t key = std::stoull(key_str);
            mesh.face[key] = vertices.get<std::vector<size_t>>();
            if (key >= mesh.max_face) mesh.max_face = key + 1;
        }
    }
    
    // Load face attributes
    if (data.contains("facedata")) {
        for (const auto& [key_str, attrs] : data["facedata"].items()) {
            size_t key = std::stoull(key_str);
            mesh.facedata[key] = attrs.get<std::map<std::string, double>>();
        }
    }
    
    // Load edge attributes
    if (data.contains("edgedata")) {
        for (const auto& [edge_str, attrs] : data["edgedata"].items()) {
            size_t comma_pos = edge_str.find(',');
            size_t u = std::stoull(edge_str.substr(0, comma_pos));
            size_t v = std::stoull(edge_str.substr(comma_pos + 1));
            mesh.edgedata[{u, v}] = attrs.get<std::map<std::string, double>>();
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
    if (data.contains("max_vertex")) {
        mesh.max_vertex = data["max_vertex"];
    }
    if (data.contains("max_face")) {
        mesh.max_face = data["max_face"];
    }

    // Load optional triangle BVH caches
    if (data.contains("triangle_boxes_cache")) {
        mesh.triangle_boxes_cache.clear();
        for (const auto& box_json : data["triangle_boxes_cache"]) {
            mesh.triangle_boxes_cache.push_back(BoundingBox::jsonload(box_json));
        }
    }
    // triangle_data_cache deprecated: no longer loaded
    // Note: triangle_bvh_built is stored in JSON but not used during load
    // BVH is rebuilt from triangle_boxes_cache if present
    if (!mesh.triangle_boxes_cache.empty()) {
        mesh.triangle_bvh = std::make_shared<BVH>();
        mesh.triangle_bvh->build(mesh.triangle_boxes_cache);
        mesh.triangle_bvh_built = true;
    } else {
        mesh.triangle_bvh_built = false;
    }
    return mesh;
}

void Mesh::build_triangle_bvh(bool force) const {
    if (triangle_bvh_built && !force) return;

    triangle_boxes_cache.clear();
    triangle_aabbs_cache.clear();
    triangle_indices_cache.clear();
    triangle_face_subidx_cache.clear();
    vertices_cache.clear();

    auto vf = to_vertices_and_faces();
    vertices_cache = vf.first;
    const std::vector<std::vector<size_t>>& faces_vec = vf.second;

    size_t tri_count = 0;
    for (const auto& f : faces_vec) if (f.size() >= 3) tri_count += (f.size() - 2);
    triangle_aabbs_cache.resize(tri_count);
    triangle_indices_cache.resize(tri_count);
    triangle_face_subidx_cache.resize(tri_count);

    struct TriTask { uint32_t i0, i1, i2; size_t face_idx; size_t sub_idx; size_t out_idx; };
    std::vector<TriTask> tasks;
    tasks.reserve(tri_count);

    size_t out = 0;
    for (size_t fi = 0; fi < faces_vec.size(); ++fi) {
        const auto& fv = faces_vec[fi];
        if (fv.size() < 3) continue;
        for (size_t j = 1; j + 1 < fv.size(); ++j) {
            tasks.push_back(TriTask{static_cast<uint32_t>(fv[0]), static_cast<uint32_t>(fv[j]), static_cast<uint32_t>(fv[j+1]), fi, j, out});
            out++;
        }
    }

    auto worker = [&](size_t begin, size_t end){
        for (size_t k = begin; k < end; ++k) {
            const auto& t = tasks[k];
            const Point& p0 = vertices_cache[t.i0];
            const Point& p1 = vertices_cache[t.i1];
            const Point& p2 = vertices_cache[t.i2];

            double min_x = std::min({p0.x(), p1.x(), p2.x()}) - 0.001;
            double min_y = std::min({p0.y(), p1.y(), p2.y()}) - 0.001;
            double min_z = std::min({p0.z(), p1.z(), p2.z()}) - 0.001;
            double max_x = std::max({p0.x(), p1.x(), p2.x()}) + 0.001;
            double max_y = std::max({p0.y(), p1.y(), p2.y()}) + 0.001;
            double max_z = std::max({p0.z(), p1.z(), p2.z()}) + 0.001;

            double cx = (min_x + max_x) * 0.5;
            double cy = (min_y + max_y) * 0.5;
            double cz = (min_z + max_z) * 0.5;
            double hx = (max_x - min_x) * 0.5;
            double hy = (max_y - min_y) * 0.5;
            double hz = (max_z - min_z) * 0.5;

            BvhAABB bb{cx, cy, cz, hx, hy, hz};
            triangle_aabbs_cache[t.out_idx] = bb;
            triangle_indices_cache[t.out_idx] = TriangleIndex{t.i0, t.i1, t.i2};
            triangle_face_subidx_cache[t.out_idx] = {t.face_idx, t.sub_idx};
        }
    };

    unsigned int threads = std::max(1u, std::thread::hardware_concurrency());
    size_t total = tasks.size();
    size_t block = (total + threads - 1) / threads;
    std::vector<std::thread> pool;
    pool.reserve(threads);
    size_t beginIdx = 0;
    for (unsigned int ti = 0; ti < threads && beginIdx < total; ++ti) {
        size_t endIdx = std::min(beginIdx + block, total);
        pool.emplace_back(worker, beginIdx, endIdx);
        beginIdx = endIdx;
    }
    for (auto& th : pool) th.join();

    // Compute world size from object bounds (triangle AABBs)
    double min_x = std::numeric_limits<double>::infinity();
    double min_y = std::numeric_limits<double>::infinity();
    double min_z = std::numeric_limits<double>::infinity();
    double max_x = -std::numeric_limits<double>::infinity();
    double max_y = -std::numeric_limits<double>::infinity();
    double max_z = -std::numeric_limits<double>::infinity();
    for (const auto& bb : triangle_aabbs_cache) {
        double bx0 = bb.cx - bb.hx; double bx1 = bb.cx + bb.hx;
        double by0 = bb.cy - bb.hy; double by1 = bb.cy + bb.hy;
        double bz0 = bb.cz - bb.hz; double bz1 = bb.cz + bb.hz;
        if (bx0 < min_x) min_x = bx0;
        if (bx1 > max_x) max_x = bx1;
        if (by0 < min_y) min_y = by0;
        if (by1 > max_y) max_y = by1;
        if (bz0 < min_z) min_z = bz0;
        if (bz1 > max_z) max_z = bz1;
    }
    double extent_x = std::max(std::fabs(min_x), std::fabs(max_x));
    double extent_y = std::max(std::fabs(min_y), std::fabs(max_y));
    double extent_z = std::max(std::fabs(min_z), std::fabs(max_z));
    double max_extent = std::max({extent_x, extent_y, extent_z});
    double world_size = std::max(2.2 * max_extent, 10.0);

    triangle_bvh = std::make_shared<BVH>();
    triangle_bvh->build_from_aabbs(triangle_aabbs_cache.data(), triangle_aabbs_cache.size(), world_size);
    triangle_bvh_built = true;
}

bool Mesh::triangle_bvh_ray_cast(const Point& origin, const Vector& direction, std::vector<int>& candidate_ids, bool find_all) const {
    build_triangle_bvh(false);
    if (!triangle_bvh) return false;
    return triangle_bvh->ray_cast(origin, direction, candidate_ids, find_all);
}

bool Mesh::get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2) const {
    if (tri_id < 0) return false;
    size_t id = static_cast<size_t>(tri_id);
    if (id >= triangle_indices_cache.size() || id >= triangle_face_subidx_cache.size()) return false;
    const auto& tri = triangle_indices_cache[id];
    const auto& fs = triangle_face_subidx_cache[id];
    face_idx = fs.first;
    sub_idx = fs.second;
    if (tri.i0 >= vertices_cache.size() || tri.i1 >= vertices_cache.size() || tri.i2 >= vertices_cache.size()) return false;
    v0 = vertices_cache[tri.i0];
    v1 = vertices_cache[tri.i1];
    v2 = vertices_cache[tri.i2];
    return true;
}

void Mesh::clear_triangle_bvh() const {
    triangle_bvh_built = false;
    triangle_bvh.reset();
    triangle_boxes_cache.clear();
    triangle_aabbs_cache.clear();
    triangle_indices_cache.clear();
    triangle_face_subidx_cache.clear();
    vertices_cache.clear();
}

} // namespace session_cpp
