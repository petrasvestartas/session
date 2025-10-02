#include "catch/include/catch_amalgamated.hpp"
#include "xform.hpp"
#include <cmath>

namespace session_cpp {

static bool approx_f32(float a, float b) {
    return std::abs(static_cast<double>(a) - static_cast<double>(b)) < 1e-5;
}

static bool matrices_close(const Xform& a, const Xform& b) {
    for (int i = 0; i < 16; i++) {
        if (!approx_f32(a.m[i], b.m[i])) return false;
    }
    return true;
}

TEST_CASE("test_xform_identity") {
    Xform id = Xform::identity();
    REQUIRE(id.is_identity());
}

TEST_CASE("test_xform_default") {
    Xform def;
    REQUIRE(def.is_identity());
}

TEST_CASE("test_xform_identity_transformed_point") {
    Point p(1.0f, 2.0f, 3.0f);
    Point t = Xform::identity().transformed_point(p);
    REQUIRE(t.x() == 1.0f);
    REQUIRE(t.y() == 2.0f);
    REQUIRE(t.z() == 3.0f);
}

TEST_CASE("test_xform_translation_point") {
    Xform t = Xform::translation(1.0f, 2.0f, 3.0f);
    Point p(4.0f, 5.0f, 6.0f);
    Point tp = t.transformed_point(p);
    REQUIRE(tp.x() == 5.0f);
    REQUIRE(tp.y() == 7.0f);
    REQUIRE(tp.z() == 9.0f);
}

TEST_CASE("test_xform_translation_vector") {
    Xform t = Xform::translation(1.0f, 2.0f, 3.0f);
    Vector v(1.0f, 2.0f, 3.0f);
    Vector tv = t.transformed_vector(v);
    REQUIRE(tv[0] == 1.0f);
    REQUIRE(tv[1] == 2.0f);
    REQUIRE(tv[2] == 3.0f);
}

TEST_CASE("test_xform_scaling_point") {
    Xform s = Xform::scaling(2.0f, 3.0f, 4.0f);
    Point p(1.0f, -2.0f, 0.5f);
    Point sp = s.transformed_point(p);
    REQUIRE(sp.x() == 2.0f);
    REQUIRE(sp.y() == -6.0f);
    REQUIRE(sp.z() == 2.0f);
}

TEST_CASE("test_xform_scaling_vector") {
    Xform s = Xform::scaling(2.0f, 3.0f, 4.0f);
    Vector v(1.0f, -2.0f, 0.5f);
    Vector sv = s.transformed_vector(v);
    REQUIRE(sv[0] == 2.0f);
    REQUIRE(sv[1] == -6.0f);
    REQUIRE(sv[2] == 2.0f);
}

TEST_CASE("test_xform_rotation_z") {
    Xform r = Xform::rotation_z(M_PI / 2.0f);
    Point p(1.0f, 0.0f, 0.0f);
    Point rp = r.transformed_point(p);
    REQUIRE(approx_f32(rp.x(), 0.0f));
    REQUIRE(approx_f32(rp.y(), 1.0f));
    REQUIRE(approx_f32(rp.z(), 0.0f));
}

TEST_CASE("test_xform_axis_rotation") {
    Vector axis(0.0f, 0.0f, 1.0f);
    Xform r1 = Xform::rotation_z(M_PI / 2.0f);
    Xform r2 = Xform::axis_rotation(M_PI / 2.0f, axis);
    Point p(1.0f, 0.0f, 0.0f);
    Point p1 = r1.transformed_point(p);
    Point p2 = r2.transformed_point(p);
    REQUIRE(approx_f32(p1.x(), p2.x()));
    REQUIRE(approx_f32(p1.y(), p2.y()));
    REQUIRE(approx_f32(p1.z(), p2.z()));
}

TEST_CASE("test_xform_inverse") {
    Xform t = Xform::translation(1.0f, 2.0f, 3.0f) * Xform::rotation_z(0.7f) * Xform::scaling(2.0f, 2.0f, 2.0f);
    Xform inv = t.inverse().value();
    Xform id = t * inv;
    REQUIRE(matrices_close(id, Xform::identity()));
}

TEST_CASE("test_xform_change_basis_alt_identity") {
    Point o0(0.0f, 0.0f, 0.0f);
    Point o1(0.0f, 0.0f, 0.0f);
    Vector x(1.0f, 0.0f, 0.0f);
    Vector y(0.0f, 1.0f, 0.0f);
    Vector z(0.0f, 0.0f, 1.0f);
    Xform cb = Xform::change_basis_alt(o1, x, y, z, o0, x, y, z);
    REQUIRE(cb.is_identity());
}

TEST_CASE("test_xform_change_basis_alt_translation") {
    Point o0(4.0f, 5.0f, 6.0f);
    Point o1(1.0f, 2.0f, 3.0f);
    Vector x(1.0f, 0.0f, 0.0f);
    Vector y(0.0f, 1.0f, 0.0f);
    Vector z(0.0f, 0.0f, 1.0f);
    Xform cb = Xform::change_basis_alt(o1, x, y, z, o0, x, y, z);
    Point p(1.0f, 1.0f, 1.0f);
    Point tp = cb.transformed_point(p);
    REQUIRE(approx_f32(tp.x(), p.x() + 3.0f));
    REQUIRE(approx_f32(tp.y(), p.y() + 3.0f));
    REQUIRE(approx_f32(tp.z(), p.z() + 3.0f));
}

TEST_CASE("test_xform_plane_to_plane") {
    Point o0(1.0f, 2.0f, 3.0f);
    Point o1(-2.0f, 0.5f, 7.0f);
    Vector x0(1.0f, 0.0f, 0.0f);
    Vector y0(0.0f, 1.0f, 0.0f);
    Vector z0(0.0f, 0.0f, 1.0f);
    Vector x1(1.0f, 0.0f, 0.0f);
    Vector y1(0.0f, 1.0f, 0.0f);
    Vector z1(0.0f, 0.0f, 1.0f);
    Xform m = Xform::plane_to_plane(o0, x0, y0, z0, o1, x1, y1, z1);
    Point mapped = m.transformed_point(o0);
    REQUIRE(approx_f32(mapped.x(), o1.x()));
    REQUIRE(approx_f32(mapped.y(), o1.y()));
    REQUIRE(approx_f32(mapped.z(), o1.z()));
}

TEST_CASE("test_xform_mul") {
    Xform a = Xform::translation(1.0f, 2.0f, 3.0f);
    Xform b = Xform::scaling(2.0f, 3.0f, 4.0f);
    Xform r_ref = a * b;
    Xform r_owned = a * b;
    REQUIRE(matrices_close(r_ref, r_owned));
}

TEST_CASE("test_xform_mul_assign") {
    Xform a = Xform::translation(1.0f, 2.0f, 3.0f);
    Xform b = Xform::scaling(2.0f, 3.0f, 4.0f);
    Xform acc = Xform::identity();
    acc *= a;
    acc *= b;
    Xform r2 = Xform::identity() * (Xform::translation(1.0f, 2.0f, 3.0f) * Xform::scaling(2.0f, 3.0f, 4.0f));
    REQUIRE(matrices_close(acc, r2));
}

TEST_CASE("test_xform_json_round_trip") {
    Xform x = Xform::from_matrix({1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 4.0f, 5.0f, 6.0f, 1.0f});
    nlohmann::json data = x.to_json_data();
    Xform y = Xform::from_json_data(data);
    REQUIRE(matrices_close(x, y));
}

TEST_CASE("test_xform_from_matrix") {
    std::array<float, 16> m = {1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 5.0f, 10.0f, 15.0f, 1.0f};
    Xform x = Xform::from_matrix(m);
    REQUIRE(x.m == m);
}

TEST_CASE("test_xform_rotation_x") {
    Xform r = Xform::rotation_x(M_PI / 2.0f);
    Point p(0.0f, 1.0f, 0.0f);
    Point rp = r.transformed_point(p);
    REQUIRE(approx_f32(rp.x(), 0.0f));
    REQUIRE(approx_f32(rp.y(), 0.0f));
    REQUIRE(approx_f32(rp.z(), 1.0f));
}

TEST_CASE("test_xform_rotation_y") {
    Xform r = Xform::rotation_y(M_PI / 2.0f);
    Point p(1.0f, 0.0f, 0.0f);
    Point rp = r.transformed_point(p);
    REQUIRE(approx_f32(rp.x(), 0.0f));
    REQUIRE(approx_f32(rp.y(), 0.0f));
    REQUIRE(approx_f32(rp.z(), -1.0f));
}

TEST_CASE("test_xform_rotation") {
    Vector axis(0.0f, 0.0f, 1.0f);
    Xform r = Xform::rotation(axis, M_PI / 2.0f);
    Point p(1.0f, 0.0f, 0.0f);
    Point rp = r.transformed_point(p);
    REQUIRE(approx_f32(rp.x(), 0.0f));
    REQUIRE(approx_f32(rp.y(), 1.0f));
    REQUIRE(approx_f32(rp.z(), 0.0f));
}

TEST_CASE("test_xform_change_basis") {
    Point o(1.0f, 2.0f, 3.0f);
    Vector x(1.0f, 0.0f, 0.0f);
    Vector y(0.0f, 1.0f, 0.0f);
    Vector z(0.0f, 0.0f, 1.0f);
    Xform cb = Xform::change_basis(o, x, y, z);
    REQUIRE(approx_f32(cb.m[12], 1.0f));
    REQUIRE(approx_f32(cb.m[13], 2.0f));
    REQUIRE(approx_f32(cb.m[14], 3.0f));
}

TEST_CASE("test_xform_plane_to_xy") {
    Point o(1.0f, 2.0f, 3.0f);
    Vector x(1.0f, 0.0f, 0.0f);
    Vector y(0.0f, 1.0f, 0.0f);
    Vector z(0.0f, 0.0f, 1.0f);
    Xform m = Xform::plane_to_xy(o, x, y, z);
    Point mapped = m.transformed_point(o);
    REQUIRE(approx_f32(mapped.x(), 0.0f));
    REQUIRE(approx_f32(mapped.y(), 0.0f));
    REQUIRE(approx_f32(mapped.z(), 0.0f));
}

TEST_CASE("test_xform_xy_to_plane") {
    Point o(1.0f, 2.0f, 3.0f);
    Vector x(1.0f, 0.0f, 0.0f);
    Vector y(0.0f, 1.0f, 0.0f);
    Vector z(0.0f, 0.0f, 1.0f);
    Xform m = Xform::xy_to_plane(o, x, y, z);
    Point origin(0.0f, 0.0f, 0.0f);
    Point mapped = m.transformed_point(origin);
    REQUIRE(approx_f32(mapped.x(), o.x()));
    REQUIRE(approx_f32(mapped.y(), o.y()));
    REQUIRE(approx_f32(mapped.z(), o.z()));
}

TEST_CASE("test_xform_scale_xyz") {
    Xform s = Xform::scale_xyz(2.0f, 3.0f, 4.0f);
    Point p(1.0f, 1.0f, 1.0f);
    Point sp = s.transformed_point(p);
    REQUIRE(sp.x() == 2.0f);
    REQUIRE(sp.y() == 3.0f);
    REQUIRE(sp.z() == 4.0f);
}

TEST_CASE("test_xform_scale_uniform") {
    Point o(1.0f, 1.0f, 1.0f);
    Xform s = Xform::scale_uniform(o, 2.0f);
    Point p(2.0f, 2.0f, 2.0f);
    Point sp = s.transformed_point(p);
    REQUIRE(approx_f32(sp.x(), 3.0f));
    REQUIRE(approx_f32(sp.y(), 3.0f));
    REQUIRE(approx_f32(sp.z(), 3.0f));
}

TEST_CASE("test_xform_scale_non_uniform") {
    Point o(0.0f, 0.0f, 0.0f);
    Xform s = Xform::scale_non_uniform(o, 2.0f, 3.0f, 4.0f);
    Point p(1.0f, 1.0f, 1.0f);
    Point sp = s.transformed_point(p);
    REQUIRE(sp.x() == 2.0f);
    REQUIRE(sp.y() == 3.0f);
    REQUIRE(sp.z() == 4.0f);
}

TEST_CASE("test_xform_is_identity") {
    Xform x = Xform::identity();
    REQUIRE(x.is_identity());
    x.m[0] = 2.0f;
    REQUIRE(!x.is_identity());
}

TEST_CASE("test_xform_transformed_point") {
    Xform t = Xform::translation(1.0f, 2.0f, 3.0f);
    Point p(0.0f, 0.0f, 0.0f);
    Point tp = t.transformed_point(p);
    REQUIRE(tp.x() == 1.0f);
    REQUIRE(tp.y() == 2.0f);
    REQUIRE(tp.z() == 3.0f);
}

TEST_CASE("test_xform_transformed_vector") {
    Xform s = Xform::scaling(2.0f, 3.0f, 4.0f);
    Vector v(1.0f, 1.0f, 1.0f);
    Vector sv = s.transformed_vector(v);
    REQUIRE(sv[0] == 2.0f);
    REQUIRE(sv[1] == 3.0f);
    REQUIRE(sv[2] == 4.0f);
}

TEST_CASE("test_xform_transform_point") {
    Xform t = Xform::translation(1.0f, 2.0f, 3.0f);
    Point p(0.0f, 0.0f, 0.0f);
    t.transform_point(p);
    REQUIRE(p.x() == 1.0f);
    REQUIRE(p.y() == 2.0f);
    REQUIRE(p.z() == 3.0f);
}

TEST_CASE("test_xform_transform_vector") {
    Xform s = Xform::scaling(2.0f, 3.0f, 4.0f);
    Vector v(1.0f, 1.0f, 1.0f);
    s.transform_vector(v);
    REQUIRE(v[0] == 2.0f);
    REQUIRE(v[1] == 3.0f);
    REQUIRE(v[2] == 4.0f);
}

TEST_CASE("test_xform_to_json_data") {
    Xform x = Xform::identity();
    x.name = "test_matrix";
    nlohmann::json data = x.to_json_data();
    REQUIRE(data["name"] == "test_matrix");
    REQUIRE(data["type"] == "Xform");
    REQUIRE(data["m"].size() == 16);
}

TEST_CASE("test_xform_from_json_data") {
    nlohmann::json data;
    data["type"] = "Xform";
    data["guid"] = "test-guid";
    data["name"] = "test_matrix";
    data["m"] = std::vector<float>{1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f};
    Xform x = Xform::from_json_data(data);
    REQUIRE(x.name == "test_matrix");
    REQUIRE(x.guid == "test-guid");
}

TEST_CASE("test_xform_to_json_from_json") {
    Xform x = Xform::translation(1.0f, 2.0f, 3.0f);
    x.name = "test_file";
    std::string filepath = "test_xform_file_cpp.json";
    x.to_json(filepath);
    Xform y = Xform::from_json(filepath);
    REQUIRE(y.name == "test_file");
    REQUIRE(matrices_close(x, y));
    std::remove(filepath.c_str());
}

TEST_CASE("test_xform_getitem") {
    Xform x = Xform::identity();
    REQUIRE(x(0, 0) == 1.0f);
    REQUIRE(x(1, 1) == 1.0f);
    REQUIRE(x(2, 2) == 1.0f);
    REQUIRE(x(3, 3) == 1.0f);
    REQUIRE(x(0, 3) == 0.0f);
}

TEST_CASE("test_xform_setitem") {
    Xform x = Xform::identity();
    x(0, 3) = 5.0f;
    x(1, 3) = 10.0f;
    x(2, 3) = 15.0f;
    REQUIRE(x(0, 3) == 5.0f);
    REQUIRE(x(1, 3) == 10.0f);
    REQUIRE(x(2, 3) == 15.0f);
}

}
