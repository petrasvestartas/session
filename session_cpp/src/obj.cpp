#include "obj.h"
#include "mesh.h"
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

namespace session_cpp { namespace obj {

void write_obj(const Mesh& mesh, const std::string& filepath) {
    auto vf = mesh.to_vertices_and_faces();
    const auto& vertices = vf.first;
    const auto& faces = vf.second;

    std::ofstream out(filepath);
    if (!out.is_open()) {
        return; // Failed to open file
    }
    
    for (const auto& p : vertices) {
        out << "v " << p.x() << " " << p.y() << " " << p.z() << "\n";
    }
    for (const auto& face : faces) {
        if (face.size() < 3) continue;
        out << "f";
        for (auto i : face) {
            out << " " << (i + 1);
        }
        out << "\n";
    }
    
    out.close(); // Explicitly close and flush
}

Mesh read_obj(const std::string& filepath) {
    std::ifstream in(filepath);
    std::string line;
    std::vector<Point> verts;
    std::vector<std::vector<size_t>> faces;

    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') continue;
        if (line.rfind("v ", 0) == 0) {
            std::istringstream iss(line);
            char v; double x, y, z; iss >> v >> x >> y >> z;
            verts.emplace_back(x, y, z);
        } else if (line.rfind("f ", 0) == 0) {
            std::istringstream iss(line.substr(2));
            std::string tok;
            std::vector<size_t> face;
            while (iss >> tok) {
                auto slash = tok.find('/');
                std::string sidx = (slash == std::string::npos) ? tok : tok.substr(0, slash);
                if (sidx.empty()) continue;
                long long idx = std::stoll(sidx);
                if (idx == 0) continue;
                size_t vidx = (idx > 0) ? static_cast<size_t>(idx - 1)
                                        : static_cast<size_t>(static_cast<long long>(verts.size()) + idx);
                face.push_back(vidx);
            }
            if (face.size() >= 3) faces.push_back(std::move(face));
        }
    }

    Mesh mesh;
    std::vector<size_t> vkeys; vkeys.reserve(verts.size());
    for (const auto& p : verts) vkeys.push_back(mesh.add_vertex(p));
    for (const auto& f : faces) {
        std::vector<size_t> vlist; vlist.reserve(f.size());
        for (auto i : f) vlist.push_back(vkeys[i]);
        mesh.add_face(vlist);
    }
    return mesh;
}

} } // namespace session_cpp::obj
