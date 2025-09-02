#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "globals.hpp"
#include "test_methods.hpp"

void default_constructor() {
    geo::Vector v;
    my_assert(v[0] == 0 && v[1] == 0 && v[2] == 0);
}

void constructor() {
    geo::Vector v(0.57, -158.63, 180.890);
    my_assert(v[0] == 0.57 && v[1] == -158.63 && v[2] == 180.890);
}

void static_methods() {
    geo::Vector v = geo::Vector::XAxis();
    my_assert(v[0] == 1 && v[1] == 0 && v[2] == 0);
    v = geo::Vector::YAxis();
    my_assert(v[0] == 0 && v[1] == 1 && v[2] == 0);
    v = geo::Vector::ZAxis();
    my_assert(v[0] == 0 && v[1] == 0 && v[2] == 1);
}

void from_start_and_end() {
    geo::Vector start(8.7, 5.7, -1.87);
    geo::Vector end(1, 1.57, 2);
    geo::Vector v = geo::Vector::from_start_and_end(start, end);
    my_assert(std::abs(v[0] - (-7.7)) < geo::GLOBALS::ZERO_TOLERANCE &&
              std::abs(v[1] - (-4.13)) < geo::GLOBALS::ZERO_TOLERANCE &&
              std::abs(v[2] - 3.87) < geo::GLOBALS::ZERO_TOLERANCE);
}

void operators() {
    geo::Vector v1(1, 2, 3);
    geo::Vector v2(4, 5, 6);
    geo::Vector v3 = v1 + v2;
    my_assert(v3[0] == 5 && v3[1] == 7 && v3[2] == 9);
    v3 = v1 - v2;
    my_assert(v3[0] == -3 && v3[1] == -3 && v3[2] == -3);
    v3 = v1 * 2;
    my_assert(v3[0] == 2 && v3[1] == 4 && v3[2] == 6);
    v3 = v1 / 2;
    my_assert(v3[0] == 0.5 && v3[1] == 1 && v3[2] == 1.5);
    // v3 = -v1;
    // my_assert(v3[0] == -1 && v3[1] == -2 && v3[2] == -3);
    v3 = v1;
    my_assert(v3[0] == 1 && v3[1] == 2 && v3[2] == 3);
    v3 += v2;
    my_assert(v3[0] == 5 && v3[1] == 7 && v3[2] == 9);
    v3 -= v2;
    my_assert(v3[0] == 1 && v3[1] == 2 && v3[2] == 3);
    v3 *= 2;
    my_assert(v3[0] == 2 && v3[1] == 4 && v3[2] == 6);
    v3 /= 2;
    my_assert(v3[0] == 1 && v3[1] == 2 && v3[2] == 3);
}

void reverse() {
    geo::Vector v(1, 2, 3);
    v.reverse();
    my_assert(v[0] == -1 && v[1] == -2 && v[2] == -3);
}

void length() {
    geo::Vector v(5.5697, -9.84, 1.587);
    double length = v.length();
    // std::cout << std::setprecision(std::numeric_limits<double>::max_digits10) << length;
    my_assert(length == 11.4177811806848);
}

void unitize() {
    geo::Vector v(5.5697, -9.84, 1.587);
    geo::Vector unitized_vector = v.unitized();
    my_assert(unitized_vector.length() == 1);
    v.unitize();
    my_assert(v.length() == 1);
}

void projection() {
    geo::Vector v(1, 1, 1);
    geo::Vector x_axis(1, 0, 0);
    geo::Vector y_axis(0, 1, 0);
    geo::Vector z_axis(0, 0, 1);
    double projection_length;
    geo::Vector projection_perpendicular_vector;
    double projection_perpendicular_vector_length;
    geo::Vector projection = v.projection(x_axis);
    my_assert(projection[0] == 1 && projection[1] == 0 && projection[2] == 0);
    projection = v.projection(y_axis);
    my_assert(projection[0] == 0 && projection[1] == 1 && projection[2] == 0);
    projection = v.projection(z_axis);
    my_assert(projection[0] == 0 && projection[1] == 0 && projection[2] == 1);

    v = geo::Vector(0, 300, 0);
    geo::Vector projection_vector = geo::Vector(2, 6, 3);
    projection =
        v.projection(projection_vector, geo::GLOBALS::ZERO_TOLERANCE, &projection_length,
                     &projection_perpendicular_vector, &projection_perpendicular_vector_length);
    my_assert(projection[0] - 73.4694 < geo::GLOBALS::MILLI &&
              projection[1] - 220.408 < geo::GLOBALS::MILLI &&
              projection[2] - 110.204 < geo::GLOBALS::MILLI);
    my_assert(projection_length - 257.142857 < geo::GLOBALS::MILLI);
    my_assert(projection_perpendicular_vector[0] - (-73.4694) < geo::GLOBALS::MILLI &&
              projection_perpendicular_vector[1] - 79.5918 < geo::GLOBALS::MILLI &&
              projection_perpendicular_vector[2] - (-110.204) < geo::GLOBALS::MILLI);
    my_assert(projection_perpendicular_vector_length - 154.523626 < geo::GLOBALS::MILLI);
}

void is_parallel_to() {
    geo::Vector v1(0, 0, 1);
    geo::Vector v2(0, 0, 2);
    geo::Vector v3(0, 0, -1);
    geo::Vector v4(0, 1, -1);
    my_assert(v1.is_parallel_to(v2) == 1);
    my_assert(v1.is_parallel_to(v3) == -1);
    my_assert(v1.is_parallel_to(v4) == 0);
}

void dot() {
    geo::Vector v1(1, 0, 0);
    geo::Vector v2(0, 1, 0);
    geo::Vector v3(-1, 0, 0);
    my_assert(v1.dot(v2) == 0);   // orthogonal
    my_assert(v1.dot(v3) == -1);  // antiparallel
    my_assert(v1.dot(v1) == 1);   // parallel

    // angle between vectors in degrees
    double dot_product = v1.dot(v2);
    double magnitudes = v1.length() * v2.length();

    if (magnitudes > 0.0) {
        double cos_angle = dot_product / magnitudes;
        double angle = acos(cos_angle);                             // angle in radians
        double angle_degrees = angle * (180.0 / geo::GLOBALS::PI);  // convert to degrees
        my_assert(angle_degrees == 90);                             // orthogonal
    } else {
        std::cout << "One or both vectors are zero vectors." << std::endl;
    }
}

void cross() {
    geo::Vector v1(1, 0, 0);
    geo::Vector v2(0, 1, 0);
    geo::Vector v3 = v1.cross(v2);
    my_assert(v3[0] == 0 && v3[1] == 0 && v3[2] == 1);
}

void angle() {
    geo::Vector v1(1, 1, 0);
    geo::Vector v2(0, 1, 0);
    double angle = v1.angle(v2, false);
    my_assert(angle - 45 < geo::GLOBALS::ZERO_TOLERANCE);
    v1 = geo::Vector(-1, 1, 0);
    angle = v1.angle(v2, angle);
    my_assert(angle - (-45) < geo::GLOBALS::ZERO_TOLERANCE);
}

void get_leveled_vector() {
    geo::Vector v(1, 1, 1);
    double scale = 1.0;
    geo::Vector leveled_vector = v.get_leveled_vector(scale);
    my_assert(abs(leveled_vector.length() - 4.1684325329666283) < geo::GLOBALS::ZERO_TOLERANCE);
}

void cosine_law() {
    double triangle_edge_length_a = 100;
    double triangle_edge_length_b = 150;
    double angle_in_degrees_between_edges = 115;
    double triangle_edge_length_c = geo::Vector::cosine_law(
        triangle_edge_length_a, triangle_edge_length_b, angle_in_degrees_between_edges, true);

    double scale = std::pow(10.0, 2);
    triangle_edge_length_c = std::round(triangle_edge_length_c * scale) / scale;
    my_assert(triangle_edge_length_c == 212.55);
}

void sine_law_angle() {
    double triangle_edge_length_a = 212.55;
    double angle_in_degrees_in_front_of_a = 115;
    double triangle_edge_length_b = 150;

    double angle_in_degrees_in_front_of_b = geo::Vector::sine_law_angle(
        triangle_edge_length_a, angle_in_degrees_in_front_of_a, triangle_edge_length_b);

    double scale = std::pow(10.0, 2);
    angle_in_degrees_in_front_of_b = std::round(angle_in_degrees_in_front_of_b * scale) / scale;
    my_assert(angle_in_degrees_in_front_of_b == 39.76);
}

void sine_law_length() {
    double triangle_edge_length_a = 212.55;
    double angle_in_degrees_in_front_of_a = 115;
    double angle_in_degrees_in_front_of_b = 39.761714;

    double triangle_edge_length_b = geo::Vector::sine_law_length(
        triangle_edge_length_a, angle_in_degrees_in_front_of_a, angle_in_degrees_in_front_of_b);

    double scale = std::pow(10.0, 2);
    triangle_edge_length_b = std::round(triangle_edge_length_b * scale) / scale;
    my_assert(triangle_edge_length_b == 150);
}

void angle_between_vector_xy_components() {
    geo::Vector v(sqrt(3), 1, 0);
    double angle = geo::Vector::angle_between_vector_xy_components_degrees(v);
    my_assert(round(angle * 100) / 100 == 30);
    v = geo::Vector(1, sqrt(3), 0);
    angle = geo::Vector::angle_between_vector_xy_components_degrees(v);
    my_assert(round(angle * 100) / 100 == 60);
}

void sum_of_vectors() {
    std::vector<geo::Vector> vectors = {geo::Vector(1, 1, 1), geo::Vector(2, 2, 2),
                                        geo::Vector(3, 3, 3)};
    geo::Vector sum = geo::Vector::sum_of_vectors(vectors);
    my_assert(sum[0] == 6 && sum[1] == 6 && sum[2] == 6);
}

void coordinate_direction_angles() {
    geo::Vector v(35.4, 35.4, 86.6);
    std::array<double, 3> alpha_beta_gamma = v.coordinate_direction_3angles(true);
    my_assert(abs(alpha_beta_gamma[0] - 69.274204) < geo::GLOBALS::MICRO &&
              abs(alpha_beta_gamma[1] - 69.274204) < geo::GLOBALS::MICRO &&
              abs(alpha_beta_gamma[2] - 30.032058) < geo::GLOBALS::MICRO);

    v = geo::Vector(1, 1, sqrt(2));
    std::array<double, 2> phi_theta = v.coordinate_direction_2angles(true);
    my_assert(abs(phi_theta[0] - 45) < geo::GLOBALS::MICRO &&
              abs(phi_theta[1] - 45) < geo::GLOBALS::MICRO);
}

void scale() {
    geo::Vector v(1, 1, 1);
    v.scale(2);
    my_assert(v[0] == 2 && v[1] == 2 && v[2] == 2);
}

void scale_up() {
    geo::Vector v(1, 1, 1);
    v.scale_up();
    my_assert(v[0] == geo::GLOBALS::SCALE && v[1] == geo::GLOBALS::SCALE &&
              v[2] == geo::GLOBALS::SCALE);
}

void scale_down() {
    geo::Vector v(geo::GLOBALS::SCALE, geo::GLOBALS::SCALE, geo::GLOBALS::SCALE);
    v.scale_down();
    my_assert(v[0] == 1 && v[1] == 1 && v[2] == 1);
}

int test_vector_main() {
    std::cout << "Running test_vector..." << std::endl;
    default_constructor();
    constructor();
    static_methods();
    from_start_and_end();
    operators();
    reverse();
    length();
    unitize();
    projection();
    is_parallel_to();
    dot();
    cross();
    angle();
    get_leveled_vector();
    cosine_law();
    sine_law_angle();
    sine_law_length();
    angle_between_vector_xy_components();
    sum_of_vectors();
    coordinate_direction_angles();
    scale();
    scale_up();
    scale_down();
    return 0;
}