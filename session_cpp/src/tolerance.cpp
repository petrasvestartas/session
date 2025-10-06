#include "tolerance.h"
#include <stdexcept>
#include <cmath>
#include <algorithm>

namespace session_cpp {

// Global tolerance instance
Tolerance TOL;

Tolerance::Tolerance(const std::string& unit) 
    : _unit(unit), _absolute(0), _relative(0), _angular(0), _approximation(0), 
      _precision(0), _lineardeflection(0), _angulardeflection(0),
      _has_absolute(false), _has_relative(false), _has_angular(false), 
      _has_approximation(false), _has_precision(false), 
      _has_lineardeflection(false), _has_angulardeflection(false) {
}

void Tolerance::reset() {
    _has_absolute = false;
    _has_relative = false;
    _has_angular = false;
    _has_approximation = false;
    _has_precision = false;
    _has_lineardeflection = false;
    _has_angulardeflection = false;
}

void Tolerance::set_unit(const std::string& value) {
    if (value != "M" && value != "MM") {
        throw std::invalid_argument("Invalid unit: " + value);
    }
    _unit = value;
}

void Tolerance::set_absolute(double value) {
    _absolute = value;
    _has_absolute = true;
}

void Tolerance::set_relative(double value) {
    _relative = value;
    _has_relative = true;
}

void Tolerance::set_angular(double value) {
    _angular = value;
    _has_angular = true;
}

void Tolerance::set_approximation(double value) {
    _approximation = value;
    _has_approximation = true;
}

void Tolerance::set_precision(int value) {
    if (value == 0) {
        throw std::invalid_argument("Precision cannot be zero.");
    }
    _precision = value;
    _has_precision = true;
}

void Tolerance::set_lineardeflection(double value) {
    _lineardeflection = value;
    _has_lineardeflection = true;
}

void Tolerance::set_angulardeflection(double value) {
    _angulardeflection = value;
    _has_angulardeflection = true;
}

double Tolerance::tolerance(double truevalue, double rtol, double atol) const {
    return rtol * std::abs(truevalue) + atol;
}

bool Tolerance::compare(double a, double b, double rtol, double atol) const {
    return std::abs(a - b) <= tolerance(b, rtol, atol);
}

bool Tolerance::is_zero(double a, double tol) const {
    double tolerance_val = (tol >= 0) ? tol : absolute();
    return std::abs(a) <= tolerance_val;
}

bool Tolerance::is_positive(double a, double tol) const {
    double tolerance_val = (tol >= 0) ? tol : absolute();
    return a > tolerance_val;
}

bool Tolerance::is_negative(double a, double tol) const {
    double tolerance_val = (tol >= 0) ? tol : absolute();
    return a < -tolerance_val;
}

bool Tolerance::is_between(double value, double minval, double maxval, double atol) const {
    double tolerance_val = (atol >= 0) ? atol : absolute();
    return minval - tolerance_val <= value && value <= maxval + tolerance_val;
}

bool Tolerance::is_close(double a, double b, double rtol, double atol) const {
    double rtol_val = (rtol >= 0) ? rtol : relative();
    double atol_val = (atol >= 0) ? atol : absolute();
    return compare(a, b, rtol_val, atol_val);
}

bool Tolerance::is_angle_zero(double a, double tol) const {
    double tolerance_val = (tol >= 0) ? tol : angular();
    return std::abs(a) <= tolerance_val;
}

bool Tolerance::is_angles_close(double a, double b, double tol) const {
    double tolerance_val = (tol >= 0) ? tol : angular();
    return std::abs(a - b) <= tolerance_val;
}

std::string Tolerance::geometric_key(double x, double y, double z, int precision) const {
    int prec = (precision != -999) ? precision : this->precision();
    
    if (prec == 0) {
        throw std::invalid_argument("Precision cannot be zero.");
    }
    
    if (prec == -1) {
        return std::to_string(static_cast<int>(x)) + "," + 
               std::to_string(static_cast<int>(y)) + "," + 
               std::to_string(static_cast<int>(z));
    }
    
    if (prec < -1) {
        int p = -prec - 1;
        double factor = std::pow(10.0, p);
        return std::to_string(static_cast<int>(std::round(x / factor) * factor)) + "," +
               std::to_string(static_cast<int>(std::round(y / factor) * factor)) + "," +
               std::to_string(static_cast<int>(std::round(z / factor) * factor));
    }
    
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(prec);
    
    // Handle -0.000 case
    std::ostringstream minzero;
    minzero << std::fixed << std::setprecision(prec) << -0.0;
    std::string minzero_str = minzero.str();
    
    std::ostringstream x_oss, y_oss, z_oss;
    x_oss << std::fixed << std::setprecision(prec) << x;
    y_oss << std::fixed << std::setprecision(prec) << y;
    z_oss << std::fixed << std::setprecision(prec) << z;
    
    if (x_oss.str() == minzero_str) x = 0.0;
    if (y_oss.str() == minzero_str) y = 0.0;
    if (z_oss.str() == minzero_str) z = 0.0;
    
    oss << x << "," << y << "," << z;
    return oss.str();
}

std::string Tolerance::geometric_key_xy(double x, double y, int precision) const {
    int prec = (precision != -999) ? precision : this->precision();
    
    if (prec == 0) {
        throw std::invalid_argument("Precision cannot be zero.");
    }
    
    if (prec == -1) {
        return std::to_string(static_cast<int>(x)) + "," + 
               std::to_string(static_cast<int>(y));
    }
    
    if (prec < -1) {
        int p = -prec - 1;
        double factor = std::pow(10.0, p);
        return std::to_string(static_cast<int>(std::round(x / factor) * factor)) + "," +
               std::to_string(static_cast<int>(std::round(y / factor) * factor));
    }
    
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(prec);
    
    // Handle -0.000 case
    std::ostringstream minzero;
    minzero << std::fixed << std::setprecision(prec) << -0.0;
    std::string minzero_str = minzero.str();
    
    std::ostringstream x_oss, y_oss;
    x_oss << std::fixed << std::setprecision(prec) << x;
    y_oss << std::fixed << std::setprecision(prec) << y;
    
    if (x_oss.str() == minzero_str) x = 0.0;
    if (y_oss.str() == minzero_str) y = 0.0;
    
    oss << x << "," << y;
    return oss.str();
}

std::string Tolerance::format_number(double number, int precision) const {
    int prec = (precision != -999) ? precision : this->precision();
    
    if (prec == 0) {
        throw std::invalid_argument("Precision cannot be zero.");
    }
    
    if (prec == -1) {
        return std::to_string(static_cast<int>(std::round(number)));
    }
    
    if (prec < -1) {
        int p = -prec - 1;
        double factor = std::pow(10.0, p);
        return std::to_string(static_cast<int>(std::round(number / factor) * factor));
    }
    
    std::ostringstream oss;
    oss << std::fixed << std::setprecision(prec) << number;
    return oss.str();
}

int Tolerance::precision_from_tolerance(double tol) const {
    double tolerance_val = (tol >= 0) ? tol : absolute();
    if (tolerance_val < 1.0) {
        std::ostringstream oss;
        oss << std::scientific << tolerance_val;
        std::string s = oss.str();
        size_t pos = s.find("e-");
        if (pos != std::string::npos) {
            return std::stoi(s.substr(pos + 2));
        }
    }
    return 0;
}

bool is_finite(double x) {
    return std::isfinite(x);
}

} // namespace session_cpp
