#include "line.hpp"

#include <cmath>  // for std::sqrt

namespace geo {
Line::Line() : points{Point(), Point()} {}

Line::Line(const Point& p1, const Point& p2) : points{p1, p2} {}

Line::Line(const double& x0, const double& y0, const double& z0, const double& x1, const double& y1,
           const double& z1) {
    points[0] = Point(x0, y0, z0);
    points[1] = Point(x1, y1, z1);
}



Point& Line::operator[](int index) { return points[index]; }

const Point& Line::operator[](int index) const { return points[index]; }

double Line::length() const {
    double dx = points[1][0] - points[0][0];
    double dy = points[1][1] - points[0][1];
    double dz = points[1][2] - points[0][2];
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

double Line::squared_length() const {
    double dx = points[1][0] - points[0][0];
    double dy = points[1][1] - points[0][1];
    double dz = points[1][2] - points[0][2];
    return dx * dx + dy * dy + dz * dz;
}

Vector Line::to_vector() const {
    return Vector(points[1][0] - points[0][0], points[1][1] - points[0][1],
                  points[1][2] - points[0][2]);
}

Point point_at(const Line& line, const double& t) {
    const double s = 1.0 - t;

    return Point((line[0][0] == line[1][0]) ? line[0][0] : s * line[0][0] + t * line[1][0],
                 (line[0][1] == line[1][1]) ? line[0][1] : s * line[0][1] + t * line[1][1],
                 (line[0][2] == line[1][2]) ? line[0][2] : s * line[0][2] + t * line[1][2]);
}

void Line::scale(double factor) {
    points[0][0] *= factor;
    points[0][1] *= factor;
    points[0][2] *= factor;
    points[1][0] *= factor;
    points[1][1] *= factor;
    points[1][2] *= factor;
}

Line Line::scaled(double factor) {
    return Line(points[0][0] + factor, points[0][1] + factor, points[0][2] + factor,
                points[1][0] + factor, points[1][1] + factor, points[1][2] + factor);
}

void Line::translate(Vector& translation_vector) {
    points[0][0] += translation_vector[0];
    points[0][1] += translation_vector[1];
    points[0][2] += translation_vector[2];
    points[1][0] += translation_vector[0];
    points[1][1] += translation_vector[1];
    points[1][2] += translation_vector[2];
}

Line Line::translated(Vector& translation_vector) {
    return Line(points[0][0] + translation_vector[0], points[0][1] + translation_vector[1],
                points[0][2] + translation_vector[2], points[1][0] + translation_vector[0],
                points[1][1] + translation_vector[1], points[1][2] + translation_vector[2]);
}

// bool closest_point_to(const Point &point, double &t)
// {
//     bool rc = false;

//     const IK::Vector_3 D = this.to_vector();
//     const double DoD = D.squared_length();

//     if (DoD > 0.0)
//     {
//         if ((point - line[0]).squared_length() <= (point - line[1]).squared_length())
//             t = ((point - line[0]) * D) / DoD;
//         else
//             t = 1.0 + ((point - line[1]) * D) / DoD;

//         rc = true;
//     }
//     else
//     { // (GBA) Closest point to a degenerate line works as well
//         t = 0.0;
//         rc = true;
//     }

//     return rc;
// }

// double angle_to_xaxis(IK::Segment_3 &edge)
// {

//     auto delta = edge[0] - edge[1];
//     return -std::atan(delta.hy() / delta.hx());
// }

}  // namespace geo