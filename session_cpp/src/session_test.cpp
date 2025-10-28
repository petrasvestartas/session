#include "catch_amalgamated.hpp"
#include <fstream>
#include "point.h"
#include "session.h"
#include "encoders.h"
#include <filesystem>
#include <chrono>
#include <random>
#include <iostream>

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
  box1->xform = Xform::rotation_z(Tolerance::PI / 1.5) * xy_to_top;
  
  box2->xform = Xform::translation(2.0, 0, 0) * Xform::rotation_z(Tolerance::PI / 6.0);
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

TEST_CASE("Session Ray Casting (sanity)", "[perf][ray]") {
  std::cout << "\n=== Session Ray Casting (test) ===\n";
  Session scene("ray_test_perf");
  auto pt1 = std::make_shared<Point>(5, 0, 0); scene.add_point(pt1);
  auto pt2 = std::make_shared<Point>(15, 0, 0); scene.add_point(pt2);
  auto line1 = std::make_shared<Line>(Line::from_points(Point(10, -2, 0), Point(10, 2, 0))); scene.add_line(line1);
  Point p20(20, 0, 0); Vector vx(1,0,0); Vector vy(0,1,0);
  auto plane1 = std::make_shared<Plane>(p20, vx, vy); scene.add_plane(plane1);
  std::vector<Point> poly_pts = { Point(25, -1, -1), Point(25, 0, 0), Point(25, 1, 1) };
  auto polyline1 = std::make_shared<Polyline>(poly_pts); scene.add_polyline(polyline1);

  Point ray_origin(0, 0, 0);
  Vector ray_dir(1, 0, 0);

  auto t0 = std::chrono::high_resolution_clock::now();
  auto hits = scene.ray_cast(ray_origin, ray_dir, 0.5);
  auto t1 = std::chrono::high_resolution_clock::now();
  double ms = std::chrono::duration<double, std::milli>(t1 - t0).count();

  std::cout << hits.size() << " hit(s) in " << ms << " ms\n";
  REQUIRE(hits.size() >= 1);
}

TEST_CASE("Comprehensive 10k Mixed Geometry", "[perf][integration][oobb]") {
  const int OBJECT_COUNT = 10000;
  const double WORLD_SIZE = 50.0;

  std::cout << "\n=== Comprehensive 10k Mixed Geometry Test (test) ===\n";

  Session scene("comprehensive_test_perf");
  std::vector<BoundingBox> aabb_boxes; aabb_boxes.reserve(OBJECT_COUNT);
  std::vector<BoundingBox> oobb_boxes; oobb_boxes.reserve(OBJECT_COUNT);

  std::srand(42);

  for (int i = 0; i < OBJECT_COUNT; ++i) {
    double x = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
    double y = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
    double z = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
    int geom_type = i % 7;

    if (geom_type == 0) {
      auto pt = std::make_shared<Point>(x, y, z);
      scene.add_point(pt);
      aabb_boxes.push_back(BoundingBox::from_point(*pt, 0.1));
      oobb_boxes.push_back(BoundingBox::from_point(*pt, 0.1));
    } else if (geom_type == 1) {
      double dx = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 5.0;
      double dy = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 5.0;
      double dz = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 5.0;
      auto line = std::make_shared<Line>(Line::from_points(Point(x, y, z), Point(x + dx, y + dy, z + dz)));
      scene.add_line(line);
      aabb_boxes.push_back(BoundingBox::from_line(*line, 0.1));
      oobb_boxes.push_back(BoundingBox::from_line(*line, 0.1));
    } else if (geom_type == 2) {
      Point plane_pt(x, y, z);
      Vector plane_x(1, 0, 0);
      Vector plane_y(0, 1, 0);
      auto plane = std::make_shared<Plane>(plane_pt, plane_x, plane_y);
      scene.add_plane(plane);
      aabb_boxes.push_back(BoundingBox(*plane, 2.0, 2.0, 0.1));
      oobb_boxes.push_back(BoundingBox(*plane, 2.0, 2.0, 0.1));
    } else if (geom_type == 3) {
      std::vector<Point> pts;
      int num_pts = 3 + (i % 5);
      for (int j = 0; j < num_pts; ++j) {
        double jx = x + (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 3.0;
        double jy = y + (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 3.0;
        double jz = z + (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * 3.0;
        pts.emplace_back(jx, jy, jz);
      }
      auto poly = std::make_shared<Polyline>(pts);
      scene.add_polyline(poly);
      Point poly_origin; Vector poly_x, poly_y, poly_z; poly->get_average_plane(poly_origin, poly_x, poly_y, poly_z);
      Plane fit_plane(poly_origin, poly_x, poly_y);
      aabb_boxes.push_back(BoundingBox::from_polyline(*poly, 0.1));
      oobb_boxes.push_back(BoundingBox::from_polyline(*poly, fit_plane, 0.1));
    } else if (geom_type == 4) {
      auto mesh = std::make_shared<Mesh>();
      double h = 0.5;
      std::vector<Point> verts = {
        Point(x - h, y - h, z - h), Point(x + h, y - h, z - h),
        Point(x + h, y + h, z - h), Point(x - h, y + h, z - h),
        Point(x - h, y - h, z + h), Point(x + h, y - h, z + h),
        Point(x + h, y + h, z + h), Point(x - h, y + h, z + h)
      };
      for (size_t vi = 0; vi < verts.size(); ++vi) mesh->add_vertex(verts[vi], vi);
      mesh->add_face({0,1,2,3});
      mesh->add_face({4,7,6,5});
      scene.add_mesh(mesh);
      Point mesh_pt(x, y, z); Vector mesh_x(1, 0, 0); Vector mesh_y(0, 1, 0);
      Plane mesh_plane(mesh_pt, mesh_x, mesh_y);
      aabb_boxes.push_back(BoundingBox::from_mesh(*mesh, 0.1));
      oobb_boxes.push_back(BoundingBox::from_mesh(*mesh, mesh_plane, 0.1));
    } else if (geom_type == 5) {
      Line cyl_line = Line::from_points(Point(x - 1, y, z), Point(x + 1, y, z));
      auto cyl = std::make_shared<Cylinder>(cyl_line, 0.3);
      scene.add_cylinder(cyl);
      Point cyl_pt(x, y, z); Vector cyl_x(1, 0, 0); Vector cyl_y(0, 1, 0);
      Plane cyl_plane(cyl_pt, cyl_x, cyl_y);
      aabb_boxes.push_back(BoundingBox::from_cylinder(*cyl, 0.1));
      oobb_boxes.push_back(BoundingBox::from_cylinder(*cyl, cyl_plane, 0.1));
    } else {
      Line arrow_line = Line::from_points(Point(x - 1, y, z), Point(x + 1, y, z));
      auto arrow = std::make_shared<Arrow>(arrow_line, 0.3);
      scene.add_arrow(arrow);
      Point arrow_pt(x, y, z); Vector arrow_x(1, 0, 0); Vector arrow_y(0, 1, 0);
      Plane arrow_plane(arrow_pt, arrow_x, arrow_y);
      aabb_boxes.push_back(BoundingBox::from_arrow(*arrow, 0.1));
      oobb_boxes.push_back(BoundingBox::from_arrow(*arrow, arrow_plane, 0.1));
    }
  }

  std::cout << "\n(a) AABB BVH Collision Detection:" << "\n";
  auto aabb_start = std::chrono::high_resolution_clock::now();
  BVH aabb_bvh = BVH::from_boxes(aabb_boxes, WORLD_SIZE);
  auto [aabb_collisions, aabb_indices, aabb_checks] = aabb_bvh.check_all_collisions(aabb_boxes);
  auto aabb_end = std::chrono::high_resolution_clock::now();
  double aabb_ms = std::chrono::duration<double, std::milli>(aabb_end - aabb_start).count();
  (void)aabb_indices; (void)aabb_checks;
  std::cout << "  Build + query: " << aabb_ms << "ms\n";
  std::cout << "  Collision pairs: " << aabb_collisions.size() << "\n";

  std::cout << "\n(b) Ray BVH Intersection:" << "\n";
  Point ray_origin(0, 0, 0); Vector ray_dir(1, 0, 0);
  auto ray_start = std::chrono::high_resolution_clock::now();
  std::vector<int> ray_candidates; aabb_bvh.ray_cast(ray_origin, ray_dir, ray_candidates, true);
  auto ray_end = std::chrono::high_resolution_clock::now();
  double ray_ms = std::chrono::duration<double, std::milli>(ray_end - ray_start).count();
  std::cout << "  Query: " << ray_ms << "ms\n";
  std::cout << "  Candidates: " << ray_candidates.size() << "\n";

  std::cout << "\n(c) OOBB BVH Collision Detection (Optimized):" << "\n";
  auto oobb_start = std::chrono::high_resolution_clock::now();
  BVH oobb_bvh = BVH::from_boxes(oobb_boxes, WORLD_SIZE);
  auto [oobb_candidates, oobb_indices, oobb_checks] = oobb_bvh.check_all_collisions(oobb_boxes);
  (void)oobb_indices; (void)oobb_checks;
  std::cout << "  BVH broad-phase: " << oobb_candidates.size() << " candidates\n";

  auto sat_start = std::chrono::high_resolution_clock::now();
  double bvh_ms = std::chrono::duration<double, std::milli>(sat_start - oobb_start).count();
  std::cout << "  BVH build + query: " << bvh_ms << "ms\n";
  std::cout << "  Running SAT...\n";

  int true_oobb_collisions = 0;
  size_t report_interval = std::max<size_t>(1, oobb_candidates.size() / 10);
  for (size_t idx = 0; idx < oobb_candidates.size(); ++idx) {
    const auto& [i, j] = oobb_candidates[idx];
    if (oobb_boxes[i].collides_with(oobb_boxes[j])) true_oobb_collisions++;
    if (idx > 0 && idx % report_interval == 0) {
      int progress = static_cast<int>((idx * 100) / oobb_candidates.size());
      std::cout << "    Progress: " << progress << "% (" << idx << "/" << oobb_candidates.size() << ")\n";
    }
  }
  auto sat_end = std::chrono::high_resolution_clock::now();
  double sat_ms = std::chrono::duration<double, std::milli>(sat_end - sat_start).count();

  std::cout << "  SAT: " << sat_ms << "ms\n";
  std::cout << "  BVH candidate pairs: " << oobb_candidates.size() << "\n";
  std::cout << "  True OOBB collisions: " << true_oobb_collisions << "\n";

  auto session_start = std::chrono::high_resolution_clock::now();
  auto session_hits = scene.ray_cast(ray_origin, ray_dir, 1.0);
  auto session_end = std::chrono::high_resolution_clock::now();
  double session_ms = std::chrono::duration<double, std::milli>(session_end - session_start).count();

  std::cout << "Session:  " << session_ms << "ms (" << session_hits.size() << " hits)\n";

  REQUIRE(session_ms >= 0.0);
}

} // namespace session_cpp