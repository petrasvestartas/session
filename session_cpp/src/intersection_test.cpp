#include "intersection.h"
#include "catch_amalgamated.hpp"
#include <cmath>

using namespace session_cpp;

TEST_CASE("Line-Line Intersection", "[intersection]") {
    Line line0(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    Line line1(0.5f, -1.0f, 0.0f, 0.5f, 1.0f, 0.0f);
    
    Point output;
    bool result = Intersection::line_line(line0, line1, output);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 0.5f) < 1e-5f);
    REQUIRE(std::fabs(output.y() - 0.0f) < 1e-5f);
    REQUIRE(std::fabs(output.z() - 0.0f) < 1e-5f);
}

TEST_CASE("Line-Line Parallel", "[intersection]") {
    Line line0(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    Line line1(0.0f, 1.0f, 0.0f, 1.0f, 1.0f, 0.0f);
    
    Point output;
    bool result = Intersection::line_line(line0, line1, output);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Line-Line Parameters", "[intersection]") {
    Line line0(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    Line line1(0.5f, -1.0f, 0.0f, 0.5f, 1.0f, 0.0f);
    
    float t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1);
    
    REQUIRE(result);
    REQUIRE(std::fabs(t0 - 0.5f) < 1e-5f);
    REQUIRE(std::fabs(t1 - 0.5f) < 1e-5f);
}

TEST_CASE("Line-Line Parameters Exact Endpoints", "[intersection]") {
    Line line0(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    Line line1(0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f);
    
    float t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1);
    
    REQUIRE(result);
    REQUIRE(t0 == 0.0f);
    REQUIRE(t1 == 0.0f);
}

TEST_CASE("Line-Line Parameters Infinite Lines", "[intersection]") {
    Line line0(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    Line line1(2.0f, -1.0f, 0.0f, 2.0f, 1.0f, 0.0f);
    
    float t0, t1;
    bool result = Intersection::line_line_parameters(line0, line1, t0, t1, 0.0f, false);
    
    REQUIRE(result);
    REQUIRE(std::fabs(t0 - 2.0f) < 1e-5f);
}

TEST_CASE("Plane-Plane Intersection", "[intersection]") {
    Point p0(0.0f, 0.0f, 0.0f);
    Vector n0(0.0f, 0.0f, 1.0f);
    Plane plane0 = Plane::from_point_normal(p0, n0);
    
    Point p1(0.0f, 0.0f, 0.0f);
    Vector n1(0.0f, 1.0f, 0.0f);
    Plane plane1 = Plane::from_point_normal(p1, n1);
    
    Line output;
    bool result = Intersection::plane_plane(plane0, plane1, output);
    
    REQUIRE(result);
    
    Vector line_dir = output.to_vector();
    REQUIRE(std::fabs(std::fabs(line_dir.x()) - 1.0f) < 1e-4f);
    REQUIRE(std::fabs(line_dir.y()) < 1e-4f);
    REQUIRE(std::fabs(line_dir.z()) < 1e-4f);
}

TEST_CASE("Line-Plane Intersection", "[intersection]") {
    Point p(0.0f, 0.0f, 1.0f);
    Vector n(0.0f, 0.0f, 1.0f);
    Plane plane = Plane::from_point_normal(p, n);
    
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 2.0f);
    
    Point output;
    bool result = Intersection::line_plane(line, plane, output, true);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 0.0f) < 1e-5f);
    REQUIRE(std::fabs(output.y() - 0.0f) < 1e-5f);
    REQUIRE(std::fabs(output.z() - 1.0f) < 1e-5f);
}

TEST_CASE("Line-Plane Parallel", "[intersection]") {
    Point p(0.0f, 0.0f, 1.0f);
    Vector n(0.0f, 0.0f, 1.0f);
    Plane plane = Plane::from_point_normal(p, n);
    
    Line line(0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f);
    
    Point output;
    bool result = Intersection::line_plane(line, plane, output, true);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Plane-Plane-Plane Intersection", "[intersection]") {
    Point p0(0.0f, 0.0f, 0.0f);
    Vector n0(0.0f, 0.0f, 1.0f);
    Plane plane0 = Plane::from_point_normal(p0, n0);
    
    Point p1(0.0f, 0.0f, 0.0f);
    Vector n1(0.0f, 1.0f, 0.0f);
    Plane plane1 = Plane::from_point_normal(p1, n1);
    
    Point p2(0.0f, 0.0f, 0.0f);
    Vector n2(1.0f, 0.0f, 0.0f);
    Plane plane2 = Plane::from_point_normal(p2, n2);
    
    Point output;
    bool result = Intersection::plane_plane_plane(plane0, plane1, plane2, output);
    
    REQUIRE(result);
    REQUIRE(std::fabs(output.x() - 0.0f) < 1e-4f);
    REQUIRE(std::fabs(output.y() - 0.0f) < 1e-4f);
    REQUIRE(std::fabs(output.z() - 0.0f) < 1e-4f);
}

TEST_CASE("Plane-Plane-Plane Parallel", "[intersection]") {
    Point p0(0.0f, 0.0f, 0.0f);
    Vector n0(0.0f, 0.0f, 1.0f);
    Plane plane0 = Plane::from_point_normal(p0, n0);
    
    Point p1(0.0f, 0.0f, 1.0f);
    Vector n1(0.0f, 0.0f, 1.0f);
    Plane plane1 = Plane::from_point_normal(p1, n1);
    
    Point p2(0.0f, 0.0f, 0.0f);
    Vector n2(1.0f, 0.0f, 0.0f);
    Plane plane2 = Plane::from_point_normal(p2, n2);
    
    Point output;
    bool result = Intersection::plane_plane_plane(plane0, plane1, plane2, output);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Box Intersection", "[intersection]") {
    Point center(0.0f, 0.0f, 0.0f);
    Vector x_axis(1.0f, 0.0f, 0.0f);
    Vector y_axis(0.0f, 1.0f, 0.0f);
    Vector z_axis(0.0f, 0.0f, 1.0f);
    Vector half_size(1.0f, 1.0f, 1.0f);
    BoundingBox box(center, x_axis, y_axis, z_axis, half_size);
    
    Point origin(-5.0f, 0.0f, 0.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    
    float tmin, tmax;
    bool result = Intersection::ray_box(origin, direction, box, 0.0f, 100.0f, tmin, tmax);
    
    REQUIRE(result);
    REQUIRE(std::fabs(tmin - 4.0f) < 1e-4f);
    REQUIRE(std::fabs(tmax - 6.0f) < 1e-4f);
}

TEST_CASE("Ray-Box Miss", "[intersection]") {
    Point center(0.0f, 0.0f, 0.0f);
    Vector x_axis(1.0f, 0.0f, 0.0f);
    Vector y_axis(0.0f, 1.0f, 0.0f);
    Vector z_axis(0.0f, 0.0f, 1.0f);
    Vector half_size(1.0f, 1.0f, 1.0f);
    BoundingBox box(center, x_axis, y_axis, z_axis, half_size);
    
    Point origin(-5.0f, 5.0f, 0.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    
    float tmin, tmax;
    bool result = Intersection::ray_box(origin, direction, box, 0.0f, 100.0f, tmin, tmax);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Sphere Intersection", "[intersection]") {
    Point origin(-5.0f, 0.0f, 0.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    Point center(0.0f, 0.0f, 0.0f);
    float radius = 2.0f;
    
    float t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 2);
    REQUIRE(std::fabs(t0 - 3.0f) < 1e-4f);
    REQUIRE(std::fabs(t1 - 7.0f) < 1e-4f);
}

TEST_CASE("Ray-Sphere Tangent", "[intersection]") {
    Point origin(-5.0f, 2.0f, 0.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    Point center(0.0f, 0.0f, 0.0f);
    float radius = 2.0f;
    
    float t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 1);
    REQUIRE(std::fabs(t0 - 5.0f) < 1e-4f);
}

TEST_CASE("Ray-Sphere Miss", "[intersection]") {
    Point origin(-5.0f, 5.0f, 0.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    Point center(0.0f, 0.0f, 0.0f);
    float radius = 2.0f;
    
    float t0, t1;
    int hits = Intersection::ray_sphere(origin, direction, center, radius, t0, t1);
    
    REQUIRE(hits == 0);
}

TEST_CASE("Ray-Triangle Intersection", "[intersection]") {
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    Point v0(0.0f, 0.0f, 0.0f);
    Point v1(1.0f, 0.0f, 0.0f);
    Point v2(0.0f, 1.0f, 0.0f);
    
    float t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6f, t, u, v, parallel);
    
    REQUIRE(result);
    REQUIRE_FALSE(parallel);
    REQUIRE(std::fabs(t - 1.0f) < 1e-4f);
}

TEST_CASE("Ray-Triangle Miss", "[intersection]") {
    Point origin(2.0f, 2.0f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    Point v0(0.0f, 0.0f, 0.0f);
    Point v1(1.0f, 0.0f, 0.0f);
    Point v2(0.0f, 1.0f, 0.0f);
    
    float t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6f, t, u, v, parallel);
    
    REQUIRE_FALSE(result);
}

TEST_CASE("Ray-Triangle Parallel", "[intersection]") {
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(1.0f, 0.0f, 0.0f);
    
    Point v0(0.0f, 0.0f, 0.0f);
    Point v1(1.0f, 0.0f, 0.0f);
    Point v2(0.0f, 1.0f, 0.0f);
    
    float t, u, v;
    bool parallel;
    bool result = Intersection::ray_triangle(origin, direction, v0, v1, v2, 1e-6f, t, u, v, parallel);
    
    REQUIRE_FALSE(result);
    REQUIRE(parallel);
}

TEST_CASE("Ray-Mesh Intersection", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)},
        {Point(0.0f, 0.0f, 1.0f), Point(1.0f, 0.0f, 1.0f), Point(1.0f, 1.0f, 1.0f), Point(0.0f, 1.0f, 1.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, true);
    
    REQUIRE(result);
    REQUIRE(hits.size() >= 1);
    REQUIRE(std::fabs(hits[0].t - 1.0f) < 1e-3f);
}

TEST_CASE("Ray-Mesh Find First", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)},
        {Point(0.0f, 0.0f, 1.0f), Point(1.0f, 0.0f, 1.0f), Point(1.0f, 1.0f, 1.0f), Point(0.0f, 1.0f, 1.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, false);
    
    REQUIRE(result);
    REQUIRE(hits.size() == 1);
}

TEST_CASE("Ray-Mesh Miss", "[intersection]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.0f, 5.0f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh(origin, direction, mesh, hits, true);
    
    REQUIRE_FALSE(result);
    REQUIRE(hits.size() == 0);
}

TEST_CASE("Ray-Mesh BVH Intersection", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)},
        {Point(0.0f, 0.0f, 1.0f), Point(1.0f, 0.0f, 1.0f), Point(1.0f, 1.0f, 1.0f), Point(0.0f, 1.0f, 1.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh_bvh(origin, direction, mesh, hits, true);
    
    REQUIRE(result);
    REQUIRE(hits.size() >= 1);
    REQUIRE(std::fabs(hits[0].t - 1.0f) < 1e-3f);
}

TEST_CASE("Ray-Mesh BVH Find First", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)},
        {Point(0.0f, 0.0f, 1.0f), Point(1.0f, 0.0f, 1.0f), Point(1.0f, 1.0f, 1.0f), Point(0.0f, 1.0f, 1.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(0.5f, 0.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
    std::vector<Intersection::RayHit> hits;
    bool result = Intersection::ray_mesh_bvh(origin, direction, mesh, hits, false);
    
    REQUIRE(result);
    REQUIRE(hits.size() == 1);
}

TEST_CASE("Ray-Mesh BVH Miss", "[intersection][bvh]") {
    std::vector<std::vector<Point>> polygons = {
        {Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)}
    };
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.0f, 5.0f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
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
            float x = static_cast<float>(i);
            float y = static_cast<float>(j);
            polygons.push_back({
                Point(x, y, 0.0f),
                Point(x + 1.0f, y, 0.0f),
                Point(x + 1.0f, y + 1.0f, 0.0f),
                Point(x, y + 1.0f, 0.0f)
            });
        }
    }
    
    Mesh mesh = Mesh::from_polygons(polygons);
    
    Point origin(5.5f, 5.5f, -1.0f);
    Vector direction(0.0f, 0.0f, 1.0f);
    
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
        REQUIRE(std::fabs(hits_naive[0].t - hits_bvh[0].t) < 1e-4f);
        REQUIRE(hits_naive[0].face_index == hits_bvh[0].face_index);
    }
}

