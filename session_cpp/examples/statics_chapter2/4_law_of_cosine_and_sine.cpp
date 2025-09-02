#include "core.hpp"

int main() {

    // cosine law
    double f1 = 100;
    double f2 = 150;
    double theta_r = 115;
    double r = geo::Vector::cosine_law(f1, f2, theta_r);
    geo::log(std::to_string(r));

    // sine law - angle
    double triangle_edge_length_a = 212.55;
    double angle_in_degrees_in_front_of_a = 115; 
    double triangle_edge_length_b = 150;
    double angle_in_degrees_in_front_of_b = geo::Vector::sine_law_angle(triangle_edge_length_a, angle_in_degrees_in_front_of_a, triangle_edge_length_b);
    geo::log(std::to_string(angle_in_degrees_in_front_of_b));

    // sine law - length
    triangle_edge_length_a = 212.55;
    angle_in_degrees_in_front_of_a = 115;
    angle_in_degrees_in_front_of_b = 39.761714;
    triangle_edge_length_b = geo::Vector::sine_law_length(triangle_edge_length_a, angle_in_degrees_in_front_of_a, angle_in_degrees_in_front_of_b);
    geo::log(std::to_string(triangle_edge_length_b));

    return 1;
}