#include "catch_amalgamated.hpp"
#include "mesh.h"
#include <cmath>

namespace session_cpp {

TEST_CASE("Mesh constructor", "[mesh]") {
    Mesh mesh;
    REQUIRE(mesh.number_of_vertices() == 0);
    REQUIRE(mesh.number_of_faces() == 0);
    REQUIRE(mesh.is_empty());
    REQUIRE(mesh.euler() == 0);
}

TEST_CASE("Mesh add vertex", "[mesh]") {
    Mesh mesh;
    auto v_key = mesh.add_vertex(Point(1.0f, 2.0f, 3.0f));
    REQUIRE(mesh.number_of_vertices() == 1);
    REQUIRE(!mesh.is_empty());
    
    auto pos = mesh.vertex_position(v_key);
    REQUIRE(pos.has_value());
    REQUIRE(pos->x() == 1.0f);
    REQUIRE(pos->y() == 2.0f);
    REQUIRE(pos->z() == 3.0f);
}

TEST_CASE("Mesh add vertex with key", "[mesh]") {
    Mesh mesh;
    auto v_key = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f), 42);
    REQUIRE(v_key == 42);
    REQUIRE(mesh.number_of_vertices() == 1);
}

TEST_CASE("Mesh add face", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    auto f_key = mesh.add_face({v0, v1, v2});
    REQUIRE(f_key.has_value());
    REQUIRE(mesh.number_of_faces() == 1);
    REQUIRE(mesh.number_of_edges() == 3);
    REQUIRE(mesh.euler() == 1);
}

TEST_CASE("Mesh add face invalid", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    
    REQUIRE(!mesh.add_face({v0, v1}).has_value());
    REQUIRE(!mesh.add_face({v0, v1, 999}).has_value());
    REQUIRE(!mesh.add_face({v0, v1, v0}).has_value());
}

TEST_CASE("Mesh face vertices", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    auto f = mesh.add_face({v0, v1, v2}).value();
    auto vertices = mesh.face_vertices(f);
    REQUIRE(vertices.has_value());
    REQUIRE(vertices.value() == std::vector<size_t>{v0, v1, v2});
}

TEST_CASE("Mesh vertex neighbors", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    mesh.add_face({v0, v1, v2});
    
    auto neighbors = mesh.vertex_neighbors(v0);
    REQUIRE(neighbors.size() == 2);
    REQUIRE(std::find(neighbors.begin(), neighbors.end(), v1) != neighbors.end());
    REQUIRE(std::find(neighbors.begin(), neighbors.end(), v2) != neighbors.end());
}

TEST_CASE("Mesh vertex faces", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    auto v3 = mesh.add_vertex(Point(1.0f, 1.0f, 0.0f));
    
    auto f1 = mesh.add_face({v0, v1, v2}).value();
    auto f2 = mesh.add_face({v1, v3, v2}).value();
    
    auto faces = mesh.vertex_faces(v1);
    REQUIRE(faces.size() == 2);
    REQUIRE(std::find(faces.begin(), faces.end(), f1) != faces.end());
    REQUIRE(std::find(faces.begin(), faces.end(), f2) != faces.end());
}

TEST_CASE("Mesh vertex on boundary", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    mesh.add_face({v0, v1, v2});
    
    REQUIRE(mesh.is_vertex_on_boundary(v0));
    REQUIRE(mesh.is_vertex_on_boundary(v1));
    REQUIRE(mesh.is_vertex_on_boundary(v2));
}

TEST_CASE("Mesh face normal", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    auto f = mesh.add_face({v0, v1, v2}).value();
    auto normal = mesh.face_normal(f);
    
    REQUIRE(normal.has_value());
    REQUIRE(std::abs(normal->z() - 1.0f) < 1e-6f);
    REQUIRE(std::abs(normal->x()) < 1e-6f);
    REQUIRE(std::abs(normal->y()) < 1e-6f);
}

TEST_CASE("Mesh vertex normal", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    mesh.add_face({v0, v1, v2});
    auto normal = mesh.vertex_normal(v0);
    
    REQUIRE(normal.has_value());
    REQUIRE(std::abs(normal->z() - 1.0f) < 1e-6f);
}

TEST_CASE("Mesh face area", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    auto f = mesh.add_face({v0, v1, v2}).value();
    auto area = mesh.face_area(f);
    
    REQUIRE(area.has_value());
    REQUIRE(std::abs(area.value() - 0.5f) < 1e-6f);
}

TEST_CASE("Mesh vertex angle in face", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    
    auto f = mesh.add_face({v0, v1, v2}).value();
    auto angle = mesh.vertex_angle_in_face(v0, f);
    
    REQUIRE(angle.has_value());
    REQUIRE(std::abs(angle.value() - M_PI / 2.0f) < 1e-6f);
}

TEST_CASE("Mesh from polygons simple", "[mesh]") {
    std::vector<Point> triangle = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f)
    };
    
    auto mesh = Mesh::from_polygons({triangle});
    REQUIRE(mesh.number_of_vertices() == 3);
    REQUIRE(mesh.number_of_faces() == 1);
    REQUIRE(mesh.number_of_edges() == 3);
}

TEST_CASE("Mesh from polygons vertex merging", "[mesh]") {
    std::vector<Point> triangle1 = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f)
    };
    std::vector<Point> triangle2 = {
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    
    auto mesh = Mesh::from_polygons({triangle1, triangle2});
    REQUIRE(mesh.number_of_vertices() == 4);
    REQUIRE(mesh.number_of_faces() == 2);
}

TEST_CASE("Mesh from polygons precision", "[mesh]") {
    std::vector<Point> triangle1 = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f)
    };
    std::vector<Point> triangle2 = {
        Point(1.0000001f, 0.0f, 0.0f),
        Point(0.0f, 1.0000001f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    
    auto mesh = Mesh::from_polygons({triangle1, triangle2}, 1e-6f);
    REQUIRE(mesh.number_of_vertices() == 4);
    REQUIRE(mesh.number_of_faces() == 2);
}

TEST_CASE("Mesh clear", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    mesh.add_face({v0, v1, v2});
    
    REQUIRE(!mesh.is_empty());
    mesh.clear();
    REQUIRE(mesh.is_empty());
    REQUIRE(mesh.number_of_vertices() == 0);
    REQUIRE(mesh.number_of_faces() == 0);
}

TEST_CASE("Mesh JSON serialization", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    mesh.add_face({v0, v1, v2});
    
    auto data = mesh.to_json_data();
    auto restored = Mesh::from_json_data(data);
    
    REQUIRE(restored.number_of_vertices() == 3);
    REQUIRE(restored.number_of_faces() == 1);
    REQUIRE(restored.number_of_edges() == 3);
}

TEST_CASE("Mesh JSON file IO", "[mesh]") {
    Mesh mesh;
    auto v0 = mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    mesh.add_face({v0, v1, v2});
    
    std::string filename = "../test_mesh.json";
    mesh.to_json(filename);
    auto loaded = Mesh::from_json(filename);
    
    REQUIRE(loaded.number_of_vertices() == 3);
    REQUIRE(loaded.number_of_faces() == 1);
}

} // namespace session_cpp
