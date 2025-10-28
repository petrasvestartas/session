#include "polyline.h"

namespace session_cpp {

Polyline::Polyline() : guid(::guid()), name("my_polyline"), plane(Plane()) {}

Polyline::Polyline(const std::vector<Point>& pts) 
    : guid(::guid()), name("my_polyline"), points(pts) {
    recompute_plane_if_needed();
}

size_t Polyline::len() const {
    return points.size();
}

bool Polyline::is_empty() const {
    return points.empty();
}

size_t Polyline::segment_count() const {
    return points.size() > 1 ? points.size() - 1 : 0;
}

double Polyline::length() const {
    double total_length = 0.0;
    for (size_t i = 0; i < segment_count(); i++) {
        Vector segment_vector = points[i + 1] - points[i];
        total_length += segment_vector.magnitude();
    }
    return total_length;
}

Point* Polyline::get_point(size_t index) {
    if (index < points.size()) {
        return &points[index];
    }
    return nullptr;
}

const Point* Polyline::get_point(size_t index) const {
    if (index < points.size()) {
        return &points[index];
    }
    return nullptr;
}

void Polyline::add_point(const Point& point) {
    points.push_back(point);
    if (points.size() == 3) {
        recompute_plane_if_needed();
    }
}

void Polyline::insert_point(size_t index, const Point& point) {
    if (index <= points.size()) {
        points.insert(points.begin() + index, point);
        if (points.size() == 3) {
            recompute_plane_if_needed();
        }
    }
}

bool Polyline::remove_point(size_t index, Point& out_point) {
    if (index < points.size()) {
        out_point = points[index];
        points.erase(points.begin() + index);
        if (points.size() == 3) {
            recompute_plane_if_needed();
        }
        return true;
    }
    return false;
}

void Polyline::reverse() {
    std::reverse(points.begin(), points.end());
    plane.reverse();
}

Polyline Polyline::reversed() const {
    Polyline result = *this;
    result.reverse();
    return result;
}

void Polyline::recompute_plane_if_needed() {
    if (points.size() >= 3) {
        plane = Plane::from_points(points);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

Polyline& Polyline::operator+=(const Vector& v) {
    for (auto& p : points) {
        p += v;
    }
    // Update plane origin
    Point new_origin = plane.origin() + v;
    Vector x = plane.x_axis();
    Vector y = plane.y_axis();
    plane = Plane(new_origin, x, y);
    return *this;
}

Polyline Polyline::operator+(const Vector& v) const {
    Polyline result = *this;
    result += v;
    return result;
}

Polyline& Polyline::operator-=(const Vector& v) {
    for (auto& p : points) {
        p -= v;
    }
    // Update plane origin
    Point new_origin = plane.origin() - v;
    Vector x = plane.x_axis();
    Vector y = plane.y_axis();
    plane = Plane(new_origin, x, y);
    return *this;
}

Polyline Polyline::operator-(const Vector& v) const {
    Polyline result = *this;
    result -= v;
    return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void Polyline::transform() {
  for (auto& pt : points) {
    xform.transform_point(pt);
  }
  xform = Xform::identity();
}

Polyline Polyline::transformed() const {
  Polyline result = *this;
  result.transform();
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Polyline::jsondump() const {
    nlohmann::ordered_json j;
    j["type"] = "Polyline";
    j["guid"] = guid;
    j["name"] = name;
    j["points"] = nlohmann::json::array();
    for (const auto& pt : points) {
        j["points"].push_back(pt.jsondump());
    }
    j["plane"] = plane.jsondump();
    j["width"] = width;
    j["linecolor"] = linecolor.jsondump();
    j["xform"] = xform.jsondump();
    return j;
}

Polyline Polyline::jsonload(const nlohmann::json& data) {
    Polyline polyline;
    polyline.guid = data["guid"];
    polyline.name = data["name"];
    
    for (const auto& pt_json : data["points"]) {
        polyline.points.push_back(Point::jsonload(pt_json));
    }
    
    polyline.plane = Plane::jsonload(data["plane"]);
    
    if (data.contains("width")) {
        polyline.width = data["width"];
    }
    if (data.contains("linecolor")) {
        polyline.linecolor = Color::jsonload(data["linecolor"]);
    }
    return polyline;
}



///////////////////////////////////////////////////////////////////////////////////////////
// Geometric Utilities
///////////////////////////////////////////////////////////////////////////////////////////

void Polyline::shift(int times) {
    if (points.empty()) return;
    
    // Remove last point if closed
    bool was_closed = is_closed();
    if (was_closed) {
        points.pop_back();
    }
    
    // Perform rotation
    if (!points.empty() && times != 0) {
        size_t n = points.size();
        times = times % static_cast<int>(n);
        if (times < 0) times += static_cast<int>(n);
        
        std::rotate(points.begin(), points.begin() + times, points.end());
    }
    
    // Restore closure if it was closed
    if (was_closed && !points.empty()) {
        points.push_back(points[0]);
    }
}

double Polyline::length_squared() const {
    double length = 0.0;
    for (size_t i = 0; i < segment_count(); i++) {
        Vector segment = points[i + 1] - points[i];
        length += segment.length_squared();
    }
    return length;
}

Point Polyline::point_at_parameter(const Point& start, const Point& end, double t) {
    const double s = 1.0 - t;
    return Point(
        (start.x() == end.x()) ? start.x() : static_cast<double>(s * start.x() + t * end.x()),
        (start.y() == end.y()) ? start.y() : static_cast<double>(s * start.y() + t * end.y()),
        (start.z() == end.z()) ? start.z() : static_cast<double>(s * start.z() + t * end.z())
    );
}

void Polyline::closest_point_to_line(const Point& point, const Point& line_start, 
                                    const Point& line_end, double& t) {
    Vector D = line_end - line_start;
    double DoD = D.length_squared();

    if (DoD > 0.0) {
        Vector to_point_start = point - line_start;
        Vector to_point_end = point - line_end;
        
        if (to_point_start.length_squared() <= to_point_end.length_squared()) {
            t = to_point_start.dot(D) / DoD;
        } else {
            t = 1.0 + to_point_end.dot(D) / DoD;
        }
    } else {
        t = 0.0;
    }
}

bool Polyline::line_line_overlap(const Point& line0_start, const Point& line0_end,
                                const Point& line1_start, const Point& line1_end,
                                Point& overlap_start, Point& overlap_end) {
    double t[4];
    t[0] = 0.0;
    t[1] = 1.0;

    closest_point_to_line(line1_start, line0_start, line0_end, t[2]);
    closest_point_to_line(line1_end, line0_start, line0_end, t[3]);

    // Check if there is an overlap
    bool do_overlap = !((t[2] < 0 && t[3] < 0) || (t[2] > 1 && t[3] > 1));

    // Sort parameters
    std::sort(t, t + 4);

    // Check if the overlap is not just a point
    do_overlap = do_overlap && (std::abs(t[2] - t[1]) > Tolerance::ZERO_TOLERANCE);

    // Get overlap points
    overlap_start = point_at_parameter(line0_start, line0_end, t[1]);
    overlap_end = point_at_parameter(line0_start, line0_end, t[2]);

    return do_overlap;
}

void Polyline::line_line_average(const Point& line0_start, const Point& line0_end,
                                const Point& line1_start, const Point& line1_end,
                                Point& output_start, Point& output_end) {
    output_start = Point(
        (line0_start.x() + line1_start.x()) * 0.5,
        (line0_start.y() + line1_start.y()) * 0.5,
        (line0_start.z() + line1_start.z()) * 0.5
    );
    
    output_end = Point(
        (line0_end.x() + line1_end.x()) * 0.5,
        (line0_end.y() + line1_end.y()) * 0.5,
        (line0_end.z() + line1_end.z()) * 0.5
    );
}

void Polyline::line_line_overlap_average(const Point& line0_start, const Point& line0_end,
                                        const Point& line1_start, const Point& line1_end,
                                        Point& output_start, Point& output_end) {
    // Get two overlaps
    Point lineA_start, lineA_end;
    line_line_overlap(line0_start, line0_end, line1_start, line1_end, lineA_start, lineA_end);
    
    Point lineB_start, lineB_end;
    line_line_overlap(line1_start, line1_end, line0_start, line0_end, lineB_start, lineB_end);

    // Construct middle lines, in case the first one is flipped
    Point mid_line0_start(
        (lineA_start.x() + lineB_start.x()) * 0.5,
        (lineA_start.y() + lineB_start.y()) * 0.5,
        (lineA_start.z() + lineB_start.z()) * 0.5
    );
    Point mid_line0_end(
        (lineA_end.x() + lineB_end.x()) * 0.5,
        (lineA_end.y() + lineB_end.y()) * 0.5,
        (lineA_end.z() + lineB_end.z()) * 0.5
    );
    
    Point mid_line1_start(
        (lineA_start.x() + lineB_end.x()) * 0.5,
        (lineA_start.y() + lineB_end.y()) * 0.5,
        (lineA_start.z() + lineB_end.z()) * 0.5
    );
    Point mid_line1_end(
        (lineA_end.x() + lineB_start.x()) * 0.5,
        (lineA_end.y() + lineB_start.y()) * 0.5,
        (lineA_end.z() + lineB_start.z()) * 0.5
    );

    // The diagonal is always longer, so return the longer
    Vector mid0_vec = mid_line0_end - mid_line0_start;
    Vector mid1_vec = mid_line1_end - mid_line1_start;
    
    if (mid0_vec.length_squared() > mid1_vec.length_squared()) {
        output_start = mid_line0_start;
        output_end = mid_line0_end;
    } else {
        output_start = mid_line1_start;
        output_end = mid_line1_end;
    }
}

bool Polyline::line_from_projected_points(const Point& line_start, const Point& line_end,
                                         const std::vector<Point>& points,
                                         Point& output_start, Point& output_end) {
    if (points.empty()) return false;
    
    std::vector<double> t_values;
    t_values.reserve(points.size());

    // Project all points to the line
    for (const auto& point : points) {
        double t;
        closest_point_to_line(point, line_start, line_end, t);
        t_values.push_back(t);
    }

    // Sort parameters
    std::sort(t_values.begin(), t_values.end());

    // Output first and last points
    output_start = point_at_parameter(line_start, line_end, t_values.front());
    output_end = point_at_parameter(line_start, line_end, t_values.back());

    // Check if not just a point
    return std::abs(t_values.front() - t_values.back()) > Tolerance::ZERO_TOLERANCE;
}

double Polyline::closest_distance_and_point(const Point& point, size_t& edge_id, Point& closest_point) const {
    edge_id = 0;
    double closest_distance = std::numeric_limits<double>::max();
    double best_t = 0.0;

    for (size_t i = 0; i < segment_count(); i++) {
        double t;
        closest_point_to_line(point, points[i], points[i + 1], t);
        
        Point point_on_segment = point_at_parameter(points[i], points[i + 1], t);
        double distance = point.distance(point_on_segment);

        if (distance < closest_distance) {
            closest_distance = distance;
            edge_id = i;
            best_t = t;
        }

        if (closest_distance < Tolerance::ZERO_TOLERANCE) {
            break;
        }
    }

    closest_point = point_at_parameter(points[edge_id], points[edge_id + 1], best_t);
    return closest_distance;
}

bool Polyline::is_closed() const {
    if (points.size() < 2) return false;
    return points.front().distance(points.back()) < static_cast<double>(Tolerance::ZERO_TOLERANCE);
}

Point Polyline::center() const {
    if (points.empty()) return Point(0, 0, 0);
    
    double x = 0, y = 0, z = 0;
    size_t n = is_closed() ? points.size() - 1 : points.size();

    for (size_t i = 0; i < n; i++) {
        x += points[i].x();
        y += points[i].y();
        z += points[i].z();
    }
    
    x /= n;
    y /= n;
    z /= n;

    return Point(static_cast<double>(x), static_cast<double>(y), static_cast<double>(z));
}

Vector Polyline::center_vec() const {
    Point c = center();
    return Vector(c.x(), c.y(), c.z());
}

void Polyline::get_average_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) const {
    // Origin
    origin = center();

    // X-axis (first segment direction)
    if (points.size() >= 2) {
        x_axis = points[1] - points[0];
        x_axis.normalize_self();
    } else {
        x_axis = Vector(1, 0, 0);
    }

    // Z-axis (average normal)
    average_normal(z_axis);

    // Y-axis (cross product)
    y_axis = z_axis.cross(x_axis);
    y_axis.normalize_self();
}

void Polyline::get_fast_plane(Point& origin, Plane& pln) const {
    if (points.empty()) {
        origin = Point(0, 0, 0);
        pln = Plane();
        return;
    }

    origin = points[0];
    
    Vector normal;
    average_normal(normal);
    
    // Create plane from point and normal
    pln = Plane::from_point_normal(origin, normal);
}

void Polyline::get_middle_line(const Point& line0_start, const Point& line0_end,
                              const Point& line1_start, const Point& line1_end,
                              Point& output_start, Point& output_end) {
    output_start = Point(
        (line0_start.x() + line1_start.x()) * 0.5,
        (line0_start.y() + line1_start.y()) * 0.5,
        (line0_start.z() + line1_start.z()) * 0.5
    );

    output_end = Point(
        (line0_end.x() + line1_end.x()) * 0.5,
        (line0_end.y() + line1_end.y()) * 0.5,
        (line0_end.z() + line1_end.z()) * 0.5
    );
}

void Polyline::extend_line(Point& line_start, Point& line_end, double distance0, double distance1) {
    Vector v = line_end - line_start;
    v.normalize_self();

    line_start = line_start - v * static_cast<double>(distance0);
    line_end = line_end + v * static_cast<double>(distance1);
}

void Polyline::scale_line(Point& line_start, Point& line_end, double distance) {
    Vector v = line_end - line_start;
    Point p0 = line_start + v * static_cast<double>(distance);
    Point p1 = line_end - v * static_cast<double>(distance);
    line_start = p0;
    line_end = p1;
}

void Polyline::extend_segment(int segment_id, double dist0, double dist1, 
                             double proportion0, double proportion1) {
    if (segment_id < 0 || segment_id >= static_cast<int>(segment_count())) return;
    if (dist0 == 0 && dist1 == 0 && proportion0 == 0 && proportion1 == 0) return;

    Point p0 = points[segment_id];
    Point p1 = points[segment_id + 1];
    Vector v = p1 - p0;

    if (proportion0 != 0 || proportion1 != 0) {
        p0 = p0 - v * static_cast<double>(proportion0);
        p1 = p1 + v * static_cast<double>(proportion1);
    } else {
        v.normalize_self();
        p0 = p0 - v * static_cast<double>(dist0);
        p1 = p1 + v * static_cast<double>(dist1);
    }

    points[segment_id] = p0;
    points[segment_id + 1] = p1;

    // Handle closed polylines
    if (is_closed()) {
        if (segment_id == 0) {
            points[points.size() - 1] = points[0];
        } else if ((segment_id + 1) == static_cast<int>(points.size() - 1)) {
            points[0] = points[points.size() - 1];
        }
    }
}

void Polyline::extend_segment_equally(Point& segment_start, Point& segment_end, double dist, double proportion) {
    if (dist == 0 && proportion == 0) return;

    Vector v = segment_end - segment_start;

    if (proportion != 0) {
        segment_start = segment_start - v * static_cast<double>(proportion);
        segment_end = segment_end + v * static_cast<double>(proportion);
    } else {
        v.normalize_self();
        segment_start = segment_start - v * static_cast<double>(dist);
        segment_end = segment_end + v * static_cast<double>(dist);
    }
}

void Polyline::extend_segment_equally(int segment_id, double dist, double proportion) {
    if (segment_id < 0 || segment_id >= static_cast<int>(segment_count())) return;
    
    extend_segment_equally(points[segment_id], points[segment_id + 1], dist, proportion);

    // Handle closed polylines
    if (points.size() > 2 && is_closed()) {
        if (segment_id == 0) {
            points[points.size() - 1] = points[0];
        } else if ((segment_id + 1) == static_cast<int>(points.size() - 1)) {
            points[0] = points[points.size() - 1];
        }
    }
}

void Polyline::move(const Vector& direction) {
    for (auto& point : points) {
        point += direction;
    }
}

bool Polyline::is_clockwise(const Plane& pln) const {
    (void)pln;  // Reserved for future use - may project to plane
    if (points.size() < 3) return false;

    // Create a copy for transformation
    Polyline cp = *this;

    // Ensure closed for winding calculation
    if (!cp.is_closed()) {
        cp.points.push_back(cp.points[0]);
    }

    // Calculate signed area (shoelace formula)
    double signed_area = 0.0;
    for (size_t i = 0; i < cp.points.size() - 1; i++) {
        signed_area += (cp.points[i + 1].x() - cp.points[i].x()) * 
                      (cp.points[i + 1].y() + cp.points[i].y());
    }

    return signed_area > 0;
}

void Polyline::flip() {
    std::reverse(points.begin(), points.end());
}

void Polyline::get_convex_corners(std::vector<bool>& convex_or_concave) const {
    if (points.size() < 3) return;

    bool closed = is_closed();
    size_t n = closed ? points.size() - 1 : points.size();
    
    Vector normal;
    average_normal(normal);
    
    convex_or_concave.clear();
    convex_or_concave.reserve(n);

    for (size_t current = 0; current < n; current++) {
        size_t prev = (current == 0) ? n - 1 : current - 1;
        size_t next = (current == n - 1) ? 0 : current + 1;

        Vector dir0 = points[current] - points[prev];
        dir0.normalize_self();

        Vector dir1 = points[next] - points[current];
        dir1.normalize_self();

        Vector cross = dir0.cross(dir1);
        cross.normalize_self();

        double dot = cross.dot(normal);
        bool is_convex = !(dot < 0.0);
        convex_or_concave.push_back(is_convex);
    }
}

Polyline Polyline::tween_two_polylines(const Polyline& polyline0, const Polyline& polyline1, double weight) {
    if (polyline0.points.size() != polyline1.points.size()) {
        // Return first polyline if sizes don't match
        return polyline0;
    }

    Polyline result;
    result.points.reserve(polyline0.points.size());

    for (size_t i = 0; i < polyline0.points.size(); i++) {
        Vector diff = polyline1.points[i] - polyline0.points[i];
        Point interpolated = polyline0.points[i] + diff * static_cast<double>(weight);
        result.points.push_back(interpolated);
    }

    return result;
}

void Polyline::average_normal(Vector& average_normal) const {
    size_t len = points.size();
    if (len < 3) {
        average_normal = Vector(0, 0, 1);
        return;
    }

    // Check if closed
    bool closed = (points.front().distance(points.back()) < static_cast<double>(Tolerance::ZERO_TOLERANCE));
    if (closed && len > 1) len = len - 1;

    average_normal = Vector(0, 0, 0);

    for (size_t i = 0; i < len; i++) {
        size_t prev = (i == 0) ? len - 1 : i - 1;
        size_t next = (i + 1) % len;
        
        Vector v1 = points[i] - points[prev];
        Vector v2 = points[next] - points[i];
        Vector cross = v1.cross(v2);
        average_normal += cross;
    }
    average_normal.normalize_self();
}

///////////////////////////////////////////////////////////////////////////////////////////
// Stream operator
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream& operator<<(std::ostream& os, const Polyline& polyline) {
    return os << fmt::format("Polyline(guid={}, name={}, points={})",
                            polyline.guid, polyline.name, polyline.points.size());
}

} // namespace session_cpp
