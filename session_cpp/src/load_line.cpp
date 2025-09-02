#include "load_line.hpp"
#include <sstream>

namespace geo {

LoadLine::LoadLine() : p0(), p1(), l0(), l1() {}

LoadLine::LoadLine(const Point& point0, const Point& point1, const Vector& load_start, const Vector& load_end)
    : p0(point0), p1(point1), l0(load_start), l1(load_end) {}

LoadLine::LoadLine(const Line& line, const Vector& load_start, const Vector& load_end)
    : p0(line[0]), p1(line[1]), l0(load_start), l1(load_end) {}

// No accessor methods needed with the new direct access design

std::string LoadLine::to_string() const {
    std::ostringstream oss;
    oss << "LoadLine from (" << p0[0] << ", " << p0[1] << ", " << p0[2] << ") "
        << "to (" << p1[0] << ", " << p1[1] << ", " << p1[2] << ") "
        << "with loads start [" << l0[0] << ", " << l0[1] << ", " << l0[2] << "] "
        << "end [" << l1[0] << ", " << l1[1] << ", " << l1[2] << "]";
    return oss.str();
}

}  // namespace geo

// Implementation of the stream insertion operator
std::ostream& geo::operator<<(std::ostream& os, const geo::LoadLine& load_line) {
    os << "LoadLine from (" << load_line.p0[0] << ", " << load_line.p0[1] << ", " << load_line.p0[2] << ") "
       << "to (" << load_line.p1[0] << ", " << load_line.p1[1] << ", " << load_line.p1[2] << ") "
       << "with loads start [" << load_line.l0[0] << ", " << load_line.l0[1] << ", " << load_line.l0[2] << "] kN"
       << "end [" << load_line.l1[0] << ", " << load_line.l1[1] << ", " << load_line.l1[2] << "] kN";
    return os;
}
