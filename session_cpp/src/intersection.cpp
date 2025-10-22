#include "intersection.h"
#include <cmath>
#include <algorithm>
#include <limits>
#include <cstring>
#include <set>

namespace session_cpp {

int Intersection::solve_3x3(
    const double row0[3], const double row1[3], const double row2[3],
    double d0, double d1, double d2,
    double& x, double& y, double& z,
    double& pivot_ratio) {
    
    int i, j;
    double *p0, *p1, *p2;
    double temp, workarray[12], maxpiv, minpiv;
    
    const int sizeof_row = 3 * sizeof(row0[0]);
    
    pivot_ratio = x = y = z = 0.0;
    
    temp = std::fabs(row0[0]);
    i = j = 0;
    double val = std::fabs(row0[1]);
    if (val > temp) { temp = val; j = 1; }
    val = std::fabs(row0[2]);
    if (val > temp) { temp = val; j = 2; }
    val = std::fabs(row1[0]);
    if (val > temp) { temp = val; i = 1; j = 0; }
    val = std::fabs(row1[1]);
    if (val > temp) { temp = val; i = 1; j = 1; }
    val = std::fabs(row1[2]);
    if (val > temp) { temp = val; i = 1; j = 2; }
    val = std::fabs(row2[0]);
    if (val > temp) { temp = val; i = 2; j = 0; }
    val = std::fabs(row2[1]);
    if (val > temp) { temp = val; i = 2; j = 1; }
    val = std::fabs(row2[2]);
    if (val > temp) { temp = val; i = 2; j = 2; }
    
    if (temp == 0.0) return 0;
    
    maxpiv = minpiv = std::fabs(temp);
    p0 = workarray;
    
    switch (i) {
        case 1:
            std::memcpy(p0, row1, sizeof_row); p0[3] = d1; p0 += 4;
            std::memcpy(p0, row0, sizeof_row); p0[3] = d0; p0 += 4;
            std::memcpy(p0, row2, sizeof_row); p0[3] = d2;
            break;
        case 2:
            std::memcpy(p0, row2, sizeof_row); p0[3] = d2; p0 += 4;
            std::memcpy(p0, row1, sizeof_row); p0[3] = d1; p0 += 4;
            std::memcpy(p0, row0, sizeof_row); p0[3] = d0;
            break;
        default:
            std::memcpy(p0, row0, sizeof_row); p0[3] = d0; p0 += 4;
            std::memcpy(p0, row1, sizeof_row); p0[3] = d1; p0 += 4;
            std::memcpy(p0, row2, sizeof_row); p0[3] = d2;
            break;
    }
    
    double *x_addr = &x;
    double *y_addr = &y;
    double *z_addr = &z;
    
    switch (j) {
        case 1:
            std::swap(x_addr, y_addr);
            p0 = &workarray[0];
            std::swap(p0[0], p0[1]); p0 += 4;
            std::swap(p0[0], p0[1]); p0 += 4;
            std::swap(p0[0], p0[1]);
            break;
        case 2:
            std::swap(x_addr, z_addr);
            p0 = &workarray[0];
            std::swap(p0[0], p0[2]); p0 += 4;
            std::swap(p0[0], p0[2]); p0 += 4;
            std::swap(p0[0], p0[2]);
            break;
    }
    
    temp = 1.0 / workarray[0];
    p0 = p1 = workarray + 1;
    *p1++ *= temp; *p1++ *= temp; *p1++ *= temp;
    temp = -(*p1++);
    if (temp != 0.0) {
        *p1++ += temp * (*p0++);
        *p1++ += temp * (*p0++);
        *p1++ += temp * (*p0);
        p0 -= 2;
    } else {
        p1 += 3;
    }
    temp = -(*p1++);
    if (temp != 0.0) {
        *p1++ += temp * (*p0++);
        *p1++ += temp * (*p0++);
        *p1++ += temp * (*p0);
        p0 -= 2;
    }
    
    temp = std::fabs(workarray[5]);
    i = j = 0;
    val = std::fabs(workarray[6]);
    if (val > temp) { temp = val; j = 1; }
    val = std::fabs(workarray[9]);
    if (val > temp) { temp = val; i = 1; j = 0; }
    val = std::fabs(workarray[10]);
    if (val > temp) { temp = val; i = j = 1; }
    
    if (temp == 0.0) return 1;
    
    val = std::fabs(temp);
    if (val > maxpiv) maxpiv = val;
    else if (val < minpiv) minpiv = val;
    
    if (j) {
        p0 = workarray + 1; p1 = p0 + 1;
        std::swap(*p0, *p1); p0 += 4; p1 += 4;
        std::swap(*p0, *p1); p0 += 4; p1 += 4;
        std::swap(*p0, *p1);
        std::swap(y_addr, z_addr);
    }
    
    if (i) {
        p0 = workarray + 1;
        p1 = p0 + 8;
        p2 = p0 + 4;
    } else {
        p0 = workarray + 1;
        p1 = p0 + 4;
        p2 = p0 + 8;
    }
    
    temp = 1.0 / (*p1++);
    *p1++ *= temp; *p1 *= temp; p1--;
    temp = -(*p0++);
    if (temp != 0.0) {
        *p0++ += temp * (*p1++);
        *p0 += temp * (*p1);
        p0--; p1--;
    }
    temp = -(*p2++);
    if (temp != 0.0) {
        *p2++ += temp * (*p1++);
        *p2 += temp * (*p1);
        p2--; p1--;
    }
    
    temp = *p2++;
    if (temp == 0.0) return 2;
    
    val = std::fabs(temp);
    if (val > maxpiv) maxpiv = val;
    else if (val < minpiv) minpiv = val;
    
    *p2 /= temp;
    temp = -(*p1++);
    if (temp != 0.0) *p1 += temp * (*p2);
    temp = -(*p0++);
    if (temp != 0.0) *p0 += temp * (*p2);
    
    *x_addr = workarray[3];
    if (i) {
        *y_addr = workarray[11];
        *z_addr = workarray[7];
    } else {
        *y_addr = workarray[7];
        *z_addr = workarray[11];
    }
    
    pivot_ratio = minpiv / maxpiv;
    return 3;
}

double Intersection::plane_value_at(const Plane& plane, const Point& point) {
    return plane.a() * point.x() + plane.b() * point.y() + 
           plane.c() * point.z() + plane.d();
}

bool Intersection::line_line(
    const Line& line0,
    const Line& line1,
    Point& output,
    double tolerance) {
    
    // Use the robust OpenNURBS-style line_line_parameters method
    double t0, t1;
    bool rc = line_line_parameters(line0, line1, t0, t1, tolerance, true, false);
    
    if (rc) {
        // Compute the midpoint between the two closest points
        Point p0 = line0.point_at(t0);
        Point p1 = line1.point_at(t1);
        output = Point(
            (p0.x() + p1.x()) * 0.5,
            (p0.y() + p1.y()) * 0.5,
            (p0.z() + p1.z()) * 0.5
        );
    }
    
    return rc;
}

bool Intersection::line_line_parameters(
    const Line& line0,
    const Line& line1,
    double& t0,
    double& t1,
    double tolerance,
    bool intersect_segments,
    bool near_parallel_as_closest) {
    
    // OpenNURBS-style: Check for exact endpoint matches first
    Point p0_start = line0.start();
    Point p0_end = line0.end();
    Point p1_start = line1.start();
    Point p1_end = line1.end();
    
    if (p0_start.x() == p1_start.x() && p0_start.y() == p1_start.y() && p0_start.z() == p1_start.z()) {
        t0 = 0.0;
        t1 = 0.0;
        return true;
    }
    if (p0_start.x() == p1_end.x() && p0_start.y() == p1_end.y() && p0_start.z() == p1_end.z()) {
        t0 = 0.0;
        t1 = 1.0;
        return true;
    }
    if (p0_end.x() == p1_start.x() && p0_end.y() == p1_start.y() && p0_end.z() == p1_start.z()) {
        t0 = 1.0;
        t1 = 0.0;
        return true;
    }
    if (p0_end.x() == p1_end.x() && p0_end.y() == p1_end.y() && p0_end.z() == p1_end.z()) {
        t0 = 1.0;
        t1 = 1.0;
        return true;
    }
    
    // Use dot product method (OpenNURBS approach)
    Vector A = line0.to_vector();
    Vector B = line1.to_vector();
    Vector C(
        p1_start.x() - p0_start.x(),
        p1_start.y() - p0_start.y(),
        p1_start.z() - p0_start.z()
    );
    
    double AA = A.dot(A);
    double BB = B.dot(B);
    double AB = A.dot(B);
    double AC = A.dot(C);
    double BC = B.dot(C);
    
    // Solve 2x2 system: [AA -AB] [t0] = [AC]
    //                   [-AB BB] [t1]   [-BC]
    double det = AA * BB - AB * AB;
    
    // Check for parallel lines (determinant near zero)
    double zero_tol = std::max(AA, BB) * std::numeric_limits<double>::epsilon();
    if (std::fabs(det) < zero_tol) {
        if (!near_parallel_as_closest) {
            return false;
        }
        // Compute closest points for (near) parallel lines
        t0 = (AA > 0.0) ? (AC / AA) : 0.0;
        t1 = (BB > 0.0) ? ((BC + t0 * AB) / BB) : 0.0;

        if (intersect_segments) {
            if (t0 < 0.0) t0 = 0.0; else if (t0 > 1.0) t0 = 1.0;
            if (t1 < 0.0) t1 = 0.0; else if (t1 > 1.0) t1 = 1.0;
        }

        if (tolerance > 0.0) {
            Point pt0p = line0.point_at(t0);
            Point pt1p = line1.point_at(t1);
            return pt0p.distance(pt1p) <= tolerance;
        }
        return true;
    }
    
    double inv_det = 1.0 / det;
    t0 = (BB * AC - AB * BC) * inv_det;
    t1 = (AB * AC - AA * BC) * inv_det;
    
    // Clamp to segment if requested
    if (intersect_segments) {
        if (t0 < 0.0) t0 = 0.0;
        else if (t0 > 1.0) t0 = 1.0;
        
        if (t1 < 0.0) t1 = 0.0;
        else if (t1 > 1.0) t1 = 1.0;
    }
    
    // Check distance tolerance if specified
    if (tolerance > 0.0) {
        Point pt0 = line0.point_at(t0);
        Point pt1 = line1.point_at(t1);
        double dist = pt0.distance(pt1);
        if (dist > tolerance) {
            return false;
        }
    }
    
    return true;
}

bool Intersection::plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    Line& output) {
    
    Vector d = plane1.z_axis().cross(plane0.z_axis());
    
    Point p = Point(
        (plane0.origin().x() + plane1.origin().x()) * 0.5,
        (plane0.origin().y() + plane1.origin().y()) * 0.5,
        (plane0.origin().z() + plane1.origin().z()) * 0.5
    );
    
    Plane plane2 = Plane::from_point_normal(p, d);
    
    Point output_p;
    bool rc = plane_plane_plane(plane0, plane1, plane2, output_p);
    if (!rc) {
        return false;
    }
    
    output = Line::from_points(output_p, Point(
        output_p.x() + d.x(),
        output_p.y() + d.y(),
        output_p.z() + d.z()
    ));
    
    return true;
}

bool Intersection::line_plane(
    const Line& line,
    const Plane& plane,
    Point& output,
    bool is_finite) {
    
    bool rc = false;
    double a, b, d, fd, t;
    
    Point pt0 = line.start();
    Point pt1 = line.end();
    
    a = plane_value_at(plane, pt0);
    b = plane_value_at(plane, pt1);
    d = a - b;
    
    if (d == 0.0) {
        if (std::fabs(a) < std::fabs(b))
            t = 0.0;
        else if (std::fabs(b) < std::fabs(a))
            t = 1.0;
        else
            t = 0.5;
    } else {
        d = 1.0 / d;
        fd = std::fabs(d);
        if (fd > 1.0 && (std::fabs(a) >= std::numeric_limits<double>::max() / fd || 
                          std::fabs(b) >= std::numeric_limits<double>::max() / fd)) {
            t = 0.5;
        } else {
            t = a / (a - b);
            rc = true;
        }
    }
    
    const double s = 1.0 - t;
    
    output = Point(
        (line.x0() == line.x1()) ? line.x0() : s * line.x0() + t * line.x1(),
        (line.y0() == line.y1()) ? line.y0() : s * line.y0() + t * line.y1(),
        (line.z0() == line.z1()) ? line.z0() : s * line.z0() + t * line.z1()
    );
    
    if (is_finite && (t < 0.0 || t > 1.0))
        return false;
    
    return rc;
}

bool Intersection::plane_plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    const Plane& plane2,
    Point& output) {
    
    double pr = 0.0;
    double x, y, z;
    
    const double plane_0[3] = { plane0.a(), plane0.b(), plane0.c() };
    const double plane_1[3] = { plane1.a(), plane1.b(), plane1.c() };
    const double plane_2[3] = { plane2.a(), plane2.b(), plane2.c() };
    
    const int rank = solve_3x3(
        plane_0, plane_1, plane_2,
        -plane0.d(), -plane1.d(), -plane2.d(),
        x, y, z, pr
    );
    
    output = Point(x, y, z);
    return (rank == 3);
}

bool Intersection::ray_box(
    const Point& origin,
    const Vector& direction,
    const BoundingBox& box,
    double t0,
    double t1,
    double& tmin,
    double& tmax) {
    
    Point box_min = box.min_point();
    Point box_max = box.max_point();
    
    Vector inv_dir(
        (direction.x() != 0.0) ? 1.0 / direction.x() : std::numeric_limits<double>::max(),
        (direction.y() != 0.0) ? 1.0 / direction.y() : std::numeric_limits<double>::max(),
        (direction.z() != 0.0) ? 1.0 / direction.z() : std::numeric_limits<double>::max()
    );
    
    double tx1 = (box_min.x() - origin.x()) * inv_dir.x();
    double tx2 = (box_max.x() - origin.x()) * inv_dir.x();
    
    tmin = std::min(tx1, tx2);
    tmax = std::max(tx1, tx2);
    
    double ty1 = (box_min.y() - origin.y()) * inv_dir.y();
    double ty2 = (box_max.y() - origin.y()) * inv_dir.y();
    
    tmin = std::max(tmin, std::min(ty1, ty2));
    tmax = std::min(tmax, std::max(ty1, ty2));
    
    double tz1 = (box_min.z() - origin.z()) * inv_dir.z();
    double tz2 = (box_max.z() - origin.z()) * inv_dir.z();
    
    tmin = std::max(tmin, std::min(tz1, tz2));
    tmax = std::min(tmax, std::max(tz1, tz2));
    
    tmin = std::max(tmin, t0);
    tmax = std::min(tmax, t1);
    
    return tmax >= tmin;
}

bool Intersection::ray_box(
    const Line& line,
    const BoundingBox& box,
    double t0,
    double t1,
    double& tmin,
    double& tmax) {
    
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    return ray_box(origin, direction, box, t0, t1, tmin, tmax);
}

bool Intersection::ray_box(
    const Line& line,
    const BoundingBox& box,
    double t0,
    double t1,
    std::vector<Point>& intersection_points) {
    
    double tmin, tmax;
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    bool hit = ray_box(origin, direction, box, t0, t1, tmin, tmax);
    
    if (hit) {
        intersection_points.clear();
        
        // Entry point
        Point entry(
            origin.x() + direction.x() * tmin,
            origin.y() + direction.y() * tmin,
            origin.z() + direction.z() * tmin
        );
        intersection_points.push_back(entry);
        
        // Exit point
        Point exit(
            origin.x() + direction.x() * tmax,
            origin.y() + direction.y() * tmax,
            origin.z() + direction.z() * tmax
        );
        intersection_points.push_back(exit);
    }
    
    return hit;
}

int Intersection::ray_sphere(
    const Point& origin,
    const Vector& direction,
    const Point& center,
    double radius,
    double& t0,
    double& t1) {
    
    Vector o(
        origin.x() - center.x(),
        origin.y() - center.y(),
        origin.z() - center.z()
    );
    
    double a = direction.dot(direction);
    double b = 2.0 * direction.dot(o);
    double c = o.dot(o) - (radius * radius);
    
    double disc = b * b - 4.0 * a * c;
    
    if (disc < 0.0) {
        return 0;
    }
    
    double distSqrt = std::sqrt(disc);
    double q;
    if (b < 0.0) {
        q = (-b - distSqrt) / 2.0;
    } else {
        q = (-b + distSqrt) / 2.0;
    }
    
    t0 = q / a;
    double _t1 = c / q;
    
    if (_t1 == t0) {
        return 1;
    }
    
    t1 = _t1;
    
    if (t0 > t1) {
        std::swap(t0, t1);
    }
    
    return 2;
}

bool Intersection::ray_sphere(
    const Line& line,
    const Point& center,
    double radius,
    std::vector<Point>& intersection_points) {
    
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    double t0, t1;
    int hits = ray_sphere(origin, direction, center, radius, t0, t1);
    
    if (hits == 0) {
        return false;
    }
    
    intersection_points.clear();
    
    // First intersection point
    Point p0(
        origin.x() + direction.x() * t0,
        origin.y() + direction.y() * t0,
        origin.z() + direction.z() * t0
    );
    intersection_points.push_back(p0);
    
    // Second intersection point (if exists)
    if (hits == 2) {
        Point p1(
            origin.x() + direction.x() * t1,
            origin.y() + direction.y() * t1,
            origin.z() + direction.z() * t1
        );
        intersection_points.push_back(p1);
    }
    
    return true;
}

bool Intersection::ray_triangle(
    const Point& origin,
    const Vector& direction,
    const Point& v0,
    const Point& v1,
    const Point& v2,
    double epsilon,
    double& t,
    double& u,
    double& v,
    bool& parallel) {
    
    Vector edge1(v1.x() - v0.x(), v1.y() - v0.y(), v1.z() - v0.z());
    Vector edge2(v2.x() - v0.x(), v2.y() - v0.y(), v2.z() - v0.z());
    Vector pvec = direction.cross(edge2);
    
    double det = edge1.dot(pvec);
    
    if (det > -epsilon && det < epsilon) {
        parallel = true;
        return false;
    }
    
    parallel = false;
    double inv_det = 1.0 / det;
    
    Vector tvec(origin.x() - v0.x(), origin.y() - v0.y(), origin.z() - v0.z());
    u = tvec.dot(pvec) * inv_det;
    
    if (u < 0.0 - epsilon || u > 1.0 + epsilon) {
        return false;
    }
    
    Vector qvec = tvec.cross(edge1);
    v = direction.dot(qvec) * inv_det;
    
    if (v < 0.0 - epsilon || u + v > 1.0 + epsilon) {
        return false;
    }
    
    t = edge2.dot(qvec) * inv_det;
    return true;
}

bool Intersection::ray_triangle(
    const Line& line,
    const Point& v0,
    const Point& v1,
    const Point& v2,
    double epsilon,
    Point& output) {
    
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    double t, u, v;
    bool parallel;
    
    if (!ray_triangle(origin, direction, v0, v1, v2, epsilon, t, u, v, parallel)) {
        return false;
    }
    
    // Calculate intersection point: origin + t * direction
    output = Point(
        origin.x() + t * direction.x(),
        origin.y() + t * direction.y(),
        origin.z() + t * direction.z()
    );
    
    return true;
}

bool Intersection::ray_mesh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all) {
    
    hits.clear();
    
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    
    for (size_t i = 0; i < faces.size(); ++i) {
        const auto& face = faces[i];
        
        if (face.size() < 3) continue;
        
        for (size_t j = 1; j < face.size() - 1; ++j) {
            const Point& v0 = vertices[face[0]];
            const Point& v1 = vertices[face[j]];
            const Point& v2 = vertices[face[j + 1]];
            
            double t, u, v;
            bool parallel;
            
            if (ray_triangle(origin, direction, v0, v1, v2, 
                           static_cast<double>(Tolerance::ZERO_TOLERANCE), 
                           t, u, v, parallel)) {
                
                if (t >= 0.0) {
                    Point hit_point(
                        origin.x() + t * direction.x(),
                        origin.y() + t * direction.y(),
                        origin.z() + t * direction.z()
                    );
                    
                    hits.emplace_back(t, hit_point, u, v, static_cast<int>(i));
                    
                    if (!find_all) {
                        return true;
                    }
                }
            }
        }
    }
    
    if (!hits.empty()) {
        const double eps = 1e-6;
        std::sort(hits.begin(), hits.end(), 
                  [eps](const RayHit& a, const RayHit& b) {
                      double dt = a.t - b.t;
                      if (std::fabs(dt) <= eps) return a.face_index < b.face_index;
                      return a.t < b.t;
                  });
        return true;
    }
    
    return false;
}

bool Intersection::ray_mesh_bvh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all) {
    
    hits.clear();
    
    // Use Mesh's cached per-triangle BVH
    std::vector<int> candidates_list;
    if (!mesh.triangle_bvh_ray_cast(origin, direction, candidates_list, find_all)) {
        return false;
    }
    
    // Perform exact ray-triangle intersection on candidates
    bool any_hit = false;
    RayHit best_hit;
    double best_t = std::numeric_limits<double>::infinity();
    int best_face = std::numeric_limits<int>::max();
    for (int tri_id : candidates_list) {
        size_t face_idx, sub_idx;
        Point v0, v1, v2;
        if (!mesh.get_triangle_by_id(tri_id, face_idx, sub_idx, v0, v1, v2)) {
            continue;
        }
        
        double t, u, v;
        bool parallel;
        
        if (ray_triangle(origin, direction, v0, v1, v2, 
                       static_cast<double>(Tolerance::ZERO_TOLERANCE), 
                       t, u, v, parallel)) {
            if (t >= 0.0) {
                Point hit_point(
                    origin.x() + t * direction.x(),
                    origin.y() + t * direction.y(),
                    origin.z() + t * direction.z()
                );
                if (find_all) {
                    hits.emplace_back(t, hit_point, u, v, static_cast<int>(face_idx));
                } else {
                    const double eps = 1e-6;
                    if (t < best_t - eps || (std::fabs(t - best_t) <= eps && static_cast<int>(face_idx) < best_face)) {
                        best_t = t;
                        best_face = static_cast<int>(face_idx);
                        best_hit = RayHit(t, hit_point, u, v, static_cast<int>(face_idx));
                        any_hit = true;
                    }
                }
            }
        }
    }
    
    if (find_all) {
        if (!hits.empty()) {
            const double eps = 1e-6;
            std::sort(hits.begin(), hits.end(), 
                      [eps](const RayHit& a, const RayHit& b) {
                          double dt = a.t - b.t;
                          if (std::fabs(dt) <= eps) return a.face_index < b.face_index;
                          return a.t < b.t;
                      });
            return true;
        }
        return false;
    }
    
    if (any_hit) {
        hits.push_back(best_hit);
        return true;
    }
    return false;
}

std::vector<Point> Intersection::ray_mesh(
    const Line& line,
    const Mesh& mesh,
    double epsilon,
    bool find_all) {
    
    (void)epsilon;  // Unused - kept for API compatibility
    
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    std::vector<RayHit> hits;
    std::vector<Point> result;
    
    if (ray_mesh(origin, direction, mesh, hits, find_all)) {
        result.reserve(hits.size());
        for (const auto& hit : hits) {
            result.push_back(hit.point);
        }
    }
    
    return result;
}

std::vector<Point> Intersection::ray_mesh_bvh(
    const Line& line,
    const Mesh& mesh,
    double epsilon,
    bool find_all) {
    
    (void)epsilon;  // Unused - kept for API compatibility
    
    Point origin = line.start();
    Vector direction = line.to_vector();
    
    std::vector<RayHit> hits;
    std::vector<Point> result;
    
    if (ray_mesh_bvh(origin, direction, mesh, hits, find_all)) {
        result.reserve(hits.size());
        for (const auto& hit : hits) {
            result.push_back(hit.point);
        }
    }
    
    return result;
}

} // namespace session_cpp
