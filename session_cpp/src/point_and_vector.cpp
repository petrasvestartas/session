#include "point_and_vector.hpp"

namespace geo {

std::array<int, 2> moment_component_signs_varignon(Point& p, Vector& v) {
    // get the x and y components of the vector
    std::array<int, 2> result = {0, 0};

    double x = p[0];
    double y = p[1];
    double Fx = v[0];
    double Fy = v[1];

    // x component
    if (Fx > 0 && y > 0)
        result[0] = -1;
    else if (Fx > 0 && y < 0)
        result[0] = 1;
    else if (Fx < 0 && y > 0)
        result[0] = 1;
    else if (Fx < 0 && y < 0)
        result[0] = -1;

    // y component
    if (Fy < 0 && x > 0)
        result[1] = -1;
    else if (Fy < 0 && x < 0)
        result[1] = 1;
    else if (Fy > 0 && x > 0)
        result[1] = 1;
    else if (Fy > 0 && x < 0)
        result[1] = -1;

    return result;
}

double moment_varignon(Point& point, Vector& force) {
    // Varignon theorem: M = sign_x*(Fx * y) + sign_y*(Fy * x)
    double x_component = abs(force[0]) * abs(point[1]);
    double y_component = abs(force[1]) * abs(point[0]);
    std::array<int, 2> sign = moment_component_signs_varignon(point, force);
    double moment = sign[0] * x_component + sign[1] * y_component;
    // std::cout << sign[0]*x_component << std::endl;
    // std::cout << sign[1]*y_component << std::endl;
    return moment;
}

double moments_varignon_sum(std::vector<Point>& origins, std::vector<Vector>& forces) {
    // Verington theorem: the moment of a force abount the point is equal to the sum of the moments
    // of the force components
    double sum = 0.0;
    for (size_t i = 0; i < forces.size(); ++i) {
        sum += moment_varignon(origins[i], forces[i]);
    }

    return sum;
}
}  // namespace geo