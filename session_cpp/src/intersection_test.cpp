#include "intersection.h"
#include "catch_amalgamated.hpp"
#include <cmath>

using namespace session_cpp;

TEST_CASE("Line-Line Intersection", "[intersection]") {
    Line line0(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    Line line1(0.5, -1.0, 0.0, 0.5, 1.0, 0.0);
    
    Point output;
    bool result = Intersection::line_line(line0, line1, output, Tolerance::APPROXIMATION);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 0.5) < 1e-5);
    REQUIRE(std::fabs(output.y() - 0.0) < 1e-5);
    REQUIRE(std::fabs(output.z() - 0.0) < 1e-5);
}

TEST_CASE("Line-Line Parallel", "[intersection]") {
    Line line0(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    Line line1(0.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    
    Point output;
    bool result = Intersection::line_line(line0, line1, output, Tolerance::APPROXIMATION);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Line-Line Parameters", "[intersection]") {
    Line line0(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    Line line1(0.5, -1.0, 0.0, 0.5, 1.0, 0.0);
    
    double t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1, Tolerance::APPROXIMATION);
    
    REQUIRE(result);
    REQUIRE(std::fabs(t0 - 0.5) < 1e-5);
    REQUIRE(std::fabs(t1 - 0.5) < 1e-5);
}

TEST_CASE("Line-Line Parameters Exact Endpoints", "[intersection]") {
    Line line0(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    Line line1(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    
    double t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1, Tolerance::APPROXIMATION);
    
    REQUIRE(result);
    REQUIRE(t0 == 0.0);
    REQUIRE(t1 == 0.0);
}

TEST_CASE("Line-Line Parameters Infinite Lines", "[intersection]") {
    Line line0(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    Line line1(2.0, -1.0, 0.0, 2.0, 1.0, 0.0);
    
    double t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1, static_cast<double>(Tolerance::APPROXIMATION), false);
    
    REQUIRE(result);
    REQUIRE(std::fabs(t0 - 2.0) < 1e-5);
}

TEST_CASE("Plane-Plane Intersection", "[intersection]") {
    Point p0(0.0, 0.0, 0.0);
    Vector n0(0.0, 0.0, 1.0);
    Plane plane0 = Plane::from_point_normal(p0, n0);
    
    Point p1(0.0, 0.0, 0.0);
    Vector n1(0.0, 1.0, 0.0);
    Plane plane1 = Plane::from_point_normal(p1, n1);
    
    Line output;
    bool result = Intersection::plane_plane(plane0, plane1, output);
    
    REQUIRE(result);
    
    Vector line_dir = output.to_vector();
    REQUIRE(std::fabs(std::fabs(line_dir.x()) - 1.0) < 1e-4);
    REQUIRE(std::fabs(line_dir.y()) < 1e-4);
    REQUIRE(std::fabs(line_dir.z()) < 1e-4);
}

TEST_CASE("Plane-Plane Intersection Complex", "[intersection]") {
    Point plane_origin_0(213.787107, 513.797811, -24.743845);
    Vector plane_xaxis_0(0.907673, -0.258819, 0.330366);
    Vector plane_yaxis_0(0.272094, 0.96225, 0.006285);
    Plane pl0(plane_origin_0, plane_xaxis_0, plane_yaxis_0);
    
    Point plane_origin_1(247.17924, 499.115486, 59.619568);
    Vector plane_xaxis_1(0.552465, 0.816035, 0.16991);
    Vector plane_yaxis_1(0.172987, 0.087156, -0.98106);
    Plane pl1(plane_origin_1, plane_xaxis_1, plane_yaxis_1);
    
    Line intersection_line;
    bool result = Intersection::plane_plane(pl0, pl1, intersection_line);
    
    REQUIRE(result);
    
    Point start = intersection_line.start();
    Point end = intersection_line.end();
    
    REQUIRE(std::fabs(start.x() - 252.4632) < 0.01);
    REQUIRE(std::fabs(start.y() - 495.32248) < 0.01);
    REQUIRE(std::fabs(start.z() - (-10.002656)) < 0.01);
    
    REQUIRE(std::fabs(end.x() - 253.01033) < 0.01);
    REQUIRE(std::fabs(end.y() - 496.1218) < 0.01);
    REQUIRE(std::fabs(end.z() - (-9.888727)) < 0.01);
}

TEST_CASE("Line-Plane Intersection", "[intersection]") {
    Point p(0.0, 0.0, 1.0);
    Vector n(0.0, 0.0, 1.0);
    Plane plane = Plane::from_point_normal(p, n);
    
    Line line(0.0, 0.0, 0.0, 0.0, 0.0, 2.0);
    
    Point output;
    bool result = Intersection::line_plane(line, plane, output, true);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 0.0) < 1e-5);
    REQUIRE(std::fabs(output.y() - 0.0) < 1e-5);
    REQUIRE(std::fabs(output.z() - 1.0) < 1e-5);
}

TEST_CASE("Line-Plane Parallel", "[intersection]") {
    Point p(0.0, 0.0, 1.0);
    Vector n(0.0, 0.0, 1.0);
    Plane plane = Plane::from_point_normal(p, n);
    
    Line line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    
    Point output;
    bool result = Intersection::line_plane(line, plane, output, true);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Line-Plane Real-World Intersection", "[intersection]") {
    Line l0(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
    
    Point plane_origin_0(213.787107, 513.797811, -24.743845);
    Vector plane_xaxis_0(0.907673, -0.258819, 0.330366);
    Vector plane_yaxis_0(0.272094, 0.96225, 0.006285);
    Plane pl0(plane_origin_0, plane_xaxis_0, plane_yaxis_0);
    
    Point lp;
    bool result = Intersection::line_plane(l0, pl0, lp);
    
    REQUIRE(result);
    REQUIRE(std::fabs(lp.x() - 500.0) < 0.1);
    REQUIRE(std::fabs(lp.y() - 77.7531) < 0.01);
    REQUIRE(std::fabs(lp.z() - 111.043) < 0.01);
}

TEST_CASE("Plane-Plane-Plane Intersection", "[intersection]") {
    Point plane_origin_0(213.787107, 513.797811, -24.743845);
    Vector plane_xaxis_0(0.907673, -0.258819, 0.330366);
    Vector plane_yaxis_0(0.272094, 0.96225, 0.006285);
    Plane pl0(plane_origin_0, plane_xaxis_0, plane_yaxis_0);
    
    Point plane_origin_1(247.17924, 499.115486, 59.619568);
    Vector plane_xaxis_1(0.552465, 0.816035, 0.16991);
    Vector plane_yaxis_1(0.172987, 0.087156, -0.98106);
    Plane pl1(plane_origin_1, plane_xaxis_1, plane_yaxis_1);
    
    Point plane_origin_2(221.399816, 605.893667, -54.000116);
    Vector plane_xaxis_2(0.903451, -0.360516, -0.231957);
    Vector plane_yaxis_2(0.172742, -0.189057, 0.966653);
    Plane pl2(plane_origin_2, plane_xaxis_2, plane_yaxis_2);
    
    Point output;
    bool result = Intersection::plane_plane_plane(pl0, pl1, pl2, output);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 300.5) < 0.1);
    REQUIRE(std::fabs(output.y() - 565.5) < 0.1);
    REQUIRE(std::fabs(output.z() - 0.0) < 0.1);
}

TEST_CASE("Plane-Plane-Plane Parallel", "[intersection]") {
    Point p0(0.0, 0.0, 0.0);
    Vector n0(0.0, 0.0, 1.0);
    Plane plane0 = Plane::from_point_normal(p0, n0);
    
    Point p1(0.0, 0.0, 1.0);
    Vector n1(0.0, 0.0, 1.0);
    Plane plane1 = Plane::from_point_normal(p1, n1);
    
    Point p2(0.0, 0.0, 0.0);
    Vector n2(1.0, 0.0, 0.0);
    Plane plane2 = Plane::from_point_normal(p2, n2);
    
    Point output;
    bool result = Intersection::plane_plane_plane(plane0, plane1, plane2, output);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Box Intersection", "[intersection]") {
    Point center(0.0, 0.0, 0.0);
    Vector x_axis(1.0, 0.0, 0.0);
    Vector y_axis(0.0, 1.0, 0.0);
    Vector z_axis(0.0, 0.0, 1.0);
    Vector half_size(1.0, 1.0, 1.0);
    BoundingBox box(center, x_axis, y_axis, z_axis, half_size);
    
    Point origin(-5.0, 0.0, 0.0);
    Vector direction(1.0, 0.0, 0.0);
    
    double tmin, tmax;
    bool result = Intersection::ray_box(origin, direction, box, 0.0, 100.0, tmin, tmax);
    
    REQUIRE(result);
    REQUIRE(std::fabs(tmin - 4.0) < 1e-4);
    REQUIRE(std::fabs(tmax - 6.0) < 1e-4);
}

TEST_CASE("Ray-Box Miss", "[intersection]") {
    Point center(0.0, 0.0, 0.0);
    Vector x_axis(1.0, 0.0, 0.0);
    Vector y_axis(0.0, 1.0, 0.0);
    Vector z_axis(0.0, 0.0, 1.0);
    Vector half_size(1.0, 1.0, 1.0);
    BoundingBox box(center, x_axis, y_axis, z_axis, half_size);
    
    Point origin(-5.0, 5.0, 0.0);
    Vector direction(1.0, 0.0, 0.0);
    
    double tmin, tmax;
    bool result = Intersection::ray_box(origin, direction, box, 0.0, 100.0, tmin, tmax);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Sphere Intersection", "[intersection]") {
    Point origin(-5.0, 0.0, 0.0);
    Vector direction(1.0, 0.0, 0.0);
    Point center(0.0, 0.0, 0.0);
    double radius = 2.0;
    
    double t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 2);
    REQUIRE(std::fabs(t0 - 3.0) < 1e-4);
    REQUIRE(std::fabs(t1 - 7.0) < 1e-4);
}

TEST_CASE("Ray-Sphere Tangent", "[intersection]") {
    Point origin(-5.0, 2.0, 0.0);
    Vector direction(1.0, 0.0, 0.0);
    Point center(0.0, 0.0, 0.0);
    double radius = 2.0;
    
    double t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 1);
    REQUIRE(std::fabs(t0 - 5.0) < 1e-4);
}

TEST_CASE("Ray-Sphere Miss", "[intersection]") {
    Point origin(-5.0, 5.0, 0.0);
    Vector direction(1.0, 0.0, 0.0);
    Point center(0.0, 0.0, 0.0);
    double radius = 2.0;
    
    double t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 0);
}

TEST_CASE("Ray-Triangle Intersection", "[intersection]") {
    Point origin(0.5, 0.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    Point v0(0.0, 0.0, 0.0);
    Point v1(1.0, 0.0, 0.0);
    Point v2(0.0, 1.0, 0.0);
    
    double t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6, t, u, v, parallel);
    
    REQUIRE(result);
    REQUIRE_FALSE(parallel);
    REQUIRE(std::fabs(t - 1.0) < 1e-4);
}

TEST_CASE("Ray-Triangle Miss", "[intersection]") {
    Point origin(2.0, 2.0, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    Point v0(0.0, 0.0, 0.0);
    Point v1(1.0, 0.0, 0.0);
    Point v2(0.0, 1.0, 0.0);
    
    double t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6, t, u, v, parallel);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Triangle Parallel", "[intersection]") {
    Point origin(0.5, 0.5, -1.0);
    Vector direction(1.0, 0.0, 0.0);
    
    Point v0(0.0, 0.0, 0.0);
    Point v1(1.0, 0.0, 0.0);
    Point v2(0.0, 1.0, 0.0);
    
    double t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6, t, u, v, parallel);
    
    REQUIRE_FALSE(result);
    REQUIRE(parallel);
}

TEST_CASE("Ray-Mesh Intersection", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)},
        {Point(0.0, 0.0, 1.0), Point(1.0, 0.0, 1.0), Point(1.0, 1.0, 1.0), Point(0.0, 1.0, 1.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5, 0.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, true);
    
    REQUIRE(result);
    REQUIRE(hits.size() >= 1);
    REQUIRE(std::fabs(hits[0].t - 1.0) < 1e-3);
}

TEST_CASE("Ray-Mesh Find First", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)},
        {Point(0.0, 0.0, 1.0), Point(1.0, 0.0, 1.0), Point(1.0, 1.0, 1.0), Point(0.0, 1.0, 1.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5, 0.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, false);
    
    REQUIRE(result);
    REQUIRE(hits.size() == 1);
}

TEST_CASE("Ray-Mesh Miss", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.0, 5.0, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, true);
    
    REQUIRE_FALSE(result);
    REQUIRE(hits.size() == 0);
}

TEST_CASE("Ray-Mesh BVH Intersection", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)},
        {Point(0.0, 0.0, 1.0), Point(1.0, 0.0, 1.0), Point(1.0, 1.0, 1.0), Point(0.0, 1.0, 1.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5, 0.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh_bvh(origin, direction, mesh, hits, true);
    
    REQUIRE(result);
    REQUIRE(hits.size() >= 1);
    REQUIRE(std::fabs(hits[0].t - 1.0) < 1e-3);
}

TEST_CASE("Ray-Mesh BVH Find First", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)},
        {Point(0.0, 0.0, 1.0), Point(1.0, 0.0, 1.0), Point(1.0, 1.0, 1.0), Point(0.0, 1.0, 1.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5, 0.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh_bvh(origin, direction, mesh, hits, false);
    
    REQUIRE(result);
    REQUIRE(hits.size() == 1);
}

TEST_CASE("Ray-Mesh BVH Miss", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.0, 5.0, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh_bvh(origin, direction, mesh, hits, true);
    
    REQUIRE_FALSE(result);
    REQUIRE(hits.size() == 0);
}

TEST_CASE("Ray-Mesh BVH vs Naive Comparison", "[intersection][bvh]") {
    // Create a more complex mesh
    std::vector<std::vector<Point>> polygons;
    for (int i = 0; i < 10; ++i) {
        for (int j = 0; j < 10; ++j) {
            double x = static_cast<double>(i);
            double y = static_cast<double>(j);
            polygons.push_back({
                Point(x, y, 0.0),
                Point(x + 1.0, y, 0.0),
                Point(x + 1.0, y + 1.0, 0.0),
                Point(x, y + 1.0, 0.0)
            });
        }
    }
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.5, 5.5, -1.0);
    Vector direction(0.0, 0.0, 1.0);
    
    // Test naive version
    std::vector<Intersection::RayHit> hits_naive;
    bool result_naive = Intersection::ray_mesh(origin, direction, mesh, hits_naive, true);
    
    // Test BVH version
    std::vector<Intersection::RayHit> hits_bvh;
    bool result_bvh = Intersection::ray_mesh_bvh(origin, direction, mesh, hits_bvh, true);
    
    // Both should return same result
    REQUIRE(result_naive == result_bvh);
    REQUIRE(hits_naive.size() == hits_bvh.size());
    
    if (!hits_naive.empty()) {
        REQUIRE(std::fabs(hits_naive[0].t - hits_bvh[0].t) < 1e-4);
        REQUIRE(hits_naive[0].face_index == hits_bvh[0].face_index);
    }
}

TEST_CASE("Ray-Box Real-World Intersection", "[intersection]") {
    Line l0(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
    Point min(214.0, 192.0, 484.0);
    Point max(694.0, 567.0, 796.0);
    std::vector<Point> points {min, max};
    BoundingBox box = BoundingBox::from_points(points);
    
    std::vector<Point> intersection_points;
    bool result = Intersection::ray_box(l0, box, 0.0, 1000.0, intersection_points);
    
    REQUIRE(result);
    REQUIRE(intersection_points.size() == 2);
    
    // Entry point
    REQUIRE(std::fabs(intersection_points[0].x() - 500.0) < 0.1);
    REQUIRE(std::fabs(intersection_points[0].y() - 338.9) < 0.1);
    REQUIRE(std::fabs(intersection_points[0].z() - 484.0) < 0.1);
    
    // Exit point
    REQUIRE(std::fabs(intersection_points[1].x() - 500.0) < 0.1);
    REQUIRE(std::fabs(intersection_points[1].y() - 557.365) < 0.1);
    REQUIRE(std::fabs(intersection_points[1].z() - 796.0) < 0.1);
}

TEST_CASE("Ray-Sphere Real-World Intersection", "[intersection]") {
    Line l0(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
    Point sphere_center(457.0, 192.0, 207.0);
    double radius = 265.0;
    
    std::vector<Point> sphere_points;
    bool result = Intersection::ray_sphere(l0, sphere_center, radius, sphere_points);
    
    REQUIRE(result);
    REQUIRE(sphere_points.size() == 2);
    
    // First intersection point
    REQUIRE(std::fabs(sphere_points[0].x() - 500.0) < 0.1);
    REQUIRE(std::fabs(sphere_points[0].y() - 12.08) < 0.1);
    REQUIRE(std::fabs(sphere_points[0].z() - 17.25) < 0.1);
    
    // Second intersection point
    REQUIRE(std::fabs(sphere_points[1].x() - 500.0) < 0.1);
    REQUIRE(std::fabs(sphere_points[1].y() - 308.77) < 0.1);
    REQUIRE(std::fabs(sphere_points[1].z() - 440.97) < 0.1);
}

TEST_CASE("Ray-Triangle Real-World Intersection", "[intersection]") {
    Line l0(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
    Point p1(214.0, 567.0, 484.0);
    Point p2(214.0, 192.0, 796.0);
    Point p3(694.0, 192.0, 484.0);
    
    Point triangle_hit;
    bool result = Intersection::ray_triangle(l0, p1, p2, p3, Tolerance::APPROXIMATION, triangle_hit);
    
    REQUIRE(result);
    REQUIRE(std::fabs(triangle_hit.x() - 500.0) < 0.1);
    REQUIRE(std::fabs(triangle_hit.y() - 340.616) < 0.01);
    REQUIRE(std::fabs(triangle_hit.z() - 486.451) < 0.01);
}

