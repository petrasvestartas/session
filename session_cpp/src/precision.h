#pragma once

/**
 * @file precision.h
 * @brief Floating-point precision configuration for cross-language consistency
 * 
 * PRECISION POLICY:
 * - Python: Uses float64 (double) by default
 * - C++: Uses double to match Python
 * - Rust: Uses f64 to match Python
 * 
 * All three implementations use the same precision for consistent results.
 */

namespace session_cpp {

// Floating-point precision type
// Change this to 'float' if you need single precision
using Real = double;

// Literal suffix for consistency
#define REAL(x) x

// Common constants with correct precision
constexpr Real PI = 3.14159265358979323846;
constexpr Real TWO_PI = 6.28318530717958647692;
constexpr Real HALF_PI = 1.57079632679489661923;

/**
 * MIGRATION GUIDE: float → double
 * =================================
 * 
 * To complete the precision migration:
 * 
 * 1. CORE GEOMETRY (HIGH PRIORITY):
 *    - point.h: float x, y, z → double x, y, z
 *    - vector.h: float components → double components  
 *    - line.h: Point parameters
 *    - plane.h: Point/Vector parameters
 *    - boundingbox.h: Point/Vector parameters
 *    
 * 2. SESSION & BVH:
 *    - session.h: float tolerance → double tolerance
 *    - bvh.h: float parameters → double parameters
 *    - intersection.h: float parameters → double parameters
 *    
 * 3. COMPLEX GEOMETRY:
 *    - mesh.h: float vertices → double vertices
 *    - polyline.h: float parameters → double parameters
 *    - cylinder.h: float radius → double radius
 *    - arrow.h: float radius → double radius
 *    
 * 4. UTILITIES:
 *    - xform.h: float matrix → double matrix
 *    - quaternion.h: float components → double components
 *    - tolerance.h: float constants → double constants
 *    
 * 5. FIND-REPLACE PATTERNS:
 *    - "float " → "double "
 *    - "1.0" → "1.0"
 *    - "0.5" → "0.5"
 *    - "1e-3f" → "1e-3"
 *    
 * TESTING:
 *    - Recompile and run all tests
 *    - Verify cross-language consistency with Python
 *    - Check tolerance values (may need adjustment)
 */

} // namespace session_cpp
