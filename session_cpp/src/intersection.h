#pragma once
#include "point.h"
#include "vector.h"
#include "plane.h"
#include "line.h"
#include "mesh.h"
#include "boundingbox.h"
#include "bvh.h"
#include "tolerance.h"
#include <tuple>
#include <vector>
#include <optional>

namespace session_cpp {

class Intersection {
public:

  struct RayHit {
    float t;
    Point point;
    float u;
    float v;
    int face_index;
    
    RayHit() : t(0.0f), point(), u(0.0f), v(0.0f), face_index(-1) {}
    RayHit(float t_, const Point& p, float u_ = 0.0f, float v_ = 0.0f, int face_idx = -1)
      : t(t_), point(p), u(u_), v(v_), face_index(face_idx) {}
  };

  static bool line_line(
    const Line& line0,
    const Line& line1,
    Point& output,
    float tolerance = static_cast<float>(Tolerance::APPROXIMATION)
  );

  static bool line_line_parameters(
    const Line& line0,
    const Line& line1,
    float& t0,
    float& t1,
    float tolerance = static_cast<float>(Tolerance::APPROXIMATION),
    bool intersect_segments = true,
    bool near_parallel_as_closest = false
  );

  static bool plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    Line& output
  );

  static bool line_plane(
    const Line& line,
    const Plane& plane,
    Point& output,
    bool is_finite = true
  );

  static bool plane_plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    const Plane& plane2,
    Point& output
  );

  // Assumes an axis-aligned bounding box (AABB) defined in world axes via min/max.
  static bool ray_box(
    const Point& origin,
    const Vector& direction,
    const BoundingBox& box,
    float t0,
    float t1,
    float& tmin,
    float& tmax
  );

  static int ray_sphere(
    const Point& origin,
    const Vector& direction,
    const Point& center,
    float radius,
    float& t0,
    float& t1
  );

  static bool ray_triangle(
    const Point& origin,
    const Vector& direction,
    const Point& v0,
    const Point& v1,
    const Point& v2,
    float epsilon,
    float& t,
    float& u,
    float& v,
    bool& parallel
  );

  static bool ray_mesh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all = false
  );

  // Broad-phase culling using per-triangle AABBs (no hierarchical traversal).
  static bool ray_mesh_bvh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all = false
  );

private:
  static int solve_3x3(
    const float row0[3],
    const float row1[3],
    const float row2[3],
    float d0, float d1, float d2,
    float& x, float& y, float& z,
    float& pivot_ratio
  );
  static float plane_value_at(const Plane& plane, const Point& point);
};

} // namespace session_cpp
