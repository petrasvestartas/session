#pragma once
#include <cmath>
#include <string>
#include <sstream>
#include <iomanip>

namespace session_cpp {

// Mathematical constants
constexpr double PI = 3.14159265358979323846;
constexpr double TO_DEGREES = 180.0 / PI;
constexpr double TO_RADIANS = PI / 180.0;

// Scale factor
constexpr double SCALE = 1e6;

class Tolerance {
public:
    // Default tolerance values
    static constexpr double ABSOLUTE = 1e-9;
    static constexpr double RELATIVE = 1e-6;
    static constexpr double ANGULAR = 1e-6;
    static constexpr double APPROXIMATION = 1e-3;
    static constexpr int PRECISION = 3;
    static constexpr double LINEARDEFLECTION = 1e-3;
    static constexpr double ANGULARDEFLECTION = 1e-1;
    
    // Angle tolerance in degrees
    static constexpr double ANGLE_TOLERANCE_DEGREES = 0.11;
    
    // Zero tolerance for comparisons
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
    explicit Tolerance(const std::string& unit = "M");
    
    void reset();
    
    // Getters
    std::string unit() const { return _unit; }
    double absolute() const { return _has_absolute ? _absolute : ABSOLUTE; }
    double relative() const { return _has_relative ? _relative : RELATIVE; }
    double angular() const { return _has_angular ? _angular : ANGULAR; }
    double approximation() const { return _has_approximation ? _approximation : APPROXIMATION; }
    int precision() const { return _has_precision ? _precision : PRECISION; }
    double lineardeflection() const { return _has_lineardeflection ? _lineardeflection : LINEARDEFLECTION; }
    double angulardeflection() const { return _has_angulardeflection ? _angulardeflection : ANGULARDEFLECTION; }
    
    // Setters
    void set_unit(const std::string& value);
    void set_absolute(double value);
    void set_relative(double value);
    void set_angular(double value);
    void set_approximation(double value);
    void set_precision(int value);
    void set_lineardeflection(double value);
    void set_angulardeflection(double value);
    
    // Tolerance operations
    double tolerance(double truevalue, double rtol, double atol) const;
    bool compare(double a, double b, double rtol, double atol) const;
    bool is_zero(double a, double tol = -1) const;
    bool is_positive(double a, double tol = -1) const;
    bool is_negative(double a, double tol = -1) const;
    bool is_between(double value, double minval, double maxval, double atol = -1) const;
    bool is_close(double a, double b, double rtol = -1, double atol = -1) const;
    bool is_angle_zero(double a, double tol = -1) const;
    bool is_angles_close(double a, double b, double tol = -1) const;
    
    // Formatting
    std::string geometric_key(double x, double y, double z, int precision = -999) const;
    std::string geometric_key_xy(double x, double y, int precision = -999) const;
    std::string format_number(double number, int precision = -999) const;
    int precision_from_tolerance(double tol = -1) const;
};

// Global tolerance instance
extern Tolerance TOL;

// Utility function
bool is_finite(double x);

} // namespace session_cpp
