#include "catch_amalgamated.hpp"
#include <fstream>
#include "point.h"
#include "session.h"
#include "encoders.h"
#include <filesystem>

namespace session_cpp {

TEST_CASE("Session constructor.") {
  Session session;
  REQUIRE(session.name == "my_session");
  REQUIRE(!session.guid.empty());
  // Objects, tree, and graph are initialized by default constructor
}

TEST_CASE("Session jsondump.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  auto data = session.jsondump();
  REQUIRE(data["name"] == "my_session");
  REQUIRE(data.contains("guid"));
  REQUIRE(data["objects"]["points"].size() == 2);
  REQUIRE(data["graph"]["vertices"].size() == 2);
  REQUIRE(data["graph"]["edges"].size() == 1);
  
  encoders::json_dump(session, "test_session.json");
}

TEST_CASE("Session jsonload.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  auto data = session.jsondump();
  Session session2 = Session::jsonload(data);
  REQUIRE(session2.name == "my_session");
  REQUIRE(session2.lookup.size() == 2);
  REQUIRE(session2.graph.number_of_vertices() == 2);
}

TEST_CASE("Session file I/O with encoders.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");
  std::string filename = "test_session_roundtrip.json";

  encoders::json_dump(session, filename);
  Session loaded_session = encoders::json_load<Session>(filename);

  REQUIRE(loaded_session.name == session.name);
  REQUIRE(loaded_session.lookup.size() == session.lookup.size());
  REQUIRE(loaded_session.graph.number_of_vertices() ==
          session.graph.number_of_vertices());

  std::filesystem::remove(filename);
}

TEST_CASE("Session add_point.") {
  Session session;
  auto point = std::make_shared<Point>(1.0, 2.0, 3.0);
  session.add_point(point);

  REQUIRE(session.objects.points->size() == 1);
  REQUIRE(session.lookup.count(point->guid) == 1);
  REQUIRE(session.graph.has_node(point->guid));
}

TEST_CASE("Session add_edge.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  REQUIRE(session.graph.has_edge({point1->guid, point2->guid}));
}

TEST_CASE("Session get_object.") {
  Session session;
  auto point = std::make_shared<Point>(1.0, 2.0, 3.0);
  session.add_point(point);

  auto retrieved = session.get_object<Point>(point->guid);
  REQUIRE(retrieved != nullptr);
  REQUIRE(retrieved->guid == point->guid);
}

TEST_CASE("Session file I/O comprehensive.") {
  Session session("test_session");
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "test_connection");
  std::string filename = "test_session_comprehensive.json";

  encoders::json_dump(session, filename);
  Session loaded_session = encoders::json_load<Session>(filename);

  REQUIRE(loaded_session.name == session.name);
  REQUIRE(loaded_session.objects.points->size() ==
          session.objects.points->size());
  REQUIRE(loaded_session.graph.number_of_vertices() ==
          session.graph.number_of_vertices());
  REQUIRE(loaded_session.graph.number_of_edges() ==
          session.graph.number_of_edges());

  std::filesystem::remove(filename);
}

TEST_CASE("Session tree transformation hierarchy.") {
  Session scene("tree_transformation_test");
  
  // Helper to create box mesh
  auto create_box = [](const Point& center, double size) -> std::shared_ptr<Mesh> {
    auto mesh = std::make_shared<Mesh>();
    double h = size * 0.5;
    std::vector<Point> verts = {
      Point(center.x() - h, center.y() - h, center.z() - h),
      Point(center.x() + h, center.y() - h, center.z() - h),
      Point(center.x() + h, center.y() + h, center.z() - h),
      Point(center.x() - h, center.y() + h, center.z() - h),
      Point(center.x() - h, center.y() - h, center.z() + h),
      Point(center.x() + h, center.y() - h, center.z() + h),
      Point(center.x() + h, center.y() + h, center.z() + h),
      Point(center.x() - h, center.y() + h, center.z() + h)
    };
    for (size_t i = 0; i < verts.size(); ++i) mesh->add_vertex(verts[i], i);
    std::vector<std::vector<size_t>> faces = {
      {0,1,2,3}, {4,7,6,5}, {0,4,5,1}, {2,6,7,3}, {0,3,7,4}, {1,5,6,2}
    };
    for (const auto& f : faces) mesh->add_face(f);
    return mesh;
  };
  
  // Create boxes at same location
  auto box1 = create_box(Point(0, 0, 0), 2.0);
  box1->name = "box_1";
  auto box1_node = scene.add_mesh(box1);
  
  auto box2 = create_box(Point(0, 0, 0), 2.0);
  box2->name = "box_2";
  auto box2_node = scene.add_mesh(box2);
  
  auto box3 = create_box(Point(0, 0, 0), 2.0);
  box3->name = "box_3";
  auto box3_node = scene.add_mesh(box3);
  
  // Setup tree hierarchy
  scene.add(box1_node);
  scene.add(box2_node, box1_node);
  scene.add(box3_node, box2_node);
  
  // Apply transformations
  Point box1_top(0, 0, 1.0);
  Vector normal(0, 0, 1), x(1, 0, 0), y(0, 1, 0);
  Point xy_origin(0, 0, 0);
  Vector xy_x(1, 0, 0), xy_y(0, 1, 0), xy_z(0, 0, 1);
  
  Xform xy_to_top = Xform::plane_to_plane(xy_origin, xy_x, xy_y, xy_z,
                                           box1_top, x, y, normal);
  box1->xform = Xform::rotation_z(M_PI / 1.5) * xy_to_top;
  
  box2->xform = Xform::translation(2.0, 0, 0) * Xform::rotation_z(M_PI / 6.0);
  box3->xform = Xform::translation(2.0, 0, 0);
  
  // Extract transformed geometry
  Objects transformed = scene.get_geometry();
  
  REQUIRE(transformed.meshes->size() == 3);
  
  // Expected vertices for box_1
  std::vector<std::array<double, 3>> expected_box1 = {
    {1.36603, -0.366025, 0}, {0.366025, 1.36603, 0}, {-1.36603, 0.366025, 0},
    {-0.366025, -1.36603, 0}, {1.36603, -0.366025, 2}, {0.366025, 1.36603, 2},
    {-1.36603, 0.366025, 2}, {-0.366025, -1.36603, 2}
  };
  
  // Expected vertices for box_2
  std::vector<std::array<double, 3>> expected_box2 = {
    {0.366025, 2.09808, 0}, {-1.36603, 3.09808, 0}, {-2.36603, 1.36603, 0},
    {-0.633975, 0.366025, 0}, {0.366025, 2.09808, 2}, {-1.36603, 3.09808, 2},
    {-2.36603, 1.36603, 2}, {-0.633975, 0.366025, 2}
  };
  
  // Expected vertices for box_3
  std::vector<std::array<double, 3>> expected_box3 = {
    {-1.36603, 3.09808, 0}, {-3.09808, 4.09808, 0}, {-4.09808, 2.36603, 0},
    {-2.36603, 1.36603, 0}, {-1.36603, 3.09808, 2}, {-3.09808, 4.09808, 2},
    {-4.09808, 2.36603, 2}, {-2.36603, 1.36603, 2}
  };
  
  // Expected faces (same for all boxes)
  std::vector<std::vector<size_t>> expected_faces = {
    {0,1,2,3}, {4,7,6,5}, {0,4,5,1}, {2,6,7,3}, {0,3,7,4}, {1,5,6,2}
  };
  
  // Validate box_1
  auto& m1 = (*transformed.meshes)[0];
  REQUIRE(m1->vertex.size() == 8);
  for (size_t i = 0; i < 8; ++i) {
    const auto& v = m1->vertex.at(i);
    REQUIRE(std::abs(v.x - expected_box1[i][0]) < 1e-4);
    REQUIRE(std::abs(v.y - expected_box1[i][1]) < 1e-4);
    REQUIRE(std::abs(v.z - expected_box1[i][2]) < 1e-4);
  }
  
  // Validate box_2
  auto& m2 = (*transformed.meshes)[1];
  REQUIRE(m2->vertex.size() == 8);
  for (size_t i = 0; i < 8; ++i) {
    const auto& v = m2->vertex.at(i);
    REQUIRE(std::abs(v.x - expected_box2[i][0]) < 1e-4);
    REQUIRE(std::abs(v.y - expected_box2[i][1]) < 1e-4);
    REQUIRE(std::abs(v.z - expected_box2[i][2]) < 1e-4);
  }
  
  // Validate box_3
  auto& m3 = (*transformed.meshes)[2];
  REQUIRE(m3->vertex.size() == 8);
  for (size_t i = 0; i < 8; ++i) {
    const auto& v = m3->vertex.at(i);
    REQUIRE(std::abs(v.x - expected_box3[i][0]) < 1e-4);
    REQUIRE(std::abs(v.y - expected_box3[i][1]) < 1e-4);
    REQUIRE(std::abs(v.z - expected_box3[i][2]) < 1e-4);
  }
  
  // Validate faces (all boxes have same topology)
  for (auto* mesh : {&m1, &m2, &m3}) {
    REQUIRE((*mesh)->face.size() == 6);
    size_t face_idx = 0;
    for (const auto& [key, face] : (*mesh)->face) {
      REQUIRE(face.size() == expected_faces[face_idx].size());
      for (size_t i = 0; i < face.size(); ++i) {
        REQUIRE(face[i] == expected_faces[face_idx][i]);
      }
      face_idx++;
    }
  }
}

} // namespace session_cpp