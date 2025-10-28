#pragma once
#include <cmath>
#include <string>
#include <sstream>
#include <iomanip>

namespace session_cpp {

// Scale factor
constexpr double SCALE = 1e6;

/**
 * @class Tolerance
 * @brief A tolerance class for geometric comparisons.
 */
class Tolerance {
public:
    // Mathematical constants (moved from namespace level for MSVC compatibility)
    static constexpr double PI = 3.14159265358979323846;
    static constexpr double TO_DEGREES = 180.0 / PI;
    static constexpr double TO_RADIANS = PI / 180.0;
    
    // Default tolerance values (double precision)
    static constexpr double ABSOLUTE = 1e-9;
    static constexpr double RELATIVE = 1e-6;
    static constexpr double ANGULAR = 1e-6;
    static constexpr double APPROXIMATION = 1e-3;
    static constexpr int PRECISION = 3;
    static constexpr double LINEARDEFLECTION = 1e-3;
    static constexpr double ANGULARDEFLECTION = 1e-1;
    static constexpr double ANGLE_TOLERANCE_DEGREES = 0.11;
    static constexpr double ZERO_TOLERANCE = 1e-12;

private:
    std::string _unit;
    double _absolute;
    double _relative;
    double _angular;
    double _approximation;
    int _precision;
    double _lineardeflection;
    double _angulardeflection;
    
    bool _has_absolute;
    bool _has_relative;
    bool _has_angular;
    bool _has_approximation;
    bool _has_precision;
    bool _has_lineardeflection;
    bool _has_angulardeflection;

public:
    /// Construct tolerance with a unit system (e.g., "M", "MM")
    explicit Tolerance(const std::string& unit = "M");
    
    /// Reset all overrides to default constants
    void reset();
    
    // Getters
    /// Current unit system
    std::string unit() const { return _unit; }
    /// Absolute tolerance value (or default ABSOLUTE)
    double absolute() const { return _has_absolute ? _absolute : ABSOLUTE; }
    /// Relative tolerance value (or default RELATIVE)
    double relative() const { return _has_relative ? _relative : RELATIVE; }
    /// Angular tolerance value in radians (or default ANGULAR)
    double angular() const { return _has_angular ? _angular : ANGULAR; }
    /// Approximation tolerance (or default APPROXIMATION)
    double approximation() const { return _has_approximation ? _approximation : APPROXIMATION; }
    /// Decimal precision used for formatting (or default PRECISION)
    int precision() const { return _has_precision ? _precision : PRECISION; }
    /// Linear deflection value (or default LINEARDEFLECTION)
    double lineardeflection() const { return _has_lineardeflection ? _lineardeflection : LINEARDEFLECTION; }
    /// Angular deflection value (or default ANGULARDEFLECTION)
    double angulardeflection() const { return _has_angulardeflection ? _angulardeflection : ANGULARDEFLECTION; }
    
    // Setters
    /// Set current unit system
    void set_unit(const std::string& value);
    /// Override absolute tolerance
    void set_absolute(double value);
    /// Override relative tolerance
    void set_relative(double value);
    /// Override angular tolerance (radians)
    void set_angular(double value);
    /// Override approximation tolerance
    void set_approximation(double value);
    /// Override decimal precision for formatting
    void set_precision(int value);
    /// Override linear deflection
    void set_lineardeflection(double value);
    /// Override angular deflection
    void set_angulardeflection(double value);
    
    // Tolerance operations
    /// Compute combined tolerance from relative and absolute components
    double tolerance(double truevalue, double rtol, double atol) const;
    /// Compare two values within tolerance
    bool compare(double a, double b, double rtol, double atol) const;
    /// Check if value is within zero tolerance
    bool is_zero(double a, double tol = -1) const;
    /// Check if value is positive within tolerance
    bool is_positive(double a, double tol = -1) const;
    /// Check if value is negative within tolerance
    bool is_negative(double a, double tol = -1) const;
    /// Check if value is within a range with absolute tolerance
    bool is_between(double value, double minval, double maxval, double atol = -1) const;
    /// Check closeness between two values using rtol/atol
    bool is_close(double a, double b, double rtol = -1, double atol = -1) const;
    /// Check if an angle is effectively zero (radians)
    bool is_angle_zero(double a, double tol = -1) const;
    /// Check if two angles are close (radians)
    bool is_angles_close(double a, double b, double tol = -1) const;
    
    // Formatting
    /// Create a geometric key string for 3D point with optional precision
    std::string geometric_key(double x, double y, double z, int precision = -999) const;
    /// Create a geometric key string for 2D point with optional precision
    std::string geometric_key_xy(double x, double y, int precision = -999) const;
    /// Format a number with optional precision override
    std::string format_number(double number, int precision = -999) const;
    /// Determine decimal precision from a tolerance value
    int precision_from_tolerance(double tol = -1) const;
};

// Global tolerance instance
extern Tolerance TOL;

// Utility function
bool is_finite(double x);

} // namespace session_cpp
