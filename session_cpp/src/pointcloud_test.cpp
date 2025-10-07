#include "pointcloud.h"
#include "catch_amalgamated.hpp"

using namespace session_cpp;

TEST_CASE("PointCloud new") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f)
    };
    std::vector<Vector> normals = {
        Vector(0.0f, 0.0f, 1.0f),
        Vector(0.0f, 1.0f, 0.0f),
        Vector(1.0f, 0.0f, 0.0f)
    };
    std::vector<Color> colors = {
        Color(255, 0, 0, 255),
        Color(0, 255, 0, 255),
        Color(0, 0, 255, 255)
    };
    PointCloud cloud(points, normals, colors);
    REQUIRE(cloud.size() == 3);
    REQUIRE(!cloud.empty());
}

TEST_CASE("PointCloud default") {
    PointCloud cloud;
    REQUIRE(cloud.size() == 0);
    REQUIRE(cloud.empty());
    REQUIRE(cloud.name() == "my_pointcloud");
}

TEST_CASE("PointCloud operator+= vector") {
    std::vector<Point> points = {Point(1.0f, 2.0f, 3.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    Vector v(4.0f, 5.0f, 6.0f);
    cloud += v;
    
    REQUIRE(cloud.points()[0].x() == 5.0f);
    REQUIRE(cloud.points()[0].y() == 7.0f);
    REQUIRE(cloud.points()[0].z() == 9.0f);
}

TEST_CASE("PointCloud operator+ vector") {
    std::vector<Point> points = {Point(1.0f, 2.0f, 3.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    Vector v(4.0f, 5.0f, 6.0f);
    PointCloud cloud2 = cloud + v;
    
    REQUIRE(cloud2.points()[0].x() == 5.0f);
    REQUIRE(cloud2.points()[0].y() == 7.0f);
    REQUIRE(cloud2.points()[0].z() == 9.0f);
}

TEST_CASE("PointCloud operator-= vector") {
    std::vector<Point> points = {Point(1.0f, 2.0f, 3.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    Vector v(4.0f, 5.0f, 6.0f);
    cloud -= v;
    
    REQUIRE(cloud.points()[0].x() == -3.0f);
    REQUIRE(cloud.points()[0].y() == -3.0f);
    REQUIRE(cloud.points()[0].z() == -3.0f);
}

TEST_CASE("PointCloud operator- vector") {
    std::vector<Point> points = {Point(1.0f, 2.0f, 3.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    Vector v(4.0f, 5.0f, 6.0f);
    PointCloud cloud2 = cloud - v;
    
    REQUIRE(cloud2.points()[0].x() == -3.0f);
    REQUIRE(cloud2.points()[0].y() == -3.0f);
    REQUIRE(cloud2.points()[0].z() == -3.0f);
}

TEST_CASE("PointCloud to_string") {
    std::vector<Point> points = {Point(0.0f, 0.0f, 0.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    std::string str = cloud.to_string();
    REQUIRE(str.find("PointCloud") != std::string::npos);
    REQUIRE(str.find("points=1") != std::string::npos);
}

TEST_CASE("PointCloud JSON serialization") {
    std::vector<Point> points = {Point(1.0f, 2.0f, 3.0f)};
    std::vector<Vector> normals = {Vector(0.0f, 0.0f, 1.0f)};
    std::vector<Color> colors = {Color(255, 0, 0, 255)};
    PointCloud cloud(points, normals, colors);
    
    auto json = cloud.to_json_data();
    PointCloud cloud2 = PointCloud::from_json_data(json);
    
    REQUIRE(cloud2.points()[0].x() == 1.0f);
    REQUIRE(cloud2.points()[0].y() == 2.0f);
    REQUIRE(cloud2.points()[0].z() == 3.0f);
}

TEST_CASE("PointCloud JSON file") {
    std::vector<Point> points = {
        Point(1.0f, 2.0f, 3.0f),
        Point(4.0f, 5.0f, 6.0f),
        Point(7.0f, 8.0f, 9.0f)
    };
    std::vector<Vector> normals = {
        Vector(0.0f, 0.0f, 1.0f),
        Vector(0.0f, 1.0f, 0.0f),
        Vector(1.0f, 0.0f, 0.0f)
    };
    std::vector<Color> colors = {
        Color(255, 0, 0, 255),
        Color(0, 255, 0, 255),
        Color(0, 0, 255, 255)
    };
    PointCloud cloud(points, normals, colors);
    
    cloud.to_json("../test_pointcloud.json");
    PointCloud cloud2 = PointCloud::from_json("../test_pointcloud.json");
    
    REQUIRE(cloud2.size() == 3);
    REQUIRE(cloud2.points()[0].x() == 1.0f);
    REQUIRE(cloud2.points()[1].y() == 5.0f);
    REQUIRE(cloud2.points()[2].z() == 9.0f);
}

TEST_CASE("PointCloud JSON multiple points") {
    std::vector<Point> points = {
        Point(1.0f, 2.0f, 3.0f),
        Point(4.0f, 5.0f, 6.0f),
        Point(7.0f, 8.0f, 9.0f)
    };
    std::vector<Vector> normals = {
        Vector(0.0f, 0.0f, 1.0f),
        Vector(0.0f, 1.0f, 0.0f),
        Vector(1.0f, 0.0f, 0.0f)
    };
    std::vector<Color> colors = {
        Color(255, 0, 0, 255),
        Color(0, 255, 0, 255),
        Color(0, 0, 255, 255)
    };
    PointCloud cloud(points, normals, colors);
    
    auto json = cloud.to_json_data();
    PointCloud cloud2 = PointCloud::from_json_data(json);
    
    REQUIRE(cloud2.size() == 3);
    REQUIRE(cloud2.points()[0].x() == 1.0f);
    REQUIRE(cloud2.points()[1].y() == 5.0f);
    REQUIRE(cloud2.points()[2].z() == 9.0f);
    REQUIRE(cloud2.normals()[0].z() == 1.0f);
    REQUIRE(cloud2.colors()[1].g == 255);
    // Verify alpha is always 255 after deserialization
    REQUIRE(cloud2.colors()[0].a == 255);
    REQUIRE(cloud2.colors()[1].a == 255);
    REQUIRE(cloud2.colors()[2].a == 255);
}
