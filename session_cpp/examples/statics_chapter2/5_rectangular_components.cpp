#include "core.hpp"

int main() {

    // Get components:
    geo::Vector f(3, 2);
    geo::Vector fx = f.xc();
    geo::Vector fy = f.yc();

    geo::log(f.to_string());
    geo::log(fx.to_string());
    geo::log(fy.to_string());

    // Sum components:
    geo::Vector f1(3, 2);
    geo::Vector f2(-4, 2);
    geo::Vector f3(-1, -1);
    geo::Vector f1x = f1.xc();
    geo::Vector f1y = f1.yc();
    geo::Vector f2x = f2.xc();
    geo::Vector f2y = f2.yc();
    geo::Vector f3x = f3.xc();
    geo::Vector f3y = f3.yc();
    geo::Vector sum_fx = f1x + f2x + f3x;
    geo::Vector sum_fy = f1y + f2y + f3y;
    geo::log(sum_fx.to_string());
    geo::log(sum_fy.to_string());

    // Vector length:
    geo::log(std::to_string(sqrt(pow(sum_fx[0],2)+pow(sum_fy[1],2))));
    geo::log(std::to_string((f1+f2+f3).length()));

    // 2D Angle between diagonal and x-axis:
    double tangent_angle = geo::Vector::angle_between_vector_xy_components_degrees(f3);
    geo::log(std::to_string(tangent_angle));

    // Same result as using 3D vector by dot product:
    double vector_angle = f3x.angle(f3);    
    geo::log(std::to_string(vector_angle));


    return 1;
}