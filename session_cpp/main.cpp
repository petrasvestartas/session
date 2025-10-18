#include "src/intersection.h"
#include "src/point.h"
#include "src/vector.h"
#include "src/line.h"
#include "src/plane.h"
#include "src/mesh.h"
#include "src/boundingbox.h"
#include "src/obj.h"
#include <iostream>
#include <vector>
#include <chrono>

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
    float t0, t1;
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
    if (Intersection::ray_box(l0, box, 0.0f, 1000.0f, intersection_points)) {
        std::cout << "6. ray_box: entry=" << intersection_points[0] 
                  << ", exit=" << intersection_points[1] << "\n";
    }

    // 7. ray_sphere
    Point sphere_center_test(457.0, 192.0, 207.0);
    std::vector<Point> sphere_points;
    if (Intersection::ray_sphere(l0, sphere_center_test, 265.0f, sphere_points)) {
        std::cout << "7. ray_sphere: " << sphere_points.size() << " hits";
        for (size_t i = 0; i < sphere_points.size(); i++) {
            std::cout << ", p" << i << "=" << sphere_points[i];
        }
        std::cout << "\n";
    } else {
        std::cout << "7. ray_sphere: 0 hits\n";
    }

    // 8. ray_triangle
    Point p1(214, 567, 484);
    Point p2(214, 192, 796);
    Point p3(694, 192, 484);

    Point triangle_hit;
    if (Intersection::ray_triangle(l0, p1, p2, p3, Tolerance::APPROXIMATION, triangle_hit)) {
        std::cout << "8. ray_triangle: " << triangle_hit << "\n";
    }

    // 9. ray_mesh - Load bunny mesh
    Mesh bunny = obj::read_obj("../../data/bunny.obj");
    std::cout << "Loaded bunny mesh: " << bunny.number_of_vertices() << " vertices, " 
              << bunny.number_of_faces() << " faces\n";
    
    Line zaxis(0.201, -0.212, 0.036, -0.326, 0.677, -0.060);

    // Test brute force (slower)
    auto time2 = std::chrono::high_resolution_clock::now();
    auto mesh_hits = Intersection::ray_mesh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time3 = std::chrono::high_resolution_clock::now();
    long long mesh_time = std::chrono::duration_cast<std::chrono::microseconds>(time3 - time2).count();
    std::cout << "9. ray_mesh: " << mesh_hits.size() << " hits in " << mesh_time << " μs\n";
    for (size_t i = 0; i < mesh_hits.size(); i++) {
        std::cout << "    [" << i << "] " << mesh_hits[i] << "\n";
    }
    
    
    // Test BVH (faster)
    auto time0 = std::chrono::high_resolution_clock::now();
    auto bvh_hits = Intersection::ray_mesh_bvh(zaxis, bunny, Tolerance::APPROXIMATION, true);
    auto time1 = std::chrono::high_resolution_clock::now();
    long long bvh_time = std::chrono::duration_cast<std::chrono::microseconds>(time1 - time0).count();
    std::cout << "10. ray_mesh_bvh: " << bvh_hits.size() << " hits in " << bvh_time << " μs\n";
    for (size_t i = 0; i < bvh_hits.size(); i++) {
        std::cout << "    [" << i << "] " << bvh_hits[i] << "\n";
    }
    

    if (!bvh_hits.empty() && !mesh_hits.empty()) {
        float speedup = (float)mesh_time / bvh_time;
        std::cout << "Speedup: " << speedup << "x faster with BVH\n";
    }

    return 0;
}
