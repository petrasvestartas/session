#include "load_point.hpp"
#include <sstream>

namespace geo {

LoadPoint::LoadPoint() : p(), l() {}

LoadPoint::LoadPoint(const Point& point, const Vector& load) : p(point), l(load) {}

// Accessor methods removed since we now have public data members

std::string LoadPoint::to_string() const {
    std::ostringstream oss;
    oss << "LoadPoint at (" << p[0] << ", " << p[1] << ", " << p[2] << ") "
        << "with load [" << l[0] << ", " << l[1] << ", " << l[2] << "]";
    return oss.str();
}

}  // namespace geo

// Implementation of the stream insertion operator
std::ostream& geo::operator<<(std::ostream& os, const geo::LoadPoint& load_point) {
    os << "LoadPoint at (" << load_point.p.x() << ", " << load_point.p.y() << ", " << load_point.p.z() << ") "
       << "with load [" << load_point.l[0] << ", " << load_point.l[1] << ", " << load_point.l[2] << "] kN";
    return os;
}
