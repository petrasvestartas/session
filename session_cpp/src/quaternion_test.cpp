#include "catch_amalgamated.hpp"
#include "quaternion.h"
#include <cmath>

using namespace session_cpp;

const float PI = 3.14159265359f;

bool approx_f32(float a, float b, float tol = 1e-5f) {
    return std::abs(a - b) < tol;
}

bool vectors_close(const Vector& a, const Vector& b, float tol = 1e-5f) {
    return approx_f32(a.x(), b.x(), tol) && approx_f32(a.y(), b.y(), tol) && approx_f32(a.z(), b.z(), tol);
}

TEST_CASE("test_quaternion_identity") {
    Quaternion q = Quaternion::identity();
    REQUIRE(q.s == 1.0f);
    REQUIRE(q.v.x() == 0.0f);
    REQUIRE(q.v.y() == 0.0f);
    REQUIRE(q.v.z() == 0.0f);
}

TEST_CASE("test_quaternion_from_axis_angle_90deg_z") {
    Vector axis(0.0f, 0.0f, 1.0f);
    float angle = PI / 2.0f;
    Quaternion q = Quaternion::from_axis_angle(axis, angle);

    REQUIRE(approx_f32(q.s, std::cos(PI / 4.0f)));
    REQUIRE(approx_f32(q.v.z(), std::sin(PI / 4.0f)));
}

TEST_CASE("test_quaternion_rotate_vector_90deg_z") {
    Vector axis(0.0f, 0.0f, 1.0f);
    float angle = PI / 2.0f;
    Quaternion q = Quaternion::from_axis_angle(axis, angle);

    Vector v(1.0f, 0.0f, 0.0f);
    Vector rotated = q.rotate_vector(v);

    Vector expected(0.0f, 1.0f, 0.0f);
    REQUIRE(vectors_close(rotated, expected));
}

TEST_CASE("test_quaternion_rotate_vector_180deg_z") {
    Vector axis(0.0f, 0.0f, 1.0f);
    float angle = PI;
    Quaternion q = Quaternion::from_axis_angle(axis, angle);

    Vector v(1.0f, 0.0f, 0.0f);
    Vector rotated = q.rotate_vector(v);

    Vector expected(-1.0f, 0.0f, 0.0f);
    REQUIRE(vectors_close(rotated, expected));
}

TEST_CASE("test_quaternion_normalize") {
    Quaternion q = Quaternion::from_sv(2.0f, 0.0f, 0.0f, 0.0f);
    Quaternion normalized = q.normalize();

    REQUIRE(approx_f32(normalized.magnitude(), 1.0f));
    REQUIRE(approx_f32(normalized.s, 1.0f));
}

TEST_CASE("test_quaternion_multiplication") {
    Quaternion q1 = Quaternion::from_axis_angle(Vector(0.0f, 0.0f, 1.0f), PI / 2.0f);
    Quaternion q2 = Quaternion::from_axis_angle(Vector(0.0f, 0.0f, 1.0f), PI / 2.0f);
    Quaternion q_combined = q1 * q2;

    Vector v(1.0f, 0.0f, 0.0f);
    Vector rotated = q_combined.rotate_vector(v);

    Vector expected(-1.0f, 0.0f, 0.0f);
    REQUIRE(vectors_close(rotated, expected));
}

TEST_CASE("test_quaternion_identity_rotation") {
    Quaternion q = Quaternion::identity();
    Vector v(1.0f, 2.0f, 3.0f);
    Vector rotated = q.rotate_vector(v);

    REQUIRE(vectors_close(rotated, v));
}

TEST_CASE("test_quaternion_conjugate") {
    Quaternion q = Quaternion::from_sv(0.5f, 0.5f, 0.5f, 0.5f);
    Quaternion conj = q.conjugate();

    REQUIRE(conj.s == 0.5f);
    REQUIRE(conj.v.x() == -0.5f);
    REQUIRE(conj.v.y() == -0.5f);
    REQUIRE(conj.v.z() == -0.5f);
}

TEST_CASE("test_quaternion_magnitude") {
    Quaternion q = Quaternion::from_sv(1.0f, 0.0f, 0.0f, 0.0f);
    REQUIRE(approx_f32(q.magnitude(), 1.0f));

    Quaternion q2 = Quaternion::from_sv(2.0f, 0.0f, 0.0f, 0.0f);
    REQUIRE(approx_f32(q2.magnitude(), 2.0f));
}

TEST_CASE("test_quaternion_rotate_around_x") {
    Vector axis(1.0f, 0.0f, 0.0f);
    float angle = PI / 2.0f;
    Quaternion q = Quaternion::from_axis_angle(axis, angle);

    Vector v(0.0f, 1.0f, 0.0f);
    Vector rotated = q.rotate_vector(v);

    Vector expected(0.0f, 0.0f, 1.0f);
    REQUIRE(vectors_close(rotated, expected));
}

TEST_CASE("test_quaternion_rotate_around_y") {
    Vector axis(0.0f, 1.0f, 0.0f);
    float angle = PI / 2.0f;
    Quaternion q = Quaternion::from_axis_angle(axis, angle);

    Vector v(0.0f, 0.0f, 1.0f);
    Vector rotated = q.rotate_vector(v);

    Vector expected(1.0f, 0.0f, 0.0f);
    REQUIRE(vectors_close(rotated, expected));
}

TEST_CASE("test_quaternion_to_json_from_json") {
    Vector axis(0.0f, 0.0f, 1.0f);
    float angle = PI / 4.0f;
    Quaternion orig = Quaternion::from_axis_angle(axis, angle);

    std::string filepath = "../test_quaternion.json";
    orig.to_json(filepath);
    Quaternion loaded = Quaternion::from_json(filepath);

    REQUIRE(approx_f32(loaded.s, orig.s));
    REQUIRE(vectors_close(loaded.v, orig.v));
}
