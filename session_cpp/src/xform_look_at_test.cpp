#include "catch_amalgamated.hpp"
#include "point.h"
#include "vector.h"
#include "xform.hpp"
#include <cmath>

namespace session_cpp {

TEST_CASE("Xform look_at_rh basic") {
    Point eye(0.0f, 0.0f, 5.0f);
    Point target(0.0f, 0.0f, 0.0f);
    Vector up(0.0f, 1.0f, 0.0f);
    
    Xform xform = Xform::look_at_rh(eye, target, up);
    
    REQUIRE(std::abs(xform.m[0] - 1.0f) < 1e-5f);
    REQUIRE(std::abs(xform.m[5] - 1.0f) < 1e-5f);
    REQUIRE(std::abs(xform.m[10] - 1.0f) < 1e-5f);
    REQUIRE(std::abs(xform.m[14] - (-5.0f)) < 1e-5f);
}

TEST_CASE("Xform look_at_rh arbitrary") {
    Point eye(3.0f, 4.0f, 5.0f);
    Point target(1.0f, 2.0f, 3.0f);
    Vector up(0.0f, 1.0f, 0.0f);
    
    Xform xform = Xform::look_at_rh(eye, target, up);
    
    // Verify the matrix creates valid transformation
    // Just check that it's not identity
    bool is_not_identity = std::abs(xform.m[0] - 1.0f) > 1e-5f ||
                           std::abs(xform.m[5] - 1.0f) > 1e-5f ||
                           std::abs(xform.m[10] - 1.0f) > 1e-5f;
    REQUIRE(is_not_identity);
}

TEST_CASE("Xform look_at_rh x axis") {
    Point eye(5.0f, 0.0f, 0.0f);
    Point target(0.0f, 0.0f, 0.0f);
    Vector up(0.0f, 0.0f, 1.0f);
    
    Xform xform = Xform::look_at_rh(eye, target, up);
    
    Point test_point(0.0f, 0.0f, 0.0f);
    Point result = xform.transformed_point(test_point);
    
    REQUIRE(std::abs(result.x()) < 1e-5f);
    REQUIRE(std::abs(result.y()) < 1e-5f);
    REQUIRE(std::abs(result.z() - (-5.0f)) < 1e-5f);
}

TEST_CASE("Xform look_at_rh with inverse") {
    Point eye(0.0f, 0.0f, 10.0f);
    Point target(0.0f, 0.0f, 0.0f);
    Vector up(0.0f, 1.0f, 0.0f);
    
    Xform view = Xform::look_at_rh(eye, target, up);
    auto world_from_cam_opt = view.inverse();
    
    REQUIRE(world_from_cam_opt.has_value());
    
    Xform world_from_cam = world_from_cam_opt.value();
    Xform identity = view * world_from_cam;
    
    REQUIRE(std::abs(identity.m[0] - 1.0f) < 1e-4f);
    REQUIRE(std::abs(identity.m[5] - 1.0f) < 1e-4f);
    REQUIRE(std::abs(identity.m[10] - 1.0f) < 1e-4f);
    REQUIRE(std::abs(identity.m[15] - 1.0f) < 1e-4f);
    
    REQUIRE(std::abs(identity.m[1]) < 1e-4f);
    REQUIRE(std::abs(identity.m[2]) < 1e-4f);
    REQUIRE(std::abs(identity.m[4]) < 1e-4f);
}

TEST_CASE("Xform look_at_rh camera use case") {
    Point position(5.0f, 3.0f, 8.0f);
    Point target(0.0f, 0.0f, 0.0f);
    Vector up(0.0f, 1.0f, 0.0f);
    
    Xform view = Xform::look_at_rh(position, target, up);
    auto world_from_cam_opt = view.inverse();
    
    Xform world_from_cam = world_from_cam_opt.value_or(Xform::identity());
    
    Point cam_origin(0.0f, 0.0f, 0.0f);
    Point world_pos = world_from_cam.transformed_point(cam_origin);
    
    REQUIRE(std::abs(world_pos.x() - position.x()) < 1e-3f);
    REQUIRE(std::abs(world_pos.y() - position.y()) < 1e-3f);
    REQUIRE(std::abs(world_pos.z() - position.z()) < 1e-3f);
}

} // namespace session_cpp
