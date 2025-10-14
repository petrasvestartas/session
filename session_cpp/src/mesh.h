#pragma once
#include "point.h"
#include "vector.h"
#include "color.h"
#include "xform.h"
#include "xform.h"
#include "tolerance.h"
#include "json.h"
#include <map>
#include <set>
#include <vector>
#include <string>
#include <optional>
#include <cmath>

namespace session_cpp {


/// Normal weighting scheme for vertex normal computation
enum class NormalWeighting {
    Area,    ///< Weight by face area
    Angle,   ///< Weight by vertex angle in face
    Uniform  ///< Uniform weighting
};

/// Vertex data containing position and attributes
struct VertexData {
    float x = 0.0f;
    float y = 0.0f;
    float z = 0.0f;
    std::map<std::string, float> attributes;

    VertexData() = default;
    VertexData(const Point& p) : x(p.x()), y(p.y()), z(p.z()) {}

    /// Get vertex position as Point
    Point position() const { return Point(x, y, z); }
    
    /// Set vertex position from Point
    void set_position(const Point& p) {
        x = p.x();
        y = p.y();
        z = p.z();
    }

    /// Get vertex color as RGB array
    std::array<float, 3> color() const {
        return {
            attributes.count("r") ? attributes.at("r") : 0.5f,
            attributes.count("g") ? attributes.at("g") : 0.5f,
            attributes.count("b") ? attributes.at("b") : 0.5f
        };
    }

    /// Set vertex color
    void set_color(float r, float g, float b) {
        attributes["r"] = r;
        attributes["g"] = g;
        attributes["b"] = b;
    }

    /// Get vertex normal if set
    std::optional<std::array<float, 3>> normal() const {
        if (attributes.count("nx") && attributes.count("ny") && attributes.count("nz")) {
            return std::array<float, 3>{
                attributes.at("nx"),
                attributes.at("ny"),
                attributes.at("nz")
            };
        }
        return std::nullopt;
    }

    /// Set vertex normal
    void set_normal(float nx, float ny, float nz) {
        attributes["nx"] = nx;
        attributes["ny"] = ny;
        attributes["nz"] = nz;
    }
};

/**
 * @class Mesh
 * @brief A halfedge mesh data structure for representing polygonal surfaces.
 */
class Mesh {
public:
    std::map<size_t, std::map<size_t, std::optional<size_t>>> halfedge;  ///< Halfedge connectivity
    std::map<size_t, VertexData> vertex;                                  ///< Vertex data
    std::map<size_t, std::vector<size_t>> face;                          ///< Face vertex lists
    std::map<size_t, std::map<std::string, float>> facedata;             ///< Face attributes
    std::map<std::pair<size_t, size_t>, std::map<std::string, float>> edgedata;  ///< Edge attributes
    std::map<std::string, float> default_vertex_attributes;              ///< Default vertex attrs
    std::map<std::string, float> default_face_attributes;                ///< Default face attrs
    std::map<std::string, float> default_edge_attributes;                ///< Default edge attrs
    std::string guid = ::guid();                                         ///< Unique identifier
    std::string name = "my_mesh";                                        ///< Mesh name
    std::vector<Color> pointcolors;                                      ///< Vertex colors
    std::vector<Color> facecolors;                                       ///< Face colors
    std::vector<Color> linecolors;                                       ///< Edge colors
    std::vector<float> widths;                                           ///< Edge widths
    Xform xform;                                     ///< Transformation matrix

private:
    size_t max_vertex = 0;                                               ///< Next vertex key
    size_t max_face = 0;                                                 ///< Next face key
    std::map<size_t, std::vector<std::array<size_t, 3>>> triangulation; ///< Cached triangulations

public:

    /// Constructor
    Mesh();

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Basic Queries
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Get number of vertices
    size_t number_of_vertices() const { return vertex.size(); }
    
    /// Get number of faces
    size_t number_of_faces() const { return face.size(); }
    
    /// Get number of edges
    size_t number_of_edges() const;
    
    /// Check if mesh is empty
    bool is_empty() const { return vertex.empty(); }
    
    /// Calculate Euler characteristic (V - E + F)
    int euler() const;

    /// Clear all mesh data
    void clear();

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Vertex and Face Operations
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Add a vertex to the mesh.
     * @param position The position of the vertex.
     * @param vkey Optional vertex key.
     * @return The vertex key.
     */
    size_t add_vertex(const Point& position, std::optional<size_t> vkey = std::nullopt);
    
    /**
     * @brief Add a face to the mesh.
     * @param vertices The vertex keys forming the face.
     * @param fkey Optional face key.
     * @return The face key, or nullopt if invalid.
     */
    std::optional<size_t> add_face(const std::vector<size_t>& vertices, std::optional<size_t> fkey = std::nullopt);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Connectivity Queries
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Get the position of a vertex
    std::optional<Point> vertex_position(size_t vertex_key) const;
    
    /// Get the vertices of a face
    std::optional<std::vector<size_t>> face_vertices(size_t face_key) const;
    
    /// Get neighboring vertices of a vertex
    std::vector<size_t> vertex_neighbors(size_t vertex_key) const;
    
    /// Get faces incident to a vertex
    std::vector<size_t> vertex_faces(size_t vertex_key) const;
    
    /// Check if a vertex is on the boundary
    bool is_vertex_on_boundary(size_t vertex_key) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Geometric Properties
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Calculate the normal of a face
    std::optional<Vector> face_normal(size_t face_key) const;
    
    /// Calculate the normal of a vertex (area-weighted)
    std::optional<Vector> vertex_normal(size_t vertex_key) const;
    
    /// Calculate the normal of a vertex with specified weighting
    std::optional<Vector> vertex_normal_weighted(size_t vertex_key, NormalWeighting weighting) const;
    
    /// Calculate the area of a face
    std::optional<float> face_area(size_t face_key) const;
    
    /// Calculate the angle at a vertex in a face
    std::optional<float> vertex_angle_in_face(size_t vertex_key, size_t face_key) const;

    /// Calculate normals for all faces
    std::map<size_t, Vector> face_normals() const;
    
    /// Calculate normals for all vertices (area-weighted)
    std::map<size_t, Vector> vertex_normals() const;
    
    /// Calculate normals for all vertices with specified weighting
    std::map<size_t, Vector> vertex_normals_weighted(NormalWeighting weighting) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // COMPAS-style Export Methods
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Create a mapping from sparse vertex keys to sequential indices.
     * @return A map of vertex_key -> sequential_index (0, 1, 2, ...).
     */
    std::map<size_t, size_t> vertex_index() const;

    /**
     * @brief Export vertices and faces with sequential 0-based indices.
     * @return A pair of (vertices, faces) where faces use sequential indices.
     */
    std::pair<std::vector<Point>, std::vector<std::vector<size_t>>> to_vertices_and_faces() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Construction
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Create a mesh from a list of polygons.
     * @param polygons List of polygons as point lists.
     * @param precision Optional precision for vertex merging.
     * @return The constructed mesh.
     */
    static Mesh from_polygons(const std::vector<std::vector<Point>>& polygons, std::optional<float> precision = std::nullopt);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON-serializable object
    nlohmann::ordered_json jsondump() const;
    
    /// Create mesh from JSON data
    static Mesh jsonload(const nlohmann::json& data);
    
    /// Serialize to JSON file
    
    /// Deserialize from JSON file
};

} // namespace session_cpp
