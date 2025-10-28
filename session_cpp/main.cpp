#include "src/intersection.h"
#include "src/session.h"
#include "src/bvh.h"
#include <iostream>
#include <chrono>
#include "src/plane.h"
#include "src/mesh.h"
#include "src/boundingbox.h"
#include "src/obj.h"
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
    std::cout << "=== Intersection Examples ===\n\n";

    // 1. line_line
    Point p;
    if (Intersection::line_line(l0, l1, p, Tolerance::APPROXIMATION)) {
        std::cout << "1. line_line: " << p.x() << ", " << p.y() << ", " << p.z() << "\n";
    }

    // 2. line_line_parameters
    double t0, t1;
    if (Intersection::line_line_parameters(l0, l1, t0, t1, Tolerance::APPROXIMATION)) {
        std::cout << "2. line_line_parameters: t0=" << t0 << ", t1=" << t1 << "\n";
    }

    // 3. plane_plane
    Line intersection_line;
    if (Intersection::plane_plane(pl0, pl1, intersection_line)) {
       printf("3. plane_plane: %s\n", intersection_line.to_string().c_str());
    }

    // 4. line_plane
    Point lp;
    if (Intersection::line_plane(l0, pl0, lp)) {
        std::cout << "4. line_plane: " << lp.x() << ", " << lp.y() << ", " << lp.z() << "\n";
    }

    // 5. plane_plane_plane {300.5, 565.5, -0}
    Point ppp;
    if (Intersection::plane_plane_plane(pl0, pl1, pl2, ppp)) {
        std::cout << "5. plane_plane_plane: " << ppp.x() << ", " << ppp.y() << ", " << ppp.z() << "\n";
    }

    // 6. ray_box
    Point min(214, 192, 484);
    Point max(694, 567, 796);
    std::vector<Point> points {min, max};
    BoundingBox box = BoundingBox::from_points(points);
    

    std::vector<Point> intersection_points;
    if (Intersection::ray_box(l0, box, 0.0, 1000.0, intersection_points)) {
        std::cout << "6. ray_box: entry=" << intersection_points[0] 
                  << ", exit=" << intersection_points[1] << "\n";
    }

    // 7. ray_sphere
    Point sphere_center_test(457.0, 192.0, 207.0);
    std::vector<Point> sphere_points;
    if (Intersection::ray_sphere(l0, sphere_center_test, 265.0, sphere_points)) {
        std::cout << "7. ray_sphere: " << sphere_points.size() << " hits";
        for (size_t i = 0; i < sphere_points.size(); i++) {
            std::cout << ", p" << i << "=" << sphere_points[i];
        }
        std::cout << "\n";
    } else {
        std::cout << "7. ray_sphere: 0 hits\n";
    }

    // 8. ray_triangle
    Point tri_p1(214, 567, 484);
    Point tri_p2(214, 192, 796);
    Point tri_p3(694, 192, 484);

    Point triangle_hit;
    if (Intersection::ray_triangle(l0, tri_p1, tri_p2, tri_p3, Tolerance::APPROXIMATION, triangle_hit)) {
        std::cout << "8. ray_triangle: " << triangle_hit << "\n";
    }

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
    
    std::cout << "Bunny: " << bunny.number_of_vertices() << " vertices, " 
              << bunny.number_of_faces() << " faces\n";
    
    auto bvh_build_start = std::chrono::high_resolution_clock::now();
    bunny.build_triangle_bvh();
    auto bvh_build_end = std::chrono::high_resolution_clock::now();
    double bvh_build_time_ms = std::chrono::duration<double, std::milli>(bvh_build_end - bvh_build_start).count();
    std::cout << "BVH build: " << bvh_build_time_ms << " ms\n";
    
    Line zaxis(0.201, -0.212, 0.036, -0.326, 0.677, -0.060);

    // Test brute force (slower)
    auto time2 = std::chrono::high_resolution_clock::now();
    auto mesh_hits = Intersection::ray_mesh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time3 = std::chrono::high_resolution_clock::now();
    double mesh_time_ms = std::chrono::duration<double, std::milli>(time3 - time2).count();
    std::cout << "Ray-mesh (brute): " << mesh_hits.size() << " hits, " << mesh_time_ms << " ms\n";
    
    auto time0 = std::chrono::high_resolution_clock::now();
    auto bvh_hits = Intersection::ray_mesh_bvh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time1 = std::chrono::high_resolution_clock::now();
    double bvh_time_ms = std::chrono::duration<double, std::milli>(time1 - time0).count();
    std::cout << "Ray-mesh (BVH):   " << bvh_hits.size() << " hits, " << bvh_time_ms << " ms";
    
    if (bvh_time_ms > 0 && mesh_time_ms > 0) {
        std::cout << " (" << (mesh_time_ms / bvh_time_ms) << "x faster)\n";
    } else {
        std::cout << "\n";
    }
    
    std::cout << "\n=== BVH Collision Detection ===\n";
    
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
        
        auto coll_start = std::chrono::high_resolution_clock::now();
        auto [pairs, colliding_indices, checks] = bvh.check_all_collisions(boxes);
        auto coll_end = std::chrono::high_resolution_clock::now();
        double coll_ms = std::chrono::duration<double, std::milli>(coll_end - coll_start).count();
        (void)colliding_indices;
        std::cout << box_count << " boxes: build=" << bvh_build_ms << "ms, collisions=" << coll_ms << "ms (" << pairs.size() << " pairs, " << checks << " checks)";

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

    std::cout << "\n\n=== Comprehensive 10k Mixed Geometry Test ===\n";
    
    {
        const int OBJECT_COUNT = 10000;
        const double WORLD_SIZE = 50.0;
        
        Session scene("comprehensive_test");
        std::vector<BoundingBox> aabb_boxes;
        std::vector<BoundingBox> oobb_boxes;
        aabb_boxes.reserve(OBJECT_COUNT);
        oobb_boxes.reserve(OBJECT_COUNT);
        
        std::srand(42);
        
        std::cout << "Creating " << OBJECT_COUNT << " mixed geometry objects..." << std::endl;
        
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
                
                Point poly_origin;
                Vector poly_x, poly_y, poly_z;
                poly->get_average_plane(poly_origin, poly_x, poly_y, poly_z);
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
                
                Point mesh_pt(x, y, z);
                Vector mesh_x(1, 0, 0);
                Vector mesh_y(0, 1, 0);
                Plane mesh_plane(mesh_pt, mesh_x, mesh_y);
                aabb_boxes.push_back(BoundingBox::from_mesh(*mesh, 0.1));
                oobb_boxes.push_back(BoundingBox::from_mesh(*mesh, mesh_plane, 0.1));
            } else if (geom_type == 5) {
                Line cyl_line = Line::from_points(Point(x - 1, y, z), Point(x + 1, y, z));
                auto cyl = std::make_shared<Cylinder>(cyl_line, 0.3);
                scene.add_cylinder(cyl);
                
                Point cyl_pt(x, y, z);
                Vector cyl_x(1, 0, 0);
                Vector cyl_y(0, 1, 0);
                Plane cyl_plane(cyl_pt, cyl_x, cyl_y);
                aabb_boxes.push_back(BoundingBox::from_cylinder(*cyl, 0.1));
                oobb_boxes.push_back(BoundingBox::from_cylinder(*cyl, cyl_plane, 0.1));
            } else {
                Line arrow_line = Line::from_points(Point(x - 1, y, z), Point(x + 1, y, z));
                auto arrow = std::make_shared<Arrow>(arrow_line, 0.3);
                scene.add_arrow(arrow);
                
                Point arrow_pt(x, y, z);
                Vector arrow_x(1, 0, 0);
                Vector arrow_y(0, 1, 0);
                Plane arrow_plane(arrow_pt, arrow_x, arrow_y);
                aabb_boxes.push_back(BoundingBox::from_arrow(*arrow, 0.1));
                oobb_boxes.push_back(BoundingBox::from_arrow(*arrow, arrow_plane, 0.1));
            }
        }
        
        std::cout << "\n(a) AABB BVH Collision Detection:\n";
        auto aabb_start = std::chrono::high_resolution_clock::now();
        BVH aabb_bvh = BVH::from_boxes(aabb_boxes, WORLD_SIZE);
        auto [aabb_collisions, aabb_indices, aabb_checks] = aabb_bvh.check_all_collisions(aabb_boxes);
        auto aabb_end = std::chrono::high_resolution_clock::now();
        double aabb_ms = std::chrono::duration<double, std::milli>(aabb_end - aabb_start).count();
        (void)aabb_indices;
        (void)aabb_checks;
        
        std::cout << "  Build + query: " << aabb_ms << "ms\n";
        std::cout << "  Collision pairs: " << aabb_collisions.size() << "\n";
        
        std::cout << "\n(b) Ray BVH Intersection:\n";
        Point ray_origin(0, 0, 0);
        Vector ray_dir(1, 0, 0);
        
        auto ray_start = std::chrono::high_resolution_clock::now();
        std::vector<int> ray_candidates;
        aabb_bvh.ray_cast(ray_origin, ray_dir, ray_candidates, true);
        auto ray_end = std::chrono::high_resolution_clock::now();
        double ray_ms = std::chrono::duration<double, std::milli>(ray_end - ray_start).count();
        
        std::cout << "  Query: " << ray_ms << "ms\n";
        std::cout << "  Candidates: " << ray_candidates.size() << "\n";
        
        std::cout << "\n(c) OOBB BVH Collision Detection (Optimized):\n";
        auto oobb_start = std::chrono::high_resolution_clock::now();
        
        BVH oobb_bvh = BVH::from_boxes(oobb_boxes, WORLD_SIZE);
        auto [oobb_candidates, oobb_indices, oobb_checks] = oobb_bvh.check_all_collisions(oobb_boxes);
        (void)oobb_indices;
        (void)oobb_checks;
        
        std::cout << "  BVH broad-phase: " << oobb_candidates.size() << " candidates\n";
        
        auto sat_start = std::chrono::high_resolution_clock::now();
        double bvh_ms = std::chrono::duration<double, std::milli>(sat_start - oobb_start).count();
        std::cout << "  BVH build + query: " << bvh_ms << "ms\n";
        
        std::cout << "  Running SAT...\n";
        int true_oobb_collisions = 0;
        size_t report_interval = std::max<size_t>(1, oobb_candidates.size() / 10);  // every 10%
        for (size_t idx = 0; idx < oobb_candidates.size(); ++idx) {
            const auto& [i, j] = oobb_candidates[idx];
            if (oobb_boxes[i].collides_with(oobb_boxes[j])) {
                true_oobb_collisions++;
            }
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
        std::cout << "  Final precision: " << (100.0 * true_oobb_collisions / std::max(1, static_cast<int>(oobb_candidates.size()))) << "%\n";
        
        std::cout << "\nComparison:\n";
        std::cout << "  AABB collisions: " << aabb_collisions.size() << "\n";
        std::cout << "  OOBB collisions: " << true_oobb_collisions << "\n";
        std::cout << "  Tightness improvement: " << (100.0 * (1.0 - static_cast<double>(true_oobb_collisions) / std::max(1, static_cast<int>(aabb_collisions.size())))) << "%\n";
        std::cout << "\nOptimization impact:\n";
        std::cout << "  Using optimized RTCD SAT; no redundant AABB recheck\n";
    }


    std::cout << "\n=== Session Ray Casting ===\n";
    
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
        
        Point ray_origin(0, 0, 0);
        Vector ray_direction(1, 0, 0);
        double tolerance = 0.5;
        
        auto hits = scene.ray_cast(ray_origin, ray_direction, tolerance);
        
        std::cout << hits.size() << " hit(s):\n";
        for (size_t i = 0; i < hits.size(); ++i) {
            const auto& hit = hits[i];
            
            // Find name
            std::string name = "unknown";
            if (hit.guid == pt1->guid) name = pt1->name;
            else if (hit.guid == pt2->guid) name = pt2->name;
            else if (hit.guid == line1->guid) name = line1->name;
            else if (hit.guid == plane1->guid) name = plane1->name;
            else if (hit.guid == polyline1->guid) name = polyline1->name;
            
            std::cout << "  " << name << " (dist=" << hit.distance << ")\n";
        }
    }

    std::cout << "\n=== All Geometry Types Test ===\n";
    
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
        
        Point ray_origin(0, 0, 0);
        Vector ray_dir(0, 1, 0);
        double tolerance = 1.0;
        
        auto hits = scene.ray_cast(ray_origin, ray_dir, tolerance);
        
        std::cout << hits.size() << " hit(s):\n";
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
            
            std::cout << "  " << name << " (dist=" << hit.distance << ")\n";
        }
    }

    std::cout << "\n=== Performance Test (10k Objects) ===\n";
    
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
        
        Point ray_origin(0, 0, 0);
        Vector ray_dir(1, 0, 0);
        double tolerance = 1.0;
        
        auto session_start = std::chrono::high_resolution_clock::now();
        auto session_hits = scene.ray_cast(ray_origin, ray_dir, tolerance);
        auto session_end = std::chrono::high_resolution_clock::now();
        double session_ms = std::chrono::duration<double, std::milli>(session_end - session_start).count();
        
        auto session2_start = std::chrono::high_resolution_clock::now();
        auto session_hits2 = scene.ray_cast(ray_origin, Vector(0, 1, 0), tolerance);
        auto session2_end = std::chrono::high_resolution_clock::now();
        double session2_ms = std::chrono::duration<double, std::milli>(session2_end - session2_start).count();
        
        auto bvh_start = std::chrono::high_resolution_clock::now();
        BVH pure_bvh = BVH::from_boxes(pure_boxes, WORLD_SIZE);
        std::vector<int> candidate_ids;
        pure_bvh.ray_cast(ray_origin, ray_dir, candidate_ids, true);
        auto bvh_end = std::chrono::high_resolution_clock::now();
        double bvh_ms = std::chrono::duration<double, std::milli>(bvh_end - bvh_start).count();
        
        std::cout << "Session (first):  " << session_ms << "ms (" << session_hits.size() << " hits)\n";
        std::cout << "Session (cached): " << session2_ms << "ms (" << session_hits2.size() << " hits, " 
                  << (session_ms / session2_ms) << "x faster)\n";
        std::cout << "Pure BVH:         " << bvh_ms << "ms (" << candidate_ids.size() << " candidates)\n";
    }

    // Tree transformation example moved to session_test.cpp
    // See TEST_CASE("Session tree transformation hierarchy.")

    return 0;
}
