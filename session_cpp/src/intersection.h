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

  /**
   * @brief Ray-mesh intersection result containing hit information
   */
  struct RayHit {
    double t;           ///< Parameter along ray where intersection occurs
    Point point;       ///< 3D intersection point
    double u;           ///< Barycentric coordinate u
    double v;           ///< Barycentric coordinate v
    int face_index;    ///< Index of the intersected face
    
    RayHit() : t(0.0), point(), u(0.0), v(0.0), face_index(-1) {}
    RayHit(double t_, const Point& p, double u_ = 0.0, double v_ = 0.0, int face_idx = -1)
      : t(t_), point(p), u(u_), v(v_), face_index(face_idx) {}
  };

  /**
   * @brief Find intersection point between two 3D lines
   * @param line0 First line
   * @param line1 Second line
   * @param output Intersection point (midpoint of closest approach for skew lines)
   * @param tolerance Maximum distance between lines to consider them intersecting
   * @return true if lines intersect within tolerance, false otherwise
   */
  static bool line_line(
    const Line& line0,
    const Line& line1,
    Point& output,
    double tolerance
  );

  /**
   * @brief Find parametric values where two lines are closest
   * @param line0 First line
   * @param line1 Second line
   * @param t0 Parameter on line0 (0=start, 1=end)
   * @param t1 Parameter on line1 (0=start, 1=end)
   * @param tolerance Maximum distance to consider intersection
   * @param intersect_segments If true, clamp parameters to [0,1]; if false, treat as infinite lines
   * @param near_parallel_as_closest If true, return closest point for near-parallel lines
   * @return true if intersection found within tolerance, false otherwise
   */
  static bool line_line_parameters(
    const Line& line0,
    const Line& line1,
    double& t0,
    double& t1,
    double tolerance,
    bool intersect_segments = true,
    bool near_parallel_as_closest = false
  );

  /**
   * @brief Find intersection line between two planes
   * @param plane0 First plane
   * @param plane1 Second plane
   * @param output Intersection line (infinite)
   * @return true if planes intersect (not parallel), false otherwise
   */
  static bool plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    Line& output
  );

  /**
   * @brief Find intersection point between a line and a plane
   * @param line Line to intersect
   * @param plane Plane to intersect
   * @param output Intersection point
   * @param is_finite If true, treat line as finite segment; if false, treat as infinite
   * @return true if intersection exists, false if line is parallel to plane
   */
  static bool line_plane(
    const Line& line,
    const Plane& plane,
    Point& output,
    bool is_finite = true
  );

  /**
   * @brief Find intersection point of three planes
   * @param plane0 First plane
   * @param plane1 Second plane
   * @param plane2 Third plane
   * @param output Intersection point
   * @return true if planes intersect at a point, false if parallel or degenerate
   */
  static bool plane_plane_plane(
    const Plane& plane0,
    const Plane& plane1,
    const Plane& plane2,
    Point& output
  );

  /**
   * @brief Find parametric intersection of ray with axis-aligned bounding box
   * @param origin Ray origin point
   * @param direction Ray direction vector (not necessarily normalized)
   * @param box Axis-aligned bounding box
   * @param t0 Minimum parameter value to consider (e.g., 0.0 for ray origin)
   * @param t1 Maximum parameter value to consider (e.g., 1000.0 for max distance)
   * @param tmin Output: parameter where ray enters box
   * @param tmax Output: parameter where ray exits box
   * @return true if ray intersects box within [t0, t1], false otherwise
   */
  static bool ray_box(
    const Point& origin,
    const Vector& direction,
    const BoundingBox& box,
    double t0,
    double t1,
    double& tmin,
    double& tmax
  );

  /**
   * @brief Find parametric intersection of line with axis-aligned bounding box
   * @param line Line to intersect (start point + direction)
   * @param box Axis-aligned bounding box
   * @param t0 Minimum parameter value to consider
   * @param t1 Maximum parameter value to consider
   * @param tmin Output: parameter where line enters box
   * @param tmax Output: parameter where line exits box
   * @return true if line intersects box within [t0, t1], false otherwise
   */
  static bool ray_box(
    const Line& line,
    const BoundingBox& box,
    double t0,
    double t1,
    double& tmin,
    double& tmax
  );

  /**
   * @brief Find intersection points between line and axis-aligned bounding box
   * @param line Line to intersect
   * @param box Axis-aligned bounding box
   * @param t0 Minimum parameter value to consider
   * @param t1 Maximum parameter value to consider
   * @param intersection_points Output: vector of intersection points (entry and exit)
   * @return true if intersection exists, false otherwise
   * @note Returns 2 points: [0]=entry (closer to line start), [1]=exit (farther from line start)
   */
  static bool ray_box(
    const Line& line,
    const BoundingBox& box,
    double t0,
    double t1,
    std::vector<Point>& intersection_points
  );

  /**
   * @brief Find parametric intersection of ray with sphere
   * @param origin Ray origin point
   * @param direction Ray direction vector (not necessarily normalized)
   * @param center Sphere center point
   * @param radius Sphere radius
   * @param t0 Output: parameter of first intersection (closer to origin)
   * @param t1 Output: parameter of second intersection (farther from origin)
   * @return Number of intersections: 0 (miss), 1 (tangent), or 2 (entry/exit)
   */
  static int ray_sphere(
    const Point& origin,
    const Vector& direction,
    const Point& center,
    double radius,
    double& t0,
    double& t1
  );

  /**
   * @brief Find intersection points between line and sphere
   * @param line Line to intersect
   * @param center Sphere center point
   * @param radius Sphere radius
   * @param intersection_points Output: vector of intersection points
   * @return true if intersection exists, false otherwise
   * @note Returns 1 point (tangent) or 2 points (entry/exit), sorted from line start
   */
  static bool ray_sphere(
    const Line& line,
    const Point& center,
    double radius,
    std::vector<Point>& intersection_points
  );

  /**
   * @brief Find intersection of ray with triangle using Möller-Trumbore algorithm
   * @param origin Ray origin point
   * @param direction Ray direction vector (not necessarily normalized)
   * @param v0 First vertex of triangle
   * @param v1 Second vertex of triangle
   * @param v2 Third vertex of triangle
   * @param epsilon Tolerance for parallel detection
   * @param t Output: parameter along ray where intersection occurs
   * @param u Output: barycentric coordinate u
   * @param v Output: barycentric coordinate v (w = 1-u-v)
   * @param parallel Output: true if ray is parallel to triangle
   * @return true if intersection exists, false otherwise
   */
  static bool ray_triangle(
    const Point& origin,
    const Vector& direction,
    const Point& v0,
    const Point& v1,
    const Point& v2,
    double epsilon,
    double& t,
    double& u,
    double& v,
    bool& parallel
  );

  /**
   * @brief Find intersection point between line and triangle
   * @param line Line to intersect (start point used as origin, direction computed internally)
   * @param v0 First vertex of triangle
   * @param v1 Second vertex of triangle
   * @param v2 Third vertex of triangle
   * @param epsilon Tolerance for parallel detection
   * @param output Output: intersection point
   * @return true if intersection exists, false otherwise
   */
  static bool ray_triangle(
    const Line& line,
    const Point& v0,
    const Point& v1,
    const Point& v2,
    double epsilon,
    Point& output
  );

  /**
   * @brief Find intersection of ray with mesh (naive brute-force)
   * @param origin Ray origin point
   * @param direction Ray direction vector (not necessarily normalized)
   * @param mesh Mesh to intersect
   * @param hits Output: vector of ray-triangle intersections
   * @param find_all If true, find all intersections; if false, stop at first hit
   * @return true if any intersection found, false otherwise
   */
  static bool ray_mesh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all = false
  );

  /**
   * @brief Find intersection of ray with mesh using BVH acceleration
   * @param origin Ray origin point
   * @param direction Ray direction vector (not necessarily normalized)
   * @param mesh Mesh to intersect
   * @param hits Output: vector of ray-triangle intersections
   * @param find_all If true, find all intersections; if false, stop at first hit
   * @return true if any intersection found, false otherwise
   * @note Uses broad-phase culling with per-triangle AABBs (no hierarchical traversal)
   */
  static bool ray_mesh_bvh(
    const Point& origin,
    const Vector& direction,
    const Mesh& mesh,
    std::vector<RayHit>& hits,
    bool find_all = false
  );

  /**
   * @brief Find intersection(s) of line with mesh (simplified API)
   * @param line Line to intersect with mesh
   * @param mesh Mesh to intersect
   * @param epsilon Tolerance for intersection detection
   * @param find_all If true, find all intersections; if false, return only closest
   * @return Vector of intersection points sorted along the line (empty if no hits)
   */
  static std::vector<Point> ray_mesh(
    const Line& line,
    const Mesh& mesh,
    double epsilon,
    bool find_all = false
  );

  /**
   * @brief Find intersection(s) of line with mesh using BVH (simplified API)
   * @param line Line to intersect with mesh
   * @param mesh Mesh to intersect
   * @param epsilon Tolerance for intersection detection
   * @param find_all If true, find all intersections; if false, return only closest
   * @return Vector of intersection points sorted along the line (empty if no hits)
   */
  static std::vector<Point> ray_mesh_bvh(
    const Line& line,
    const Mesh& mesh,
    double epsilon,
    bool find_all = false
  );

private:
  static int solve_3x3(
    const double row0[3],
    const double row1[3],
    const double row2[3],
    double d0, double d1, double d2,
    double& x, double& y, double& z,
    double& pivot_ratio
  );
  static double plane_value_at(const Plane& plane, const Point& point);
};

} // namespace session_cpp
