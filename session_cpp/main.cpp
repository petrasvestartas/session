#include "src/intersection.h"
#include "src/point.h"
#include "src/vector.h"
#include "src/line.h"
#include "src/plane.h"
#include "src/mesh.h"
#include "src/boundingbox.h"
#include <iostream>
#include <vector>

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

int main() {
    std::cout << "=== Intersection Examples ===\n\n";

    // 1. line_line
    Line l0(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
    Line l1(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);
    Point p;
    if (Intersection::line_line(l0, l1, p, 1e-3f)) {
        std::cout << "1. line_line: " << p.x() << ", " << p.y() << ", " << p.z() << "\n";
    }

    // 2. line_line_parameters
    float t0, t1;
    if (Intersection::line_line_parameters(l0, l1, t0, t1)) {
        std::cout << "2. line_line_parameters: t0=" << t0 << ", t1=" << t1 << "\n";
    }

    // 3. plane_plane
    Plane pl0 = Plane::from_point_normal(origin, vz);
    Plane pl1 = Plane::from_point_normal(origin, vx);
    Line intersection_line;
    if (Intersection::plane_plane(pl0, pl1, intersection_line)) {
        std::cout << "3. plane_plane: line from (" << intersection_line.start().x() << "," 
                  << intersection_line.start().y() << "," << intersection_line.start().z() << ")\n";
    }

    // 4. line_plane
    Line l2(0, 0, -1000, 0, 0, 1000);
    Plane pl2 = Plane::from_point_normal(origin, vz);
    Point lp;
    if (Intersection::line_plane(l2, pl2, lp)) {
        std::cout << "4. line_plane: " << lp.x() << ", " << lp.y() << ", " << lp.z() << "\n";
    }

    // 5. plane_plane_plane
    Plane px = Plane::from_point_normal(origin, vx);
    Plane py = Plane::from_point_normal(origin, vy);
    Plane pz = Plane::from_point_normal(origin, vz);
    Point ppp;
    if (Intersection::plane_plane_plane(px, py, pz, ppp)) {
        std::cout << "5. plane_plane_plane: " << ppp.x() << ", " << ppp.y() << ", " << ppp.z() << "\n";
    }

    // 6. ray_box
    BoundingBox box(Point(1000, 1000, 1000), vx, vy, vz, Vector(500, 500, 500));
    float tmin, tmax;
    if (Intersection::ray_box(origin, dir, box, 0.0f, 1000.0f, tmin, tmax)) {
        std::cout << "6. ray_box: tmin=" << tmin << ", tmax=" << tmax << "\n";
    }

    // 7. ray_sphere
    float ts0, ts1;
    int hits = Intersection::ray_sphere(origin, vx, sphere_center, 1000.0f, ts0, ts1);
    std::cout << "7. ray_sphere: hits=" << hits << ", t0=" << ts0 << ", t1=" << ts1 << "\n";

    // 8. ray_triangle
    float t, u, v;
    bool parallel;
    if (Intersection::ray_triangle(origin, dir, p1, p2, p3, 1e-6f, t, u, v, parallel)) {
        std::cout << "8. ray_triangle: t=" << t << ", u=" << u << ", v=" << v << "\n";
    }

    // 9. ray_mesh
    Mesh mesh;
    auto va = mesh.add_vertex(p1);
    auto vb = mesh.add_vertex(p2);
    auto vc = mesh.add_vertex(p3);
    mesh.add_face({va, vb, vc});
    std::vector<Intersection::RayHit> mesh_hits;
    if (Intersection::ray_mesh(origin, dir, mesh, mesh_hits)) {
        std::cout << "9. ray_mesh: " << mesh_hits.size() << " hits, t=" << mesh_hits[0].t << "\n";
    }

    // 10. ray_mesh_bvh
    std::vector<Intersection::RayHit> bvh_hits;
    if (Intersection::ray_mesh_bvh(origin, dir, mesh, bvh_hits)) {
        std::cout << "10. ray_mesh_bvh: " << bvh_hits.size() << " hits, t=" << bvh_hits[0].t << "\n";
    }

    return 0;
}
