// #include "point.hpp"

// namespace geo {
// Point::Point() : _xyz{0.0, 0.0, 0.0} {}

// Point::Point(double x, double y, double z) : _xyz{x, y, z} {}

// Vector Point::to_vector() { return Vector(_xyz[0], _xyz[1], _xyz[2]); }

// double& Point::operator[](int index) { return _xyz[index]; }

// const double& Point::operator[](int index) const { return _xyz[index]; }

// Point& Point::operator*=(double factor) {
//     for (size_t i = 0; i < 3; ++i) _xyz[i] *= factor;
//     return *this;
// }

// Point& Point::operator/=(double factor) {
//     for (size_t i = 0; i < 3; ++i) _xyz[i] /= factor;
//     return *this;
// }

// Point& Point::operator+=(const Point& other) {
//     for (size_t i = 0; i < 3; ++i) _xyz[i] += other._xyz[i];
//     return *this;
// }

// Point& Point::operator-=(const Point& other) {
//     for (size_t i = 0; i < 3; ++i) _xyz[i] -= other._xyz[i];
//     return *this;
// }

// Point Point::operator*(double factor) const {
//     Point result = *this;
//     result *= factor;
//     return result;
// }

// Point Point::operator/(double factor) const {
//     Point result = *this;
//     result /= factor;
//     return result;
// }

// Point Point::operator+(const Vector& other) const {
//     Point result(_xyz[0] + other[0], _xyz[1] + other[1], _xyz[2] + other[2]);
//     return result;
// }

// Vector Point::operator-(const Vector& other) const {
//     Vector v(_xyz[0] - other[0], _xyz[1] - other[1], _xyz[2] - other[2]);
//     return v;
// }

// bool Point::operator==(const Point& other) const {
//     // Replace x, y, z with your actual member variables
//     return _xyz[0] == other[0] && _xyz[1] == other[1] && _xyz[2] == other[2];
// }

// bool Point::operator!=(const Point& other) const { return !(*this == other); }

// double Point::x() const { return _xyz[0]; }

// double Point::y() const { return _xyz[1]; }

// double Point::z() const { return _xyz[2]; }



// float Point::ccw(Point& a, Point& b, Point& c) {
//     return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
// }

// Point Point::mid_point(Point& p) {
//     return Point((_xyz[0] + p[0]) * 0.5, (_xyz[1] + p[1]) * 0.5, (_xyz[2] + p[2]) * 0.5);
// }

// double Point::distance(Point& p) {
//     double x = abs(_xyz[0] - p[0]);
//     double y = abs(_xyz[1] - p[1]);
//     double z = abs(_xyz[2] - p[2]);
//     double length = 0;

//     if (y >= x && y >= z) {
//         length = x;
//         x = y;
//         y = length;
//     } else if (z >= x && z >= y) {
//         length = x;
//         x = z;
//         z = length;
//     }

//     // For small denormalized doubles (positive but smaller
//     // than DOUBLE_MIN), some compilers/FPUs set 1.0/x to +INF.
//     // Without the DOUBLE_MIN test we end up with
//     // microscopic Points that have infinte length!
//     if (x > GLOBALS::DOUBLE_MIN) {
//         y /= x;
//         z /= x;
//         length = x * sqrt(1.0 + y * y + z * z);
//     } else if (x > 0.0 && GLOBALS::IS_FINITE(x))
//         length = x;
//     else
//         length = 0.0;

//     return length;
// }

// // bool is_left_of(const IK::Point_3 &a, const IK::Point_3 &b)
// // {
// //     return (a.x() < b.x() || (a.x() == b.x() && a.y() < b.y()));
// // }

// // float len(const IK::Point_3 &a, const IK::Point_3 &b)
// // {
// //     return std::sqrt((b.x() - a.x()) * (b.x() - a.x()) + (b.y() - a.y()) * (b.y() - a.y()));
// // }

// // float dist(const IK::Point_3 &a, const IK::Point_3 &b, const IK::Point_3 &p)
// // {
// //     return std::fabs((b.x() - a.x()) * (a.y() - p.y()) - (b.y() - a.y()) * (a.x() - p.x())) /
// //     len(a, b);
// // }

// // size_t get_farthest(const IK::Point_3 &a, const IK::Point_3 &b, const std::Point<IK::Point_3> &v)
// // {
// //     size_t idxMax = 0;
// //     float distMax = dist(a, b, v[idxMax]);

// //     for (size_t i = 1; i < v.size(); ++i)
// //     {
// //         float distCurr = dist(a, b, v[i]);
// //         if (distCurr > distMax)
// //         {
// //             idxMax = i;
// //             distMax = distCurr;
// //         }
// //     }

// //     return idxMax;
// // }

// // void quick_hull(const std::Point<IK::Point_3> &v, const IK::Point_3 &a, const IK::Point_3 &b,
// // std::Point<IK::Point_3> &hull)
// // {
// //     if (v.empty())
// //         return;

// //     IK::Point_3 f = v[get_farthest(a, b, v)];

// //     // Collect points to the left of segment (a, f)
// //     std::Point<IK::Point_3> left;
// //     for (auto p : v)
// //     {
// //         if (ccw(a, f, p) > 0)
// //         {
// //             left.emplace_back(p);
// //         }
// //     }
// //     quick_hull(left, a, f, hull);

// //     // Add f to the hull
// //     hull.emplace_back(f);

// //     // Collect points to the left of segment (f, b)
// //     std::Point<IK::Point_3> right;
// //     for (auto p : v)
// //     {
// //         if (ccw(f, b, p) > 0)
// //         {
// //             right.emplace_back(p);
// //         }
// //     }
// //     quick_hull(right, f, b, hull);
// // }

// // IK::Point_3 rotate_to_xaxis(IK::Point_3 &point, const double &angle)
// // {
// //     return IK::Point_3(point.hx() * std::cos(angle) - point.hy() * std::sin(angle), point.hx() *
// //     std::sin(angle) + point.hy() * std::cos(angle), 0);
// // }

// double Point::area(std::vector<Point>& polygon) {
//     size_t n = polygon.size();
//     double area = 0.0;
//     for (size_t i = 0; i < n; ++i) {
//         size_t j = (i + 1) % n;  // next vertex index, wrapping around
//         area += polygon[i][0] * polygon[j][1];
//         area -= polygon[j][0] * polygon[i][1];
//     }
//     return std::fabs(area) / 2.0;
// }

// void Point::scale(double factor) {
//     _xyz[0] *= factor;
//     _xyz[1] *= factor;
//     _xyz[2] *= factor;
// }

// Point Point::scaled(double factor) {
//     return Point(_xyz[0] + factor, _xyz[1] + factor, _xyz[2] + factor);
// }

// void Point::translate(Vector& translation_vector) {
//     _xyz[0] += translation_vector[0];
//     _xyz[1] += translation_vector[1];
//     _xyz[2] += translation_vector[2];
// }

// Point Point::translated(Vector& translation_vector) {
//     return Point(_xyz[0] + translation_vector[0], _xyz[1] + translation_vector[1],
//                  _xyz[2] + translation_vector[2]);
// }

// Point Point::centroid_quad(const std::vector<Point>& vertices) {
//     if (vertices.size() != 4) {
//         throw std::invalid_argument("Polygon must have exactly 4 vertices.");
//     }

//     double totalArea = 0;
//     Point centroidSum(0, 0, 0);

//     for (int i = 0; i < 4; ++i) {
//         const Point& p0 = vertices[i];
//         const Point& p1 = vertices[(i + 1) % 4];
//         const Point& p2 = vertices[(i + 2) % 4];

//         // Triangle area using the shoelace formula
//         double triangleArea =
//             std::abs(p0[0] * (p1[1] - p2[1]) + p1[0] * (p2[1] - p0[1]) + p2[0] * (p0[0] - p1[0])) /
//             2.0;
//         totalArea += triangleArea;

//         // Triangle centroid
//         Point triangleCentroid((p0[0] + p1[0] + p2[0]) / 3.0, (p0[1] + p1[1] + p2[1]) / 3.0,
//                                (p0[2] + p1[2] + p2[2]) / 3.0);
//         centroidSum += triangleCentroid * triangleArea;
//     }

//     return centroidSum / totalArea;
// }

// // Function to calculate area of the quadrilateral using Shoelace formula
// double Point::area_quad(const std::vector<Point>& vertices) {
//     if (vertices.size() != 4) {
//         throw std::invalid_argument("Polygon must have exactly 4 vertices.");
//     }

//     double area = 0.0;
//     for (size_t i = 0; i < 4; ++i) {
//         const Point& current = vertices[i];
//         const Point& next = vertices[(i + 1) % 4];
//         area += current[0] * next[1] - next[0] * current[1];
//     }
//     return std::abs(area) / 2.0;
// }

// // Convert matrix to string
// std::string Point::to_string() const {
//     std::ostringstream oss;
//     oss << "Point: ";
//     oss << _xyz[0] << " " << _xyz[1] << " " << _xyz[2];
//     return oss.str();
// }

// }  // namespace geo

// // Implementation of the stream insertion operator
// std::ostream& geo::operator<<(std::ostream& os, const geo::Point& point) {
//     os << "Point: " << point.x() << " " << point.y() << " " << point.z();
//     return os;
// }