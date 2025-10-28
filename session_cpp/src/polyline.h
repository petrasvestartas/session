#pragma once
#include "plane.h"
#include "point.h"
#include "vector.h"
#include "color.h"
#include "xform.h"
#include "xform.h"
#include "guid.h"
#include "json.h"
#include <vector>
#include <string>
#include <fstream>
#include <iostream>

namespace session_cpp {


/**
 * @class Polyline
 * @brief A polyline defined by a collection of points with an associated plane.
 */
class Polyline {
public:
    std::string guid = ::guid();
    std::string name = "my_polyline";
    std::vector<Point> points;
    Plane plane;
    double width = 1.0;
    Color linecolor = Color::white();

    Xform xform;

    /// Default constructor
    Polyline();

    /// Constructor with points
    explicit Polyline(const std::vector<Point>& pts);

    /// Returns the number of points in the polyline
    size_t len() const;

    /// Returns true if the polyline has no points
    bool is_empty() const;

    /// Returns the number of segments (n-1 for n points)
    size_t segment_count() const;

    /// Calculates the total length of the polyline
    double length() const;

    /// Returns a pointer to the point at the given index (nullptr if out of bounds)
    Point* get_point(size_t index);
    const Point* get_point(size_t index) const;

    /// Adds a point to the end of the polyline
    void add_point(const Point& point);

    /// Inserts a point at the specified index
    void insert_point(size_t index, const Point& point);

    /// Removes and returns the point at the specified index
    bool remove_point(size_t index, Point& out_point);

    /// Reverses the order of points in the polyline
    void reverse();

    /// Returns a new polyline with reversed point order
    Polyline reversed() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Translates all points by a vector (in-place)
    Polyline& operator+=(const Vector& v);

    /// Translates all points by a vector (returns new polyline)
    Polyline operator+(const Vector& v) const;

    /// Translates all points by negative vector (in-place)
    Polyline& operator-=(const Vector& v);

    /// Translates all points by negative vector (returns new polyline)
    Polyline operator-(const Vector& v) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    void transform();
    Polyline transformed() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON-serializable object
    nlohmann::ordered_json jsondump() const;

    /// Create polyline from JSON data
    static Polyline jsonload(const nlohmann::json& data);

    /// Serialize to JSON file

    /// Deserialize from JSON file

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Geometric Utilities
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Shift polyline points by specified number of positions
    void shift(int times);

    /// Calculate squared length of polyline (faster, no sqrt)
    double length_squared() const;

    /// Get point at parameter t along a line segment (t=0 is start, t=1 is end)
    static Point point_at_parameter(const Point& start, const Point& end, double t);

    /// Find closest point on line segment to given point, returns parameter t
    static void closest_point_to_line(const Point& point, const Point& line_start, 
                                     const Point& line_end, double& t);

    /// Check if two line segments overlap and return the overlapping segment
    static bool line_line_overlap(const Point& line0_start, const Point& line0_end,
                                 const Point& line1_start, const Point& line1_end,
                                 Point& overlap_start, Point& overlap_end);

    /// Calculate average of two line segments
    static void line_line_average(const Point& line0_start, const Point& line0_end,
                                 const Point& line1_start, const Point& line1_end,
                                 Point& output_start, Point& output_end);

    /// Calculate overlap average of two line segments
    static void line_line_overlap_average(const Point& line0_start, const Point& line0_end,
                                         const Point& line1_start, const Point& line1_end,
                                         Point& output_start, Point& output_end);

    /// Create line from projected points onto a base line
    static bool line_from_projected_points(const Point& line_start, const Point& line_end,
                                          const std::vector<Point>& points,
                                          Point& output_start, Point& output_end);

    /// Find closest distance and point from a point to this polyline
    double closest_distance_and_point(const Point& point, size_t& edge_id, Point& closest_point) const;

    /// Check if polyline is closed (first and last points are the same)
    bool is_closed() const;

    /// Calculate center point of polyline
    Point center() const;

    /// Calculate center as vector
    Vector center_vec() const;

    /// Get average plane from polyline points
    void get_average_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) const;

    /// Get fast plane calculation from polyline
    void get_fast_plane(Point& origin, Plane& pln) const;

    /// Calculate middle line between two line segments
    static void get_middle_line(const Point& line0_start, const Point& line0_end,
                               const Point& line1_start, const Point& line1_end,
                               Point& output_start, Point& output_end);

    /// Extend line segment by specified distances at both ends
    static void extend_line(Point& line_start, Point& line_end, 
                           double distance0, double distance1);

    /// Scale line segment inward by specified distance
    static void scale_line(Point& line_start, Point& line_end, double distance);

    /// Extend polyline segment
    void extend_segment(int segment_id, double dist0, double dist1, 
                       double proportion0 = 0.0, double proportion1 = 0.0);

    /// Extend segment equally on both ends (static utility)
    static void extend_segment_equally(Point& segment_start, Point& segment_end, 
                                      double dist, double proportion = 0.0);

    /// Extend polyline segment equally
    void extend_segment_equally(int segment_id, double dist, double proportion = 0.0);

    /// Move polyline by direction vector
    void move(const Vector& direction);

    /// Check if polyline is clockwise oriented
    bool is_clockwise(const Plane& pln) const;

    /// Flip polyline direction (reverse point order)
    void flip();

    /// Get convex/concave corners of polyline
    void get_convex_corners(std::vector<bool>& convex_or_concave) const;

    /// Interpolate between two polylines
    static Polyline tween_two_polylines(const Polyline& polyline0, const Polyline& polyline1, 
                                       double weight);

private:
    /// Helper to recompute plane when points change
    void recompute_plane_if_needed();

    /// Calculate average normal from polyline points
    void average_normal(Vector& average_normal) const;
};

///////////////////////////////////////////////////////////////////////////////////////////
// Stream operator
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream& operator<<(std::ostream& os, const Polyline& polyline);

} // namespace session_cpp

// fmt formatter specialization for Polyline
template <> struct fmt::formatter<session_cpp::Polyline> {
    constexpr auto parse(fmt::format_parse_context& ctx) { return ctx.begin(); }

    auto format(const session_cpp::Polyline& polyline, fmt::format_context& ctx) const {
        return fmt::format_to(ctx.out(), "Polyline(guid={}, name={}, points={})",
                            polyline.guid, polyline.name, polyline.points.size());
    }
};
