
#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector.h>

/**
 * @class Point
 * @brief A point defined by XYZ coordinates with display properties.
 */
class Xform {

    public:
      std::string guid = ::guid();       ///< Unique identifier for the point
      std::string name = "my_xform";     ///< XForm identifier/name
      std::array<float, 16> m;

      public:
      Xform() {
          m = {0.0f, 0.0f, 0.0f, 0.0f,
               0.0f, 0.0f, 0.0f, 0.0f,
               0.0f, 0.0f, 0.0f, 0.0f,
               0.0f, 0.0f, 0.0f, 0.0f};
          m[0] = 1.0f;
          m[5] = 1.0f;
          m[10] = 1.0f;
          m[15] = 1.0f;
      }

      Xform(const std::array<float, 16>& matrix) : m(matrix) {}

      static Xform identity() {
          return Xform();
      }


};