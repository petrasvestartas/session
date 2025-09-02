#pragma once
#include <math.h>

#include <array>
#include <cmath>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "globals.hpp"

namespace geo {

/**
 * @class Vector
 * @brief A class representing a Vector in 3D space.
 */
class Vector {
   public:
    /**
     * @brief Default constructor. Initializes the Vector at the origin.
     */
    Vector();

    /**
     * @brief Constructor that initializes the Vector at the given coordinates.
     * @param x The x-coordinate.
     * @param y The y-coordinate.
     * @param z The z-coordinate, optional.
     */
    Vector(double x, double y, double z = 0);

    /**
     * @brief Constructor that initializes the Vector at the scalars.
     * @param a The a-scale of a unit vector in x axis.
     * @param b The b-scale of a unit vector in y axis.
     * @param c The c-scale of a unit vector in z axis, optional.
     * @return Vector The vector from scalars.
     */
    static Vector from_scalars(double a, double b, double c = 0);

    /**
     * @brief Returns a unit vector along the x-axis.
     *
     * @return Vector The unit vector along the x-axis.
     */
    static Vector XAxis();

    /**
     * @brief Returns a unit vector along the y-axis.
     *
     * @return Vector The unit vector along the y-axis.
     */
    static Vector YAxis();

    /**
     * @brief Returns a unit vector along the z-axis.
     *
     * @return Vector The unit vector along the z-axis.
     */
    static Vector ZAxis();

    /**
     * @brief Get a vector component along the x-axis Vector(x, 0, 0).
     *
     * @return Vector component along the x-axis.
     */
    Vector xc() const;

    /**
     * @brief Get a vector component along the x-axis Vector(x, 0, 0).
     *
     * @return Vector component along the x-axis.
     */
    Vector yc() const;

    /**
     * @brief Get a vector component along the x-axis Vector(x, 0, 0).
     *
     * @return Vector component along the x-axis.
     */
    Vector zc() const;

    /**
     * @brief Returns a vector from start to end. The vector points to the coordinates of the end
     * vector.
     *
     * @param start The start vector.
     * @param end The end vector.
     * @return Vector The vector from start to end.
     */
    static Vector from_start_and_end(const Vector& start, const Vector& end);

    /**
     * @brief Subscript operator for non-const access.
     * @param index The index of the coordinate to access (0 for x, 1 for y, 2 for z).
     * @return A reference to the coordinate at the given index.
     */
    double& operator[](int index);

    /**
     * @brief Subscript operator for const access.
     * @param index The index of the coordinate to access (0 for x, 1 for y, 2 for z).
     * @return A const reference to the coordinate at the given index.
     */
    const double& operator[](int index) const;

    /**
     * @brief Operator for non-const access of a scalar component.
     * @param index The index of the coordinate to access (0 for a, 1 for b, 2 for c).
     * @return A reference to the coordinate at the given index.
     */
    double& operator()(int index);

    /**
     * @brief Operator for const access of a scalar component.
     * @param index The index of the coordinate to access (0 for a, 1 for b, 2 for c).
     * @return A const reference to the coordinate at the given index.
     */
    const double& operator()(int index) const;

    /**
     * @brief Scale the vector by a factor (multiplication).
     * @param factor The factor to scale the vector by.
     * @return A reference to the vector after scaling.
     */
    Vector& operator*=(double factor);

    /**
     * @brief Friend non-member operator* overload to handle double * Vector.
     * @param factor The factor to scale the vector by.
     * @param v The vector to scale.
     */
    friend Vector operator*(double factor, const Vector& v);

    /**
     * @brief Scale the vector by a factor (division).
     * @param factor The factor to scale the vector by.
     * @return A reference to the vector after scaling.
     */
    Vector& operator/=(double factor);

    /**
     * @brief Add another vector to this vector.
     * @param other The vector to add.
     * @return A reference to the vector after addition.
     */
    Vector& operator+=(const Vector& other);

    /**
     * @brief Subtract another vector from this vector.
     * @param other The vector to subtract.
     * @return A reference to the vector after subtraction.
     */
    Vector& operator-=(const Vector& other);

    /**
     * @brief Multiply the vector by a factor.
     * @param factor The factor to multiply the vector by.
     * @return A new vector that is the result of the multiplication.
     */
    Vector operator*(double factor) const;

    /**
     * @brief Divide the vector by a factor.
     * @param factor The factor to divide the vector by.
     * @return A new vector that is the result of the division.
     */
    Vector operator/(double factor) const;

    /**
     * @brief Add another vector to this vector.
     * @param other The vector to add.
     * @return A new vector that is the result of the addition.
     */
    Vector operator+(const Vector& other) const;

    /**
     * @brief Subtract another vector from this vector.
     * @param other The vector to subtract.
     * @return A new vector that is the result of the subtraction.
     */
    Vector operator-(const Vector& other) const;

    /**
     * @brief Check if this vector is equal to another vector.
     * @param other The vector to compare with.
     * @return True if the vectors are equal, false otherwise.
     */
    bool operator==(const Vector& other) const;

    /**
     * @brief Check if this vector is not equal to another vector.
     * @param other The vector to compare with.
     * @return True if the vectors are not equal, false otherwise.
     */
    bool operator!=(const Vector& other) const;

    /**
     * @brief Reverse the vector.
     * @return A new vector that is the result of the reversal.
     */
    void reverse();

    /**
     * @brief Get a length of a vector.
     * @param predefined_length predefined length of the vector, use only when unit _abc vector
     * coordinates are not zero
     * @return A vector distance.
     */
    double length(double predefined_length = 0);

    double compute_length() const;  // Declare const overload

    /**
     * @brief Get a squared length of a vector.
     * @return True if a vector is unitized.
     */
    bool unitize();

    /**
     * @brief Get a squared length copy of a vector.
     * @return True if a vector is unitized.
     */
    Vector unitized();

    /**
     * @brief Project a current vector to the projection_vector.
     *
     * @param [in] projection_vector vector to project to
     * @param [out] out_projected_vector_length length of the projected vector
     * @param [out] out_perpendicular_projected_vector perpendicular projected vector
     * @param [out] out_perpendicular_projected_vector_length length of the perpendicular projected
     * vector
     * @return projected vector
     */
    Vector projection(Vector& projection_vector, double tolerance = geo::GLOBALS::ZERO_TOLERANCE,
                      double* out_projected_vector_length = nullptr,
                      Vector* out_perpendicular_projected_vector = nullptr,
                      double* out_perpendicular_projected_vector_length = nullptr);

    /**
     * Check if two vectors are parallel or anti-parallel or not-parallel
     * tolerance geo::GLOBALS::ANGLE in degrees
     *
     * @param [in] v vector
     * @return 1: this and other vectors are and parallel
     * -1: this and other vectors are anti-parallel
     * 0: this and other vectors are not parallel or at least one of the vectors is zero
     * 1: this and other vectors are parallel
     */
    int is_parallel_to(Vector& v);

    /**
     * @brief Calculate a dot product of a vector.
     * And check if two vectors are parallel=1 or anti-parallel=-1 or orthogonal = 0.
     * Dot product can be used to calculate the angle between two vectors if the magnitude of
     * vectors is known.
     * @return Dot product.
     */
    double dot(Vector& other);

    /**
     * @brief Calculate a cross product of a vector.
     * The cross product of two vectors A and B  yields the vector C, which is written C = A x B.
     * The length of a cross product = |C| = |A| * |B| * sin(theta)
     * Laws:
     * 1. Order matters: A x B != B x A, only works by changing the sign A x B = -B x A
     * 2. Scaling: a(A x B) = (aA) x B = A x (aB) = a(A x B)
     * 3. Distributive law of addition, order matters: A x (B + C) = A x B + A x C
     *
     *
     * The cross product of two vectors results in a third vector that is orthogonal (perpendicular)
     * to the original two vectors. The magnitude (length) of the cross product vector is equal to
     * the area of the parallelogram that the two vectors span.
     * @return Cross product.
     */
    Vector cross(Vector& other);

    // /**
    //  * @brief Direction of a cross product
    //  * i x j = k, i x k = -j, i x i = 0
    //  * j x k = i, j x i = -k, j x j = 0
    //  * k x i = j, k x j = -i, k x k = 0
    //  * where i, j, k are x, y, z unit vectors
    // */
    // int cross_product_sign(Vector &other);

    /**
     * @brief Calculate an angle between two vectors. Angle is the dot product divided by the
     * multiplication of the magnitudes of the vectors.
     *
     * @param [in] other vector
     * @param [in] radians if true, the angle will be returned in radians, otherwise in degrees
     * @param [in] tolerance tolerance for comparing the angle with 0 and 180 degrees
     * @return Angle in radians or degrees.
     */
    double angle(Vector& other, bool sign_by_cross_product = true, bool degrees = true,
                 double tolerance = geo::GLOBALS::ZERO_TOLERANCE);

    /**
     * Scale the vector to the given vertical height
     * Often used for CNC machining
     *
     * @param [in] vertical_height vertical height of the vector
     * @return scaled vector, whose vertical component length is equal to the vertical_height
     */
    Vector get_leveled_vector(double& vertical_height);

    /**
     * Get the third triangle edge length.
     * When two triangle edges and angle in between them is known.
     *
     * The Law of Cosines states:
     * C = sqrt(A^2 + B^2 - 2AB * cos(c))
     *
     * Here's an ASCII art representation of a triangle:
     *
     *        c
     *        /\
     *       /  \
     *    A /    \ B
     *     /      \
     *    /________\
     *   b     C    a
     *
     * @param [in] triangle_edge_length_a first edge length
     * @param [in] triangle_edge_length_b second edge length
     * @param [in] angle_between_edges angle between the edges that will be used in cosine function
     * @return the length of opposite triangle length
     */
    static double cosine_law(double& triangle_edge_length_a, double& triangle_edge_length_b,
                             double& angle_in_degrees_between_edges, bool degrees = true);

    /**
     * Compute and edge of a triangle or the angle in front of it.
     *
     * The Law of Sines states:
     * (A / sin(a)) = (B / sin(b)) = (C / sin(c))
     *
     *
     *        c
     *        /\
     *       /  \
     *    A /    \ B <--- triangle_edge_length_b
     *     /      \
     *    /________\
     *   b     C    a
     *
     * @param [in] triangle_edge_length_a any other triangle edge length
     * @param [in] angle_in_front_of_a triangle angle opposite of that edge
     * @param [in] triangle_edge_length_b the length of searchable triangle edge
     * @return the angle in front of the edge b
     */
    static double sine_law_angle(double& triangle_edge_length_a, double& angle_in_front_of_a,
                                 double& triangle_edge_length_b, bool degrees = true);

    /**
     * Compute and edge of a triangle or the angle in front of it.
     *
     *
     * The Law of Sines states:
     * (A / sin(a)) = (B / sin(b)) = (C / sin(c))
     *
     *
     *        c
     *        /\
     *       /  \
     *    A /    \ B
     *     /      \
     *    /________\
     *   b     C    a
     *   |
     *   triangle_edge_length_b
     *
     * @param [in] triangle_edge_length_a any other triangle edge length
     * @param [in] angle_in_front_of_a triangle angle opposite of that edge
     * @param [in] angle_in_front_of_b the angle in front of searchable triangle edge
     * @return the length of searchable triangle edge
     */
    static double sine_law_length(double& triangle_edge_length_a, double& angle_in_front_of_a,
                                  double& angle_in_front_of_b, bool degrees = true);

    /**
     * Compute the angle between two vectors.
     *
     * Fy   F
     * |   /
     * |  /
     * | / θ
     * * ___ Fx
     *
     * @param [in] v1 first vector
     * @param [in] v2 second vector
     * @return the angle between two vectors
     */
    static double angle_between_vector_xy_components_degrees(Vector& vector, bool degrees = true);

    /**
     * @brief Algebraic sum of vectors, by summing up the components.
     *
     * @param [in] vectors vector list
     * @return the sum of vectors
     */
    static Vector sum_of_vectors(std::vector<Vector>& vectors);

    /**
     * @brief Calculate the angle between the vector and the coordinate axes.
     *
     * @return the angle between the vector and the coordinate axes in degrees
     */
    std::array<double, 3> coordinate_direction_3angles(bool degrees = false);

    /**
     * @brief Calculate the horizontal and vertical angle between the vector and the coordinate
     * axes.
     *
     * @return the angle between the vector and the coordinate axes in degrees
     */
    std::array<double, 2> coordinate_direction_2angles(bool degrees = false);

    /**
     * @brief Create a vector perpendicular to the current vector.
     * Implemented from the PerpendicularTo method in:
     * https://github.com/mcneel/opennurbs/blob/56a55eb632631b0eace027471eba1076b3d6e124/opennurbs_point.cpp#L1228
     *
     * @return Vector perpendicular to the current vector.
     */
    bool perpendicular_to(Vector& v);

    /**
     * @brief Scales the Vector by a given factor.
     * @param factor The factor to scale by.
     */
    void scale(double factor);

    /**
     * @brief Scales the Vector up by a factor of global SCALE.
     */
    void scale_up();

    /**
     * @brief Scales the Vector down by a factor of global SCALE.
     */
    void scale_down();

    /**
     * @brief Unitize a vector and scale it to a given length.
     * @param factor The factor to rescale by.
     */
    void rescale(double factor);

    /**
     * @brief Unitize a vector and scale it to a given length.
     * @param factor The factor to rescale by.
     * @return Vector that is a copy of the current vector.
     */
    Vector rescaled(double factor);

    // /**
    //  * @brief Transform the vector by a given matrix.
    //  * @param matrix - The matrix to transform by.
    //  */
    // void transform(const double matrix[4][4]);

    /**
     * @brief Returns a string representation of the Vector including its class name and xyz
     * coordinates.
     * @return A string representation of the Vector.
     */
    std::string to_string();

   private:
    /**
     * @brief The coordinates of the Vector.
     */
    double _xyz[3];
    double _abc[3];
    bool _has_length;
    double _length;
    bool _has_unit_vector;
};
}  // namespace geo