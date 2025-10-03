#pragma once
#include "guid.h"
#include <iomanip>
#include <random>
#include <sstream>
#include <string>
#include <cmath>

namespace geo {

struct GLOBALS {
public:

  static bool IS_FINITE(double x) {
    return 0x7FF0 != (*((unsigned short *)(&x) + 3) & 0x7FF0);
  }

  // Float overload to avoid implicit promotions when working with f32 APIs
  static bool IS_FINITE(float x) {
    return std::isfinite(x);
  }

  // Mathematical constants
  static constexpr double PI = 3.14159265358979323846;
  static constexpr double TO_DEGREES = 57.295779513082320876798154814105;
  static constexpr double TO_RADIANS = 0.01745329251994329576923690768489;

  // Float-precision equivalents (for f32-based APIs)
  static constexpr float PI_F = 3.14159265358979323846f;
  static constexpr float TO_DEGREES_F = 57.29577951308232f;
  static constexpr float TO_RADIANS_F = 0.01745329251994329577f;

  // Tolerance values
  static constexpr double ZERO_TOLERANCE = 2.3283064365386962890625e-10;
  static constexpr float ZERO_TOLERANCE_F = 2.3283064365386962890625e-10f;
  static double ANGLE;
  static float ANGLE_F;
  static constexpr double TOLERANCE = 1e-3;

  // Double precision limits
  static constexpr double DOUBLE_MIN = 2.22507385850720200e-308;
  static constexpr double DOUBLE_MAX = 1.7976931348623158e+308;
  static constexpr double EPSILON = 2.2204460492503131e-16;
  static constexpr double SQRT_EPSILON = 1.490116119385000000e-8;

  // Metric prefixes
  static constexpr double GIGA = 1e9;
  static constexpr double MEGA = 1e6;
  static constexpr double KILO = 1e3;
  static constexpr double MILLI = 1e-3;
  static constexpr double MICRO = 1e-6;
  static constexpr double NANO = 1e-9;

  // Base units
  static constexpr double FORCE = 1;
  static constexpr double MASS = 1;
  static constexpr double LENGTH = 1;

  // Scale factor
  static double SCALE;
  static float SCALE_F;

};

} // namespace geo