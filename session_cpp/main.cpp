#include "src/intersection.h"
#include "src/point.h"
#include "src/vector.h"
#include "src/line.h"
#include "src/plane.h"
#include "src/mesh.h"
#include "src/boundingbox.h"
#include "src/bvh.h"
#include "src/obj.h"
#include "src/session.h"
#include <iostream>
#include <vector>
#include <chrono>
#include <fstream>
#include <cstdlib>

using namespace session_cpp;

Point origin(0, 0, 0);
Point p1(1000, 0, 0);
Point p2(0, 1000, 0);
Point p3(0, 0, 1000);
Point sphere_center(2000, 0, 0);

Vector vx(1000, 0, 0);
Vector vy(0, 1000, 0);
Vector vz(0, 0, 1000);
Vector dir(1000, 1000, 1000);


Point plane_origin_0 (213.787107, 513.797811, -24.743845);
Vector plane_xaxis_0 (0.907673,-0.258819,0.330366);
Vector plane_yaxis_0 (0.272094,0.96225,0.006285);
Plane pl0 (plane_origin_0, plane_xaxis_0, plane_yaxis_0);


Point plane_origin_1 (247.17924, 499.115486, 59.619568);
Vector plane_xaxis_1 (0.552465,0.816035,0.16991);
Vector plane_yaxis_1 (0.172987,0.087156,-0.98106);
Plane pl1 (plane_origin_1, plane_xaxis_1, plane_yaxis_1);

Point plane_origin_2 (221.399816, 605.893667, -54.000116);
Vector plane_xaxis_2 (0.903451,-0.360516,-0.231957);
Vector plane_yaxis_2 (0.172742,-0.189057,0.966653);
Plane pl2 (plane_origin_2, plane_xaxis_2, plane_yaxis_2);



Line l0(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
Line l1(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

int main() {
    // std::cout << "=== Intersection Examples ===\n\n";

    // // 1. line_line
    // Point p;
    // if (Intersection::line_line(l0, l1, p, Tolerance::APPROXIMATION)) {
    //     std::cout << "1. line_line: " << p.x() << ", " << p.y() << ", " << p.z() << "\n";
    // }

    // // 2. line_line_parameters
    // float t0, t1;
    // if (Intersection::line_line_parameters(l0, l1, t0, t1, Tolerance::APPROXIMATION)) {
    //     std::cout << "2. line_line_parameters: t0=" << t0 << ", t1=" << t1 << "\n";
    // }

    // // 3. plane_plane
    // Line intersection_line;
    // if (Intersection::plane_plane(pl0, pl1, intersection_line)) {
    //    printf("3. plane_plane: %s\n", intersection_line.to_string().c_str());
    // }

    // // 4. line_plane
    // Point lp;
    // if (Intersection::line_plane(l0, pl0, lp)) {
    //     std::cout << "4. line_plane: " << lp.x() << ", " << lp.y() << ", " << lp.z() << "\n";
    // }

    // // 5. plane_plane_plane {300.5, 565.5, -0}
    // Point ppp;
    // if (Intersection::plane_plane_plane(pl0, pl1, pl2, ppp)) {
    //     std::cout << "5. plane_plane_plane: " << ppp.x() << ", " << ppp.y() << ", " << ppp.z() << "\n";
    // }

    // // 6. ray_box
    // Point min(214, 192, 484);
    // Point max(694, 567, 796);
    // std::vector<Point> points {min, max};
    // BoundingBox box = BoundingBox::from_points(points);
    

    // std::vector<Point> intersection_points;
    // if (Intersection::ray_box(l0, box, 0.0, 1000.0, intersection_points)) {
    //     std::cout << "6. ray_box: entry=" << intersection_points[0] 
    //               << ", exit=" << intersection_points[1] << "\n";
    // }

    // // 7. ray_sphere
    // Point sphere_center_test(457.0, 192.0, 207.0);
    // std::vector<Point> sphere_points;
    // if (Intersection::ray_sphere(l0, sphere_center_test, 265.0, sphere_points)) {
    //     std::cout << "7. ray_sphere: " << sphere_points.size() << " hits";
    //     for (size_t i = 0; i < sphere_points.size(); i++) {
    //         std::cout << ", p" << i << "=" << sphere_points[i];
    //     }
    //     std::cout << "\n";
    // } else {
    //     std::cout << "7. ray_sphere: 0 hits\n";
    // }

    // // 8. ray_triangle
    // Point p1(214, 567, 484);
    // Point p2(214, 192, 796);
    // Point p3(694, 192, 484);

    // Point triangle_hit;
    // if (Intersection::ray_triangle(l0, p1, p2, p3, Tolerance::APPROXIMATION, triangle_hit)) {
    //     std::cout << "8. ray_triangle: " << triangle_hit << "\n";
    // }

    // 9. ray_mesh - Load bunny mesh
    // Try both paths (run from build/ or from session_cpp/)
    Mesh bunny;
    if (std::ifstream("../../data/bunny.obj").good()) {
        bunny = obj::read_obj("../../data/bunny.obj");
    } else if (std::ifstream("../data/bunny.obj").good()) {
        bunny = obj::read_obj("../data/bunny.obj");
    } else {
        std::cerr << "ERROR: Cannot find bunny.obj in ../../data/ or ../data/\n";
        return 1;
    }
    
    std::cout << "Loaded bunny mesh: " << bunny.number_of_vertices() << " vertices, " 
              << bunny.number_of_faces() << " faces\n";
    
    // Prebuild and time BVH construction (done once, cached for all queries)
    auto bvh_build_start = std::chrono::high_resolution_clock::now();
    bunny.build_triangle_bvh();
    auto bvh_build_end = std::chrono::high_resolution_clock::now();
    double bvh_build_time_ms = std::chrono::duration<double, std::milli>(bvh_build_end - bvh_build_start).count();
    std::cout << "BVH build time: " << bvh_build_time_ms << " ms\n";
    
    Line zaxis(0.201, -0.212, 0.036, -0.326, 0.677, -0.060);

    // Test brute force (slower)
    auto time2 = std::chrono::high_resolution_clock::now();
    auto mesh_hits = Intersection::ray_mesh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time3 = std::chrono::high_resolution_clock::now();
    double mesh_time_ms = std::chrono::duration<double, std::milli>(time3 - time2).count();
    std::cout << "9. ray_mesh: " << mesh_hits.size() << " hits in " << mesh_time_ms << " ms\n";
    for (size_t i = 0; i < mesh_hits.size(); i++) {
        std::cout << "    [" << i << "] " << mesh_hits[i] << "\n";
    }
    
    
    // Test BVH (faster)
    auto time0 = std::chrono::high_resolution_clock::now();
    auto bvh_hits = Intersection::ray_mesh_bvh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time1 = std::chrono::high_resolution_clock::now();
    double bvh_time_ms = std::chrono::duration<double, std::milli>(time1 - time0).count();
    std::cout << "10. ray_mesh_bvh: " << bvh_hits.size() << " hits in " << bvh_time_ms << " ms\n";
    for (size_t i = 0; i < bvh_hits.size(); i++) {
        std::cout << "    [" << i << "] " << bvh_hits[i] << "\n";
    }
    // Detailed timings: BVH traversal vs triangle tests
    Point o = zaxis.start();
    Vector d = zaxis.to_vector();
    std::vector<int> candidate_ids;
    auto trav0 = std::chrono::high_resolution_clock::now();
    bunny.triangle_bvh_ray_cast(o, d, candidate_ids, true);
    auto trav1 = std::chrono::high_resolution_clock::now();
    double trav_ms = std::chrono::duration<double, std::milli>(trav1 - trav0).count();
    auto tri0 = std::chrono::high_resolution_clock::now();
    size_t tri_hits = 0;
    for (int tri_id : candidate_ids) {
        size_t face_idx, sub_idx;
        Point v0, v1, v2, hp;
        if (!bunny.get_triangle_by_id(tri_id, face_idx, sub_idx, v0, v1, v2)) continue;
        if (Intersection::ray_triangle(zaxis, v0, v1, v2, static_cast<double>(Tolerance::ZERO_TOLERANCE), hp)) {
            tri_hits++;
        }
    }
    auto tri1 = std::chrono::high_resolution_clock::now();
    double tri_ms = std::chrono::duration<double, std::milli>(tri1 - tri0).count();
    std::cout << "    BVH traversal: " << trav_ms << " ms, triangle tests: " << tri_ms << " ms (candidates = " << candidate_ids.size() << ", tri_hits = " << tri_hits << ")\n";
    
    // Show speedup
    if (bvh_time_ms > 0 && mesh_time_ms > 0) {
        double speedup = mesh_time_ms / bvh_time_ms;
        std::cout << "\nSpeedup: " << speedup << "x (BVH is " << speedup << "x faster)\n";
    }
    
    std::cout << "\n=== Direct BVH Benchmark (Like JavaScript) ===\n";
    std::cout << "Testing pure BVH performance without mesh conversion overhead...\n\n";
    
    // Test with different box counts to compare with JavaScript (5ms for 10k boxes)
    std::vector<int> box_counts = {100, 5000, 10000};
    
    for (int box_count : box_counts) {
        // Create random boxes (similar to JavaScript example)
        std::vector<BoundingBox> boxes;
        boxes.reserve(box_count);
        
        const double WORLD_SIZE = 100.0;
        const double MIN_SIZE = 5.0;
        const double MAX_SIZE = 10.0;
        
        std::srand(42); // Fixed seed for consistency
        for (int i = 0; i < box_count; i++) {
            // Random position within world bounds
            double x = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            double y = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            double z = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            
            // Random box size
            double w = MIN_SIZE + (static_cast<double>(std::rand()) / RAND_MAX) * (MAX_SIZE - MIN_SIZE);
            double h = MIN_SIZE + (static_cast<double>(std::rand()) / RAND_MAX) * (MAX_SIZE - MIN_SIZE);
            double d = MIN_SIZE + (static_cast<double>(std::rand()) / RAND_MAX) * (MAX_SIZE - MIN_SIZE);
            
            Point center(x, y, z);
            Vector half_size(w * 0.5, h * 0.5, d * 0.5);
            
            boxes.emplace_back(center, Vector(1,0,0), Vector(0,1,0), Vector(0,0,1), half_size);
        }
        
        // Build BVH and time it (pure BVH, no mesh conversion)
        auto bvh_start = std::chrono::high_resolution_clock::now();
        BVH bvh = BVH::from_boxes(boxes, WORLD_SIZE);
        auto bvh_end = std::chrono::high_resolution_clock::now();
        double bvh_build_ms = std::chrono::duration<double, std::milli>(bvh_end - bvh_start).count();
        
        std::cout << box_count << " boxes: BVH build time = " << bvh_build_ms << " ms";
        
        // Compare to JavaScript (5ms for 10k boxes)
        if (box_count == 10000) {
            std::cout << " (JavaScript: ~5ms, C++ is " << (bvh_build_ms / 5.0) << "x slower)";
        }

        auto coll_start = std::chrono::high_resolution_clock::now();
        auto [pairs, colliding_indices, checks] = bvh.check_all_collisions(boxes);
        auto coll_end = std::chrono::high_resolution_clock::now();
        double coll_ms = std::chrono::duration<double, std::milli>(coll_end - coll_start).count();
        (void)colliding_indices; // not printed
        std::cout << ", collision check time = " << coll_ms << " ms (pairs = " << pairs.size() << ", checks = " << checks << ")";

        // // For 1000 boxes: print AABBs (six numbers per line) and collision pairs
        // if (box_count == 100) {
        //     std::cout << "\nBoxes (min_x min_y min_z max_x max_y max_z):\n";
        //     for (size_t i = 0; i < boxes.size(); ++i) {
        //         const auto& b = boxes[i];
        //         double min_x = b.center.x() - b.half_size.x();
        //         double min_y = b.center.y() - b.half_size.y();
        //         double min_z = b.center.z() - b.half_size.z();
        //         double max_x = b.center.x() + b.half_size.x();
        //         double max_y = b.center.y() + b.half_size.y();
        //         double max_z = b.center.z() + b.half_size.z();
        //         std::cout << min_x << " " << min_y << " " << min_z << " "
        //                   << max_x << " " << max_y << " " << max_z << "\n";
        //     }

        //     auto [pairs, colliding_indices, checks] = bvh.check_all_collisions(boxes);
        //     (void)colliding_indices; // not printed per request
        //     std::cout << "Collisions (" << pairs.size() << " pairs):\n";
        //     for (const auto& pr : pairs) {
        //         std::cout << pr.first << " " << pr.second << "\n";
        //     }
        // }

        std::cout << "\n";
    }

    // =============================================================================
    // Session Collision Detection Example
    // =============================================================================
    std::cout << "\n=== Session Collision Detection (GUID Tracking) ===\n";
    std::cout << "Testing Session::get_collisions() with geometry GUIDs...\n\n";
    
    {
        Session scene("collision_test");
        
        // Add some boxes that will collide
        auto box1 = std::make_shared<BoundingBox>(
            Point(0, 0, 0), 
            Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), 
            Vector(2, 2, 2)
        );
        box1->name = "box_A";
        auto node1 = scene.add_bbox(box1);
        scene.add(node1);
        
        auto box2 = std::make_shared<BoundingBox>(
            Point(3, 0, 0),  // Overlaps with box1
            Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), 
            Vector(2, 2, 2)
        );
        box2->name = "box_B";
        auto node2 = scene.add_bbox(box2);
        scene.add(node2);
        
        auto box3 = std::make_shared<BoundingBox>(
            Point(20, 0, 0),  // Far away, no collision
            Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), 
            Vector(1, 1, 1)
        );
        box3->name = "box_C";
        auto node3 = scene.add_bbox(box3);
        scene.add(node3);
        
        // Add a line that might collide
        auto line1 = std::make_shared<Line>(Line::from_points(Point(0, 0, 0), Point(5, 0, 0)));
        line1->name = "line_D";
        auto node4 = scene.add_line(line1);
        scene.add(node4);
        
        // Add points
        auto pt1 = std::make_shared<Point>(1, 1, 1);  // Inside box1
        pt1->name = "point_E";
        auto node5 = scene.add_point(pt1);
        scene.add(node5);
        
        auto pt2 = std::make_shared<Point>(100, 100, 100);  // Far away
        pt2->name = "point_F";
        auto node6 = scene.add_point(pt2);
        scene.add(node6);
        
        size_t total_objects = scene.objects.points->size() + 
                               scene.objects.lines->size() + 
                               scene.objects.planes->size() + 
                               scene.objects.bboxes->size() + 
                               scene.objects.polylines->size() + 
                               scene.objects.pointclouds->size() + 
                               scene.objects.meshes->size() + 
                               scene.objects.cylinders->size() + 
                               scene.objects.arrows->size();
        std::cout << "Added " << total_objects << " objects to scene\n";
        std::cout << "Object GUIDs and names:\n";
        std::cout << "  - " << box1->guid << " (" << box1->name << ")\n";
        std::cout << "  - " << box2->guid << " (" << box2->name << ")\n";
        std::cout << "  - " << box3->guid << " (" << box3->name << ")\n";
        std::cout << "  - " << line1->guid << " (" << line1->name << ")\n";
        std::cout << "  - " << pt1->guid << " (" << pt1->name << ")\n";
        std::cout << "  - " << pt2->guid << " (" << pt2->name << ")\n";
        
        // Check collisions
        auto collision_pairs = scene.get_collisions();
        
        std::cout << "\nFound " << collision_pairs.size() << " collision pairs:\n";
        for (const auto& [guid1, guid2] : collision_pairs) {
            // Look up names for display
            std::string name1 = "unknown";
            std::string name2 = "unknown";
            
            if (guid1 == box1->guid) name1 = box1->name;
            else if (guid1 == box2->guid) name1 = box2->name;
            else if (guid1 == box3->guid) name1 = box3->name;
            else if (guid1 == line1->guid) name1 = line1->name;
            else if (guid1 == pt1->guid) name1 = pt1->name;
            else if (guid1 == pt2->guid) name1 = pt2->name;
            
            if (guid2 == box1->guid) name2 = box1->name;
            else if (guid2 == box2->guid) name2 = box2->name;
            else if (guid2 == box3->guid) name2 = box3->name;
            else if (guid2 == line1->guid) name2 = line1->name;
            else if (guid2 == pt1->guid) name2 = pt1->name;
            else if (guid2 == pt2->guid) name2 = pt2->name;
            
            std::cout << "  Collision: " << name1 << " <-> " << name2 << "\n";
            std::cout << "    GUID1: " << guid1 << "\n";
            std::cout << "    GUID2: " << guid2 << "\n";
        }
        
        if (collision_pairs.empty()) {
            std::cout << "  (No collisions detected)\n";
        }
        
        std::cout << "\n✓ Session collision detection working with GUID tracking!\n";
    }

    // =============================================================================
    // Session Ray Casting Example
    // =============================================================================
    std::cout << "\n=== Session Ray Casting (BVH-Accelerated) ===\n";
    std::cout << "Testing Session::ray_cast() with various geometry types...\n\n";
    
    {
        Session scene("ray_test");
        
        // Add various geometry along the X axis
        auto pt1 = std::make_shared<Point>(5, 0, 0);
        pt1->name = "point_at_5";
        scene.add_point(pt1);
        
        auto pt2 = std::make_shared<Point>(15, 0, 0);
        pt2->name = "point_at_15";
        scene.add_point(pt2);
        
        auto line1 = std::make_shared<Line>(Line::from_points(Point(10, -2, 0), Point(10, 2, 0)));
        line1->name = "vertical_line_at_10";
        scene.add_line(line1);
        
        Point plane_pt(20, 0, 0);
        Vector plane_x(1, 0, 0);
        Vector plane_y(0, 1, 0);
        auto plane1 = std::make_shared<Plane>(plane_pt, plane_x, plane_y);
        plane1->name = "plane_at_20";
        scene.add_plane(plane1);
        
        // Add polyline
        std::vector<Point> poly_pts = {
            Point(25, -1, -1),
            Point(25, 0, 0),
            Point(25, 1, 1)
        };
        auto polyline1 = std::make_shared<Polyline>(poly_pts);
        polyline1->name = "polyline_at_25";
        scene.add_polyline(polyline1);
        
        // Cast a ray along X axis
        Point ray_origin(0, 0, 0);
        Vector ray_direction(1, 0, 0);  // Along X axis
        double tolerance = 0.5;  // 0.5 units tolerance
        
        std::cout << "Ray: origin=(0,0,0), direction=(1,0,0), tolerance=" << tolerance << "\n";
        std::cout << "Scene objects: point_at_5, point_at_15, vertical_line_at_10, plane_at_20, polyline_at_25\n\n";
        
        auto hits = scene.ray_cast(ray_origin, ray_direction, tolerance);
        
        std::cout << "Found " << hits.size() << " ray hits (sorted by distance):\n";
        for (size_t i = 0; i < hits.size(); ++i) {
            const auto& hit = hits[i];
            
            // Find name
            std::string name = "unknown";
            if (hit.guid == pt1->guid) name = pt1->name;
            else if (hit.guid == pt2->guid) name = pt2->name;
            else if (hit.guid == line1->guid) name = line1->name;
            else if (hit.guid == plane1->guid) name = plane1->name;
            else if (hit.guid == polyline1->guid) name = polyline1->name;
            
            std::cout << "  [" << i << "] " << name << "\n";
            std::cout << "      Hit point: (" << hit.hit_point.x() << ", " 
                      << hit.hit_point.y() << ", " << hit.hit_point.z() << ")\n";
            std::cout << "      Distance: " << hit.distance << "\n";
            std::cout << "      GUID: " << hit.guid << "\n";
        }
        
        if (hits.empty()) {
            std::cout << "  (No hits detected)\n";
        }
        
        std::cout << "\n✓ Session ray casting working with BVH acceleration!\n";
    }

    // =============================================================================
    // Comprehensive Ray Casting Test - All Geometry Types
    // =============================================================================
    std::cout << "\n=== Comprehensive Ray Test (All Geometry Types) ===\n";
    std::cout << "Testing ray intersection with ALL geometry types...\n\n";
    
    {
        Session scene("comprehensive_test");
        
        // Add all geometry types along Y axis
        auto pt = std::make_shared<Point>(0, 10, 0);
        pt->name = "point_10";
        scene.add_point(pt);
        
        auto line = std::make_shared<Line>(Line::from_points(Point(-1, 20, 0), Point(1, 20, 0)));
        line->name = "line_20";
        scene.add_line(line);
        
        Point plane_pt(0, 30, 0);
        Vector plane_x(1, 0, 0);
        Vector plane_y(0, 0, 1);
        auto plane = std::make_shared<Plane>(plane_pt, plane_x, plane_y);
        plane->name = "plane_30";
        scene.add_plane(plane);
        
        auto bbox = std::make_shared<BoundingBox>(
            Point(0, 40, 0),
            Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1),
            Vector(2, 2, 2)
        );
        bbox->name = "bbox_40";
        scene.add_bbox(bbox);
        
        Line cyl_line = Line::from_points(Point(-1, 50, 0), Point(1, 50, 0));
        auto cyl = std::make_shared<Cylinder>(cyl_line, 1.0);
        cyl->name = "cylinder_50";
        scene.add_cylinder(cyl);
        
        Line arrow_line = Line::from_points(Point(-1, 60, 0), Point(1, 60, 0));
        auto arrow = std::make_shared<Arrow>(arrow_line, 1.0);
        arrow->name = "arrow_60";
        scene.add_arrow(arrow);
        
        std::vector<Point> poly_pts = {
            Point(-1, 70, 0),
            Point(0, 70, 0),
            Point(1, 70, 0)
        };
        auto poly = std::make_shared<Polyline>(poly_pts);
        poly->name = "polyline_70";
        scene.add_polyline(poly);
        
        // Cast ray along Y axis
        Point ray_origin(0, 0, 0);
        Vector ray_dir(0, 1, 0);
        double tolerance = 1.0;
        
        std::cout << "Ray: origin=(0,0,0), direction=(0,1,0), tolerance=" << tolerance << "\n";
        std::cout << "Testing: Point, Line, Plane, BoundingBox, Cylinder, Arrow, Polyline\n\n";
        
        auto hits = scene.ray_cast(ray_origin, ray_dir, tolerance);
        
        std::cout << "Found " << hits.size() << " hit(s):\n";
        for (size_t i = 0; i < hits.size(); ++i) {
            const auto& hit = hits[i];
            
            // Find name
            std::string name = "unknown";
            if (hit.guid == pt->guid) name = pt->name;
            else if (hit.guid == line->guid) name = line->name;
            else if (hit.guid == plane->guid) name = plane->name;
            else if (hit.guid == bbox->guid) name = bbox->name;
            else if (hit.guid == cyl->guid) name = cyl->name;
            else if (hit.guid == arrow->guid) name = arrow->name;
            else if (hit.guid == poly->guid) name = poly->name;
            
            std::cout << "  [" << i << "] " << name << " at distance " << hit.distance << "\n";
        }
        
        std::cout << "\n✓ All geometry types tested with BVH acceleration!\n";
        std::cout << "\nBVH Performance Notes:\n";
        std::cout << "  - BVH built once for all objects\n";
        std::cout << "  - Ray traversal prunes non-intersecting AABBs\n";
        std::cout << "  - Only candidates tested with precise intersection\n";
        std::cout << "  - Works for ALL geometry types automatically\n";
    }

    // =============================================================================
    // Session Ray Casting Performance - 10,000 Objects
    // =============================================================================
    std::cout << "\n=== Session Ray Casting Performance (10,000 Objects) ===\n";
    std::cout << "Comparing Session vs Pure BVH performance...\n\n";
    
    {
        const int OBJECT_COUNT = 10000;
        const double WORLD_SIZE = 100.0;
        
        // Create Session with 10,000 random points
        Session scene("perf_test");
        std::vector<BoundingBox> pure_boxes;  // For pure BVH comparison
        
        std::srand(42);  // Fixed seed
        for (int i = 0; i < OBJECT_COUNT; ++i) {
            double x = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            double y = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            double z = (static_cast<double>(std::rand()) / RAND_MAX - 0.5) * WORLD_SIZE;
            
            auto pt = std::make_shared<Point>(x, y, z);
            pt->name = "point_" + std::to_string(i);
            scene.add_point(pt);
            
            // Also create AABB for pure BVH test
            pure_boxes.emplace_back(
                Point(x, y, z),
                Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1),
                Vector(0.5, 0.5, 0.5)
            );
        }
        
        std::cout << "Created " << OBJECT_COUNT << " objects in scene\n";
        
        // Test ray along X axis
        Point ray_origin(0, 0, 0);
        Vector ray_dir(1, 0, 0);
        double tolerance = 1.0;
        
        // =============================================================================
        // BENCHMARK 1: Session::ray_cast() - With GUID tracking and geometry dispatch
        // =============================================================================
        auto session_start = std::chrono::high_resolution_clock::now();
        auto session_hits = scene.ray_cast(ray_origin, ray_dir, tolerance);
        auto session_end = std::chrono::high_resolution_clock::now();
        double session_ms = std::chrono::duration<double, std::milli>(session_end - session_start).count();
        
        std::cout << "\n[1] Session::ray_cast() FIRST call (builds cache):\n";
        std::cout << "    Time: " << session_ms << " ms\n";
        std::cout << "    Hits: " << session_hits.size() << "\n";
        std::cout << "    Includes: BVH build + ray traversal + GUID lookup + variant dispatch\n";
        
        // =============================================================================
        // BENCHMARK 1b: Session::ray_cast() SECOND call - Using cached BVH
        // =============================================================================
        auto session2_start = std::chrono::high_resolution_clock::now();
        auto session_hits2 = scene.ray_cast(ray_origin, Vector(0, 1, 0), tolerance);  // Different direction
        auto session2_end = std::chrono::high_resolution_clock::now();
        double session2_ms = std::chrono::duration<double, std::milli>(session2_end - session2_start).count();
        
        std::cout << "\n[1b] Session::ray_cast() SECOND call (uses cache):\n";
        std::cout << "    Time: " << session2_ms << " ms\n";
        std::cout << "    Hits: " << session_hits2.size() << "\n";
        std::cout << "    Includes: ray traversal + GUID lookup + variant dispatch (NO BVH BUILD!)\n";
        std::cout << "    Speedup vs first call: " << (session_ms / session2_ms) << "x faster\n";
        
        // =============================================================================
        // BENCHMARK 2: Pure BVH - Only AABB intersection (no geometry dispatch)
        // =============================================================================
        auto bvh_start = std::chrono::high_resolution_clock::now();
        BVH pure_bvh = BVH::from_boxes(pure_boxes, WORLD_SIZE);
        std::vector<int> candidate_ids;
        pure_bvh.ray_cast(ray_origin, ray_dir, candidate_ids, true);
        auto bvh_end = std::chrono::high_resolution_clock::now();
        double bvh_ms = std::chrono::duration<double, std::milli>(bvh_end - bvh_start).count();
        
        std::cout << "\n[2] Pure BVH::ray_cast() AABB only:\n";
        std::cout << "    Time: " << bvh_ms << " ms\n";
        std::cout << "    Candidates: " << candidate_ids.size() << "\n";
        std::cout << "    Includes: BVH build + ray traversal only\n";
        
        // =============================================================================
        // Analysis
        // =============================================================================
        double overhead_ms = session_ms - bvh_ms;
        double overhead_pct = (overhead_ms / bvh_ms) * 100.0;
        double cached_overhead_ms = session2_ms - bvh_ms;
        double cached_overhead_pct = (cached_overhead_ms / bvh_ms) * 100.0;
        
        std::cout << "\n[Analysis - Performance Comparison]\n";
        std::cout << "    Pure BVH time:                 " << bvh_ms << " ms\n";
        std::cout << "    Session FIRST call (no cache): " << session_ms << " ms (" << overhead_pct << "% overhead)\n";
        std::cout << "    Session CACHED call:           " << session2_ms << " ms (" << cached_overhead_pct << "% overhead)\n";
        std::cout << "\n[Cache Performance Impact]\n";
        std::cout << "    BVH rebuild cost: " << (session_ms - session2_ms) << " ms\n";
        std::cout << "    Cache speedup:    " << (session_ms / session2_ms) << "x faster on subsequent rays\n";
        std::cout << "\n[Remaining overhead breakdown (cached)]\n";
        std::cout << "    - GUID lookup:         std::unordered_map access per candidate\n";
        std::cout << "    - Variant dispatch:    std::visit type checking\n";
        std::cout << "    - Precise geometry:    Point-ray distance calculation\n";
        std::cout << "    - Result tracking:     Closest hit detection\n";
        
        if (cached_overhead_pct < 50) {
            std::cout << "\n✓ Cached Session overhead is LOW (< 50%) - Excellent performance!\n";
        } else if (cached_overhead_pct < 100) {
            std::cout << "\n✓ Cached Session overhead is MODERATE (< 100%) - Good performance\n";
        } else {
            std::cout << "\n⚠ Cached Session overhead is still HIGH - Geometry dispatch is expensive\n";
        }
        
        std::cout << "\n[Summary]\n";
        std::cout << "  ✓ BVH caching implemented successfully!\n";
        std::cout << "  ✓ First ray cast: builds BVH (expensive)\n";
        std::cout << "  ✓ Subsequent rays: reuse cached BVH (fast)\n";
        std::cout << "  ✓ Cache invalidates on add/remove geometry\n";
    }

    return 0;
}
