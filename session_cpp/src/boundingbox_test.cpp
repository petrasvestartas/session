#include "catch_amalgamated.hpp"
#include "boundingbox.h"
#include "line.h"
#include "polyline.h"

using namespace session_cpp;

TEST_CASE("BoundingBox default constructor") {
    BoundingBox boundingbox;
    REQUIRE(boundingbox.center.x() == 0.0f);
    REQUIRE(boundingbox.center.y() == 0.0f);
    REQUIRE(boundingbox.center.z() == 0.0f);
    REQUIRE(boundingbox.x_axis.x() == 1.0f);
    REQUIRE(boundingbox.y_axis.y() == 1.0f);
    REQUIRE(boundingbox.z_axis.z() == 1.0f);
    REQUIRE(boundingbox.half_size.x() == 0.5f);
    REQUIRE(boundingbox.half_size.y() == 0.5f);
    REQUIRE(boundingbox.half_size.z() == 0.5f);
    REQUIRE(!boundingbox.guid.empty());
}

TEST_CASE("BoundingBox constructor with parameters") {
    Point center(1.0f, 2.0f, 3.0f);
    Vector x_axis(1.0f, 0.0f, 0.0f);
    Vector y_axis(0.0f, 1.0f, 0.0f);
    Vector z_axis(0.0f, 0.0f, 1.0f);
    Vector half_size(2.0f, 3.0f, 4.0f);
    
    BoundingBox boundingbox(center, x_axis, y_axis, z_axis, half_size);
    
    REQUIRE(boundingbox.center.x() == 1.0f);
    REQUIRE(boundingbox.center.y() == 2.0f);
    REQUIRE(boundingbox.center.z() == 3.0f);
    REQUIRE(boundingbox.half_size.x() == 2.0f);
    REQUIRE(boundingbox.half_size.y() == 3.0f);
    REQUIRE(boundingbox.half_size.z() == 4.0f);
}

TEST_CASE("BoundingBox constructor from plane") {
    Point origin(0.0f, 0.0f, 0.0f);
    Vector x_axis(1.0f, 0.0f, 0.0f);
    Vector y_axis(0.0f, 1.0f, 0.0f);
    Plane plane(origin, x_axis, y_axis);
    BoundingBox boundingbox(plane, 4.0f, 6.0f, 8.0f);
    
    REQUIRE(boundingbox.center.x() == 0.0f);
    REQUIRE(boundingbox.half_size.x() == 2.0f);
    REQUIRE(boundingbox.half_size.y() == 3.0f);
    REQUIRE(boundingbox.half_size.z() == 4.0f);
}

TEST_CASE("BoundingBox point_at") {
    BoundingBox boundingbox(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    Point pt = boundingbox.point_at(1.0f, 1.0f, 1.0f);
    REQUIRE(pt.x() == 1.0f);
    REQUIRE(pt.y() == 1.0f);
    REQUIRE(pt.z() == 1.0f);
}

TEST_CASE("BoundingBox corners") {
    BoundingBox boundingbox(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    auto corners = boundingbox.corners();
    
    REQUIRE(corners.size() == 8);
    REQUIRE(corners[0].x() == 1.0f);
    REQUIRE(corners[0].y() == 1.0f);
    REQUIRE(corners[0].z() == -1.0f);
}

TEST_CASE("BoundingBox two_rectangles") {
    BoundingBox boundingbox(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    auto rects = boundingbox.two_rectangles();
    
    REQUIRE(rects.size() == 10);
    REQUIRE(rects[0].x() == rects[4].x());
    REQUIRE(rects[0].y() == rects[4].y());
    REQUIRE(rects[0].z() == rects[4].z());
}

TEST_CASE("BoundingBox inflate") {
    BoundingBox boundingbox(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 2.0f, 3.0f));
    boundingbox.inflate(0.5f);
    
    REQUIRE(boundingbox.half_size.x() == 1.5f);
    REQUIRE(boundingbox.half_size.y() == 2.5f);
    REQUIRE(boundingbox.half_size.z() == 3.5f);
}

TEST_CASE("BoundingBox collides_with overlapping") {
    BoundingBox box1(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    BoundingBox box2(Point(0.5f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    
    REQUIRE(box1.collides_with(box2));
}

TEST_CASE("BoundingBox collides_with separated") {
    BoundingBox box1(Point(0.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    BoundingBox box2(Point(5.0f, 0.0f, 0.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(1.0f, 1.0f, 1.0f));
    
    REQUIRE(!box1.collides_with(box2));
}

TEST_CASE("BoundingBox to_json_data") {
    BoundingBox boundingbox(Point(1.0f, 2.0f, 3.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(2.0f, 3.0f, 4.0f));
    
    auto data = boundingbox.to_json_data();
    
    REQUIRE(data.contains("center"));
    REQUIRE(data.contains("x_axis"));
    REQUIRE(data.contains("y_axis"));
    REQUIRE(data.contains("z_axis"));
    REQUIRE(data.contains("half_size"));
    REQUIRE(data.contains("guid"));
}

TEST_CASE("BoundingBox from_json_data") {
    BoundingBox original(Point(1.0f, 2.0f, 3.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(2.0f, 3.0f, 4.0f));
    
    auto data = original.to_json_data();
    BoundingBox loaded = BoundingBox::from_json_data(data);
    
    REQUIRE(loaded.center.x() == original.center.x());
    REQUIRE(loaded.center.y() == original.center.y());
    REQUIRE(loaded.center.z() == original.center.z());
    REQUIRE(loaded.half_size.x() == original.half_size.x());
    REQUIRE(loaded.half_size.y() == original.half_size.y());
    REQUIRE(loaded.half_size.z() == original.half_size.z());
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.guid == original.guid);
}

TEST_CASE("BoundingBox to_json from_json") {
    BoundingBox original(Point(1.0f, 2.0f, 3.0f), Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(2.0f, 3.0f, 4.0f));
    std::string filename = "../test_boundingbox.json";
    
    original.to_json_file(filename);
    BoundingBox loaded = BoundingBox::from_json_file(filename);
    
    REQUIRE(loaded.center.x() == original.center.x());
    REQUIRE(loaded.center.y() == original.center.y());
    REQUIRE(loaded.center.z() == original.center.z());
    REQUIRE(loaded.half_size.x() == original.half_size.x());
    REQUIRE(loaded.half_size.y() == original.half_size.y());
    REQUIRE(loaded.half_size.z() == original.half_size.z());
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.guid == original.guid);
}

TEST_CASE("BoundingBox from_point") {
    Point pt(1.0f, 2.0f, 3.0f);
    BoundingBox boundingbox = BoundingBox::from_point(pt);
    
    REQUIRE(boundingbox.center.x() == 1.0f);
    REQUIRE(boundingbox.center.y() == 2.0f);
    REQUIRE(boundingbox.center.z() == 3.0f);
    REQUIRE(boundingbox.half_size.x() == 0.0f);
    REQUIRE(boundingbox.half_size.y() == 0.0f);
    REQUIRE(boundingbox.half_size.z() == 0.0f);
}

TEST_CASE("BoundingBox from_point with inflate") {
    Point pt(1.0f, 2.0f, 3.0f);
    BoundingBox boundingbox = BoundingBox::from_point(pt, 0.5f);
    
    REQUIRE(boundingbox.center.x() == 1.0f);
    REQUIRE(boundingbox.center.y() == 2.0f);
    REQUIRE(boundingbox.center.z() == 3.0f);
    REQUIRE(boundingbox.half_size.x() == 0.5f);
    REQUIRE(boundingbox.half_size.y() == 0.5f);
    REQUIRE(boundingbox.half_size.z() == 0.5f);
}

TEST_CASE("BoundingBox from_points") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(2.0f, 4.0f, 6.0f)
    };
    BoundingBox boundingbox = BoundingBox::from_points(points);
    
    REQUIRE(boundingbox.center.x() == 1.0f);
    REQUIRE(boundingbox.center.y() == 2.0f);
    REQUIRE(boundingbox.center.z() == 3.0f);
    REQUIRE(boundingbox.half_size.x() == 1.0f);
    REQUIRE(boundingbox.half_size.y() == 2.0f);
    REQUIRE(boundingbox.half_size.z() == 3.0f);
}

TEST_CASE("BoundingBox from_points with inflate") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(2.0f, 4.0f, 6.0f)
    };
    BoundingBox boundingbox = BoundingBox::from_points(points, 0.5f);
    
    REQUIRE(boundingbox.center.x() == 1.0f);
    REQUIRE(boundingbox.center.y() == 2.0f);
    REQUIRE(boundingbox.center.z() == 3.0f);
    REQUIRE(boundingbox.half_size.x() == 1.5f);
    REQUIRE(boundingbox.half_size.y() == 2.5f);
    REQUIRE(boundingbox.half_size.z() == 3.5f);
}

TEST_CASE("BoundingBox from_line") {
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 0.0f, 0.0f);
    BoundingBox boundingbox = BoundingBox::from_line(line);
    
    REQUIRE(boundingbox.center.x() == 5.0f);
    REQUIRE(boundingbox.center.y() == 0.0f);
    REQUIRE(boundingbox.center.z() == 0.0f);
    REQUIRE(boundingbox.half_size.x() == 5.0f);
}

TEST_CASE("BoundingBox from_line with inflate") {
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 0.0f, 0.0f);
    BoundingBox boundingbox = BoundingBox::from_line(line, 1.0f);
    
    REQUIRE(boundingbox.center.x() == 5.0f);
    REQUIRE(boundingbox.center.y() == 0.0f);
    REQUIRE(boundingbox.center.z() == 0.0f);
    REQUIRE(boundingbox.half_size.x() == 6.0f);
    REQUIRE(boundingbox.half_size.z() == 1.0f);
}

TEST_CASE("BoundingBox from_polyline") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    Polyline polyline(points);
    BoundingBox boundingbox = BoundingBox::from_polyline(polyline);
    
    REQUIRE(boundingbox.center.x() == 0.5f);
    REQUIRE(boundingbox.center.y() == 0.5f);
    REQUIRE(boundingbox.center.z() == 0.0f);
    REQUIRE(boundingbox.half_size.x() == 0.5f);
    REQUIRE(boundingbox.half_size.y() == 0.5f);
    REQUIRE(boundingbox.half_size.z() == 0.0f);
}

TEST_CASE("BoundingBox from_polyline with inflate") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    Polyline polyline(points);
    BoundingBox boundingbox = BoundingBox::from_polyline(polyline, 0.5f);
    
    REQUIRE(boundingbox.center.x() == 0.5f);
    REQUIRE(boundingbox.center.y() == 0.5f);
    REQUIRE(boundingbox.center.z() == 0.0f);
    REQUIRE(boundingbox.half_size.x() == 1.0f);
    REQUIRE(boundingbox.half_size.y() == 1.0f);
    REQUIRE(boundingbox.half_size.z() == 0.5f);
}
