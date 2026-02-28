#include "mini_test.h"
#include "mesh.h"
#include "point.h"
#include "line.h"
#include "polyline.h"
#include "xform.h"
#include "tolerance.h"
#include "encoders.h"

#include <cmath>
#include <filesystem>

using namespace session_cpp::mini_test;

namespace session_cpp {

    MINI_TEST("Mesh", "Constructor") {
        // uncomment #include "mesh.h"

        const int sides = 6;

        // Create hexagon vertices in XY plane
        std::vector<Point> vertices;
        for (int i = 0; i < sides; ++i) {
            double angle = 2.0 * TOLERANCE.PI * i / sides;
            double x = 1.0 * std::cos(angle);
            double y = 1.0 * std::sin(angle);
            vertices.push_back({x, y, 0.0});
        }

        std::vector<std::vector<size_t>> faces = {
            {0, 1, 2, 3, 4, 5},
        };

        Mesh mesh = Mesh::from_vertices_and_faces(vertices, faces);

        std::string sstr = mesh.str();
        std::string srepr = mesh.repr();

        // Copy (new guid)
        Mesh mcopy = mesh;

        MINI_CHECK(mesh.is_valid());
    }

    MINI_TEST("Mesh", "From Polylines") {
        // uncomment #include "mesh.h"

        std::vector<Point> hex0 = {
            Point(0.5, 0.866025, 0), Point(-0.5, 0.866025, 0), Point(-1, 0, 0),
            Point(-0.5, -0.866025, 0), Point(0.5, -0.866025, 0), Point(1, 0, 0),
        };
        std::vector<Point> hex1 = {
            Point(2, 1.732051, 0), Point(1, 1.732051, 0), Point(0.5, 0.866025, 0),
            Point(1, 0, 0), Point(2, 0, 0), Point(2.5, 0.866025, 0),
        };
        std::vector<Point> hex2 = {
            Point(2.0, 0, 0), Point(1, 0, 0), Point(0.5, -0.866025, 0),
            Point(1.0, -1.732051, 0), Point(2.0, -1.732051, 0), Point(2.5, -0.866025, 0),
        };

        Mesh mesh = Mesh::from_polylines({hex0, hex1, hex2});
        MINI_CHECK(mesh.number_of_vertices() == 13);
        MINI_CHECK(mesh.number_of_faces() == 3);
        MINI_CHECK(mesh.number_of_edges() == 15);
    }

    MINI_TEST("Mesh", "From Lines") {
        std::vector<Line> lines = {
            Line::from_points(Point(3.723844, 0.971574, 0), Point(3.218847, 0.841359, 0)),
            Line::from_points(Point(3.723844, 0.971574, 0), Point(3.852993, 0.371225, 0)),
            Line::from_points(Point(3.852993, 0.371225, 0), Point(4.593264, 0.584361, 0)),
            Line::from_points(Point(3.904452, -0.157402, 0), Point(3.236395, -0.051356, 0)),
            Line::from_points(Point(4.995604, -0.149798, 0), Point(4.411839, -0.996413, 0)),
            Line::from_points(Point(3.904452, -0.157402, 0), Point(4.29633, -0.224964, 0)),
            Line::from_points(Point(4.29633, -0.224964, 0), Point(3.57903, -0.987075, 0)),
            Line::from_points(Point(3.236395, -0.051356, 0), Point(3.218847, 0.841359, 0)),
            Line::from_points(Point(4.593264, 0.584361, 0), Point(4.995604, -0.149798, 0)),
            Line::from_points(Point(4.411839, -0.996413, 0), Point(3.57903, -0.987075, 0)),
        };

        Mesh mesh = Mesh::from_lines(lines, true);
        MINI_CHECK(mesh.number_of_vertices() == 10);
        MINI_CHECK(mesh.number_of_faces() == 8);
        MINI_CHECK(mesh.number_of_edges() == 17);
    }

    MINI_TEST("Mesh", "From_polygon_with_holes") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh = Mesh::from_polygon_with_holes({
            {{0,0,0},{4,0,0},{4,4,0},{0,4,0}},
            {{1,1,0},{3,1,0},{3,3,0},{1,3,0}},
        }, false);
        MINI_CHECK(mesh.number_of_vertices() == 8);
        MINI_CHECK(mesh.number_of_faces() > 0);
        MINI_CHECK(mesh.is_valid());

        Mesh mesh_sorted = Mesh::from_polygon_with_holes({
            {{1,1,0},{3,1,0},{3,3,0},{1,3,0}},
            {{0,0,0},{4,0,0},{4,4,0},{0,4,0}},
        }, true);
        MINI_CHECK(mesh_sorted.number_of_vertices() == 8);
        MINI_CHECK(mesh_sorted.number_of_faces() == mesh.number_of_faces());
    }

    MINI_TEST("Mesh", "Loft") {
        // uncomment #include "mesh.h"
        // uncomment #include "polyline.h"
        // uncomment #include "point.h"

        Polyline bot(std::vector<Point>{Point(0,0,0), Point(1,0,0), Point(1,1,0), Point(0,1,0), Point(0,0,0)});
        Polyline top(std::vector<Point>{Point(0,0,1), Point(1,0,1), Point(1,1,1), Point(0,1,1), Point(0,0,1)});
        Mesh mesh = Mesh::loft({bot}, {top});
        MINI_CHECK(mesh.number_of_vertices() == 8);
        MINI_CHECK(mesh.number_of_faces() == 8);
        MINI_CHECK(mesh.is_valid());

        Mesh mesh_no_cap = Mesh::loft({bot}, {top}, false);
        MINI_CHECK(mesh_no_cap.number_of_vertices() == 8);
        MINI_CHECK(mesh_no_cap.number_of_faces() == 4);

        Polyline ob(std::vector<Point>{Point(0,0,0), Point(4,0,0), Point(4,4,0), Point(0,4,0), Point(0,0,0)});
        Polyline ot(std::vector<Point>{Point(0,0,1), Point(4,0,1), Point(4,4,1), Point(0,4,1), Point(0,0,1)});
        Polyline ib(std::vector<Point>{Point(1,1,0), Point(3,1,0), Point(3,3,0), Point(1,3,0), Point(1,1,0)});
        Polyline it(std::vector<Point>{Point(1,1,1), Point(3,1,1), Point(3,3,1), Point(1,3,1), Point(1,1,1)});
        Mesh mesh_hole = Mesh::loft({ob, ib}, {ot, it});
        MINI_CHECK(mesh_hole.number_of_vertices() == 16);
        MINI_CHECK(mesh_hole.number_of_faces() == 24);
        MINI_CHECK(mesh_hole.is_valid());
    }




    MINI_TEST("Mesh", "Add_vertex") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(1.0, 2.0, 3.0), std::nullopt);

        MINI_CHECK(mesh.number_of_vertices() == 1);
        MINI_CHECK(!mesh.is_empty());

        auto pos = mesh.vertex_position(v0);
        MINI_CHECK(pos.has_value());
        MINI_CHECK((*pos)[0] == 1.0 && (*pos)[1] == 2.0 && (*pos)[2] == 3.0);

        size_t v1 = mesh.add_vertex(Point(4.0, 5.0, 6.0), 42);
        MINI_CHECK(v1 == 42);
        MINI_CHECK(mesh.number_of_vertices() == 2);
    }

    MINI_TEST("Mesh", "Add_face") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        auto f = mesh.add_face({v0, v1, v2}, std::nullopt);
        MINI_CHECK(f.has_value());
        MINI_CHECK(mesh.number_of_faces() == 1);
        MINI_CHECK(mesh.number_of_edges() == 3);
        MINI_CHECK(mesh.euler() == 1);

        auto invalid1 = mesh.add_face({v0, v1}, std::nullopt);
        MINI_CHECK(!invalid1.has_value());

        auto invalid2 = mesh.add_face({v0, v1, 999}, std::nullopt);
        MINI_CHECK(!invalid2.has_value());

        auto invalid3 = mesh.add_face({v0, v1, v0}, std::nullopt);
        MINI_CHECK(!invalid3.has_value());
    }

    MINI_TEST("Mesh", "Face_vertices") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        auto f = mesh.add_face({v0, v1, v2}, std::nullopt);
        auto vertices = mesh.face_vertices(*f);

        MINI_CHECK(vertices.has_value());
        MINI_CHECK(vertices->size() == 3);
        MINI_CHECK((*vertices)[0] == v0);
        MINI_CHECK((*vertices)[1] == v1);
        MINI_CHECK((*vertices)[2] == v2);
    }

    MINI_TEST("Mesh", "Vertex_neighbors") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        mesh.add_face({v0, v1, v2}, std::nullopt);

        auto neighbors = mesh.vertex_neighbors(v0);
        MINI_CHECK(neighbors.size() == 2);
        MINI_CHECK(std::find(neighbors.begin(), neighbors.end(), v1) != neighbors.end());
        MINI_CHECK(std::find(neighbors.begin(), neighbors.end(), v2) != neighbors.end());
    }

    MINI_TEST("Mesh", "Vertex_faces") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(1.0, 1.0, 0.0), std::nullopt);

        auto f1 = mesh.add_face({v0, v1, v2}, std::nullopt);
        auto f2 = mesh.add_face({v1, v3, v2}, std::nullopt);

        auto faces = mesh.vertex_faces(v1);
        MINI_CHECK(faces.size() == 2);
        MINI_CHECK(std::find(faces.begin(), faces.end(), *f1) != faces.end());
        MINI_CHECK(std::find(faces.begin(), faces.end(), *f2) != faces.end());
    }

    MINI_TEST("Mesh", "Is_vertex_on_boundary") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        mesh.add_face({v0, v1, v2}, std::nullopt);

        MINI_CHECK(mesh.is_vertex_on_boundary(v0));
        MINI_CHECK(mesh.is_vertex_on_boundary(v1));
        MINI_CHECK(mesh.is_vertex_on_boundary(v2));
    }

    MINI_TEST("Mesh", "Vertex_edges") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        auto f0 = mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        auto f1 = mesh.add_face({v1,v4,v2}, std::nullopt);

        auto edges = mesh.vertex_edges(v1);
        MINI_CHECK(edges.size() == 3);
        MINI_CHECK(std::find(edges.begin(), edges.end(), std::make_pair(v1,v0)) != edges.end());
        MINI_CHECK(std::find(edges.begin(), edges.end(), std::make_pair(v1,v2)) != edges.end());
        MINI_CHECK(std::find(edges.begin(), edges.end(), std::make_pair(v1,v4)) != edges.end());
    }

    MINI_TEST("Mesh", "Face_edges") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        auto f0 = mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        mesh.add_face({v1,v4,v2}, std::nullopt);

        auto edges = mesh.face_edges(*f0);
        MINI_CHECK(edges.size() == 4);
        MINI_CHECK(edges[0] == std::make_pair(v0,v1));
        MINI_CHECK(edges[1] == std::make_pair(v1,v2));
        MINI_CHECK(edges[2] == std::make_pair(v2,v3));
        MINI_CHECK(edges[3] == std::make_pair(v3,v0));
    }

    MINI_TEST("Mesh", "Face_neighbors") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        auto f0 = mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        auto f1 = mesh.add_face({v1,v4,v2}, std::nullopt);

        auto nb0 = mesh.face_neighbors(*f0);
        MINI_CHECK(nb0.size() == 1);
        MINI_CHECK(nb0[0] == *f1);

        auto nb1 = mesh.face_neighbors(*f1);
        MINI_CHECK(nb1.size() == 1);
        MINI_CHECK(nb1[0] == *f0);
    }

    MINI_TEST("Mesh", "Edge_vertices") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        mesh.add_face({v0,v1,v2}, std::nullopt);

        auto ev = mesh.edge_vertices(v0, v1);
        MINI_CHECK(ev[0] == v0);
        MINI_CHECK(ev[1] == v1);
    }

    MINI_TEST("Mesh", "Edge_faces") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        auto f0 = mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        auto f1 = mesh.add_face({v1,v4,v2}, std::nullopt);

        auto [a, b] = mesh.edge_faces(v1, v2);
        MINI_CHECK(a.has_value() && b.has_value());
        MINI_CHECK((a == f0 && b == f1) || (a == f1 && b == f0));

        auto [c, d] = mesh.edge_faces(v0, v1);
        MINI_CHECK(c.has_value() != d.has_value());
    }

    MINI_TEST("Mesh", "Edge_edges") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        mesh.add_face({v1,v4,v2}, std::nullopt);

        auto ee = mesh.edge_edges(v1, v2);
        MINI_CHECK(ee.size() == 4);
        MINI_CHECK(std::find(ee.begin(), ee.end(), std::make_pair(v1,v2)) == ee.end());
        MINI_CHECK(std::find(ee.begin(), ee.end(), std::make_pair(v2,v1)) == ee.end());
    }

    MINI_TEST("Mesh", "Is_edge_on_boundary") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        mesh.add_face({v1,v4,v2}, std::nullopt);

        MINI_CHECK(mesh.is_edge_on_boundary(v0, v1));
        MINI_CHECK(!mesh.is_edge_on_boundary(v1, v2));
    }

    MINI_TEST("Mesh", "Is_face_on_boundary") {
        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0,0,0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1,0,0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1,1,0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0,1,0), std::nullopt);
        size_t v4 = mesh.add_vertex(Point(2,0,0), std::nullopt);
        auto f0 = mesh.add_face({v0,v1,v2,v3}, std::nullopt);
        auto f1 = mesh.add_face({v1,v4,v2}, std::nullopt);

        MINI_CHECK(mesh.is_face_on_boundary(*f0));
        MINI_CHECK(mesh.is_face_on_boundary(*f1));
    }

    MINI_TEST("Mesh", "Face_normal") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"
        // uncomment #include "tolerance.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        auto f = mesh.add_face({v0, v1, v2}, std::nullopt);
        auto normal = mesh.face_normal(*f);

        MINI_CHECK(normal.has_value());
        MINI_CHECK(TOLERANCE.is_close((*normal)[2], 1.0));
        MINI_CHECK(TOLERANCE.is_close((*normal)[0], 0.0));
        MINI_CHECK(TOLERANCE.is_close((*normal)[1], 0.0));
    }

    MINI_TEST("Mesh", "Face_area") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"
        // uncomment #include "tolerance.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        auto f = mesh.add_face({v0, v1, v2}, std::nullopt);
        auto area = mesh.face_area(*f);

        MINI_CHECK(area.has_value());
        MINI_CHECK(TOLERANCE.is_close(*area, 0.5));
    }

    MINI_TEST("Mesh", "Clear") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        mesh.add_face({v0, v1, v2}, std::nullopt);

        MINI_CHECK(!mesh.is_empty());

        mesh.clear();

        MINI_CHECK(mesh.is_empty());
        MINI_CHECK(mesh.number_of_vertices() == 0);
        MINI_CHECK(mesh.number_of_faces() == 0);
    }

    MINI_TEST("Mesh", "Unify_winding") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"
        // uncomment #include "tolerance.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(1.0, 1.0, 0.0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        auto f0 = mesh.add_face({v0, v1, v2}, std::nullopt);
        auto f1 = mesh.add_face({v0, v3, v2}, std::nullopt);

        bool changed = mesh.unify_winding();

        auto n0 = mesh.face_normal(*f0);
        auto n1 = mesh.face_normal(*f1);
        double dot = (*n0)[0]*(*n1)[0] + (*n0)[1]*(*n1)[1] + (*n0)[2]*(*n1)[2];

        MINI_CHECK(changed);
        MINI_CHECK(TOLERANCE.is_close(dot, 1.0));
    }

    MINI_TEST("Mesh", "Transformation") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"
        // uncomment #include "xform.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        mesh.add_face({v0, v1, v2}, std::nullopt);

        mesh.xform = Xform::translation(10.0, 20.0, 30.0);
        Mesh mesh_transformed = mesh.transformed();
        mesh.transform();

        auto pos0 = mesh.vertex_position(v0);
        MINI_CHECK(pos0.has_value());
        MINI_CHECK((*pos0)[0] == 10.0);
        MINI_CHECK((*pos0)[1] == 20.0);
        MINI_CHECK((*pos0)[2] == 30.0);
        MINI_CHECK(mesh.xform == Xform::identity());
        MINI_CHECK(mesh_transformed.xform == Xform::identity());
    }

    MINI_TEST("Mesh", "Json_roundtrip") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        mesh.name = "test_mesh";
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        mesh.add_face({v0, v1, v2}, std::nullopt);

        //   jsondump()      │ ordered_json │ to JSON object (internal use)
        //   jsonload(j)     │ ordered_json │ from JSON object (internal use)
        //   json_dumps()    │ std::string  │ to JSON string
        //   json_loads(s)   │ std::string  │ from JSON string
        //   json_dump(path) │ file         │ write to file
        //   json_load(path) │ file         │ read from file

        std::string filename = "serialization/test_mesh.json";
        encoders::json_dump(mesh, filename);
        Mesh loaded = encoders::json_load<Mesh>(filename);

        MINI_CHECK(loaded.name == mesh.name);
        MINI_CHECK(loaded.number_of_vertices() == mesh.number_of_vertices());
        MINI_CHECK(loaded.number_of_faces() == mesh.number_of_faces());
    }

    MINI_TEST("Mesh", "Protobuf_roundtrip") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        mesh.name = "test_mesh_proto";
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        mesh.add_face({v0, v1, v2}, std::nullopt);

        std::string filename = "serialization/test_mesh.bin";
        mesh.pb_dump(filename);
        Mesh loaded = Mesh::pb_load(filename);

        MINI_CHECK(loaded.name == mesh.name);
        MINI_CHECK(loaded.number_of_vertices() == mesh.number_of_vertices());
        MINI_CHECK(loaded.number_of_faces() == mesh.number_of_faces());
        MINI_CHECK(loaded.guid == mesh.guid);
    }

    MINI_TEST("Mesh", "Vertex_position") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(1.0, 2.0, 3.0), std::nullopt);

        auto pos = mesh.vertex_position(v0);
        MINI_CHECK(pos.has_value());
        MINI_CHECK((*pos)[0] == 1.0);
        MINI_CHECK((*pos)[1] == 2.0);
        MINI_CHECK((*pos)[2] == 3.0);
        MINI_CHECK(!mesh.vertex_position(999).has_value());
    }

    MINI_TEST("Mesh", "Vertex_normal") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
        size_t v3 = mesh.add_vertex(Point(1.0, 1.0, 0.0), std::nullopt);

        mesh.add_face({v0, v1, v3}, std::nullopt);
        mesh.add_face({v0, v3, v2}, std::nullopt);

        auto normal = mesh.vertex_normal(v0);
        MINI_CHECK(normal.has_value());
        MINI_CHECK(std::abs((*normal)[2]) == 1.0);
    }

    MINI_TEST("Mesh", "To_vertices_and_faces") {
        // uncomment #include "mesh.h"
        // uncomment #include "point.h"

        Mesh mesh;
        size_t v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
        size_t v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
        size_t v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);

        mesh.add_face({v0, v1, v2}, std::nullopt);

        auto [vertices, faces] = mesh.to_vertices_and_faces();

        MINI_CHECK(vertices.size() == 3);
        MINI_CHECK(faces.size() == 1);
        MINI_CHECK(faces[0].size() == 3);
    }




} // namespace session_cpp
