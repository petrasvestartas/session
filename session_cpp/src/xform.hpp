
#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include "vector.h"
#include "point.h"
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector.h>

namespace session_cpp {
/**
 * @class Xform
 * @brief A transformation matrix for 3D operations.
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

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Basic Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////
    
    static Xform translation(float x, float y, float z){
        Xform xform;
        xform.m[12] = x;
        xform.m[13] = y;
        xform.m[14] = z;
        return xform;
    }

    static Xform scaling(float x, float y, float z){
        Xform xform;
        xform.m[0] = x;
        xform.m[5] = y;
        xform.m[10] = z;
        return xform;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Rotations
    ///////////////////////////////////////////////////////////////////////////////////////////

    static Xform rotation_x(float angle_radians){
        Xform xform;

        float cos_angle = cos(angle_radians);
        float sin_angle = sin(angle_radians);

        xform.m[5] = cos_angle;
        xform.m[6] = sin_angle;
        xform.m[9] = -sin_angle;
        xform.m[10] = cos_angle;

        return xform;
    }

    static Xform rotation_y(float angle_radians){
        Xform xform;

        float cos_angle = cos(angle_radians);
        float sin_angle = sin(angle_radians);

        xform.m[0] = cos_angle;
        xform.m[2] = -sin_angle;
        xform.m[8] = sin_angle;
        xform.m[10] = cos_angle;

        return xform;
    }

    static Xform rotation_z(float angle_radians){
        Xform xform;

        float cos_angle = cos(angle_radians);
        float sin_angle = sin(angle_radians);

        xform.m[0] = cos_angle;
        xform.m[1] = sin_angle;
        xform.m[4] = -sin_angle;
        xform.m[5] = cos_angle;

        return xform;
    }

    static Xform rotation(Vector& axis, float angle_radians){
        
        Xform xform;
        axis.unitize();
        
        float cos_angle = cos(angle_radians);
        float sin_angle = sin(angle_radians);
        float one_minus_cos = 1.0f - cos_angle;

        float xx = axis.x() * axis.x();
        float xy = axis.x() * axis.y();
        float xz = axis.x() * axis.z();
        float yy = axis.y() * axis.y();
        float yz = axis.y() * axis.z();
        float zz = axis.z() * axis.z();

        xform.m[0] = cos_angle + xx * one_minus_cos;
        xform.m[1] = xy * one_minus_cos + axis.z() * sin_angle;
        xform.m[2] = xz * one_minus_cos - axis.y() * sin_angle;

        xform.m[4] = xy * one_minus_cos - axis.z() * sin_angle;
        xform.m[5] = cos_angle + yy * one_minus_cos;
        xform.m[6] = yz * one_minus_cos + axis.x() * sin_angle;

        xform.m[8] = xz * one_minus_cos + axis.y() * sin_angle;
        xform.m[9] = yz * one_minus_cos - axis.x() * sin_angle;
        xform.m[10] = cos_angle + zz * one_minus_cos;

        return xform;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Advanced Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////

    static Xform change_basis(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis){
        
        Xform xform;

        x_axis.unitize();
        y_axis.unitize();
        z_axis.unitize();

        xform.m[0] = x_axis.x();
        xform.m[1] = x_axis.y();
        xform.m[2] = x_axis.z();

        xform.m[4] = y_axis.x();
        xform.m[5] = y_axis.y();
        xform.m[6] = y_axis.z();

        xform.m[8] = z_axis.x();
        xform.m[9] = z_axis.y();
        xform.m[10] = z_axis.z();

        xform.m[12] = origin.x();
        xform.m[13] = origin.y();
        xform.m[14] = origin.z();

        return xform;
    }


    ///////////////////////////////////////////////////////////////////////////////////////////
    // Matrix Operations
    ///////////////////////////////////////////////////////////////////////////////////////////




};

} // namespace session_cpp