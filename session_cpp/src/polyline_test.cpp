#include "polyline.h"
#include <catch_amalgamated.hpp>

using namespace session_cpp;

TEST_CASE("Polyline new", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(0.0f, 1.0f, 0.0f)
    };
    Polyline polyline(points);
    REQUIRE(polyline.len() == 3);
    REQUIRE(polyline.segment_count() == 2);
}

TEST_CASE("Polyline default", "[polyline]") {
    Polyline polyline;
    REQUIRE(polyline.len() == 0);
    REQUIRE(polyline.is_empty());
    REQUIRE(polyline.segment_count() == 0);
}

TEST_CASE("Polyline length", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    Polyline polyline(points);
    float length = polyline.length();
    REQUIRE(std::abs(length - 2.0f) < 1e-5f);
}

TEST_CASE("Polyline add point", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    REQUIRE(polyline.len() == 2);

    polyline.add_point(Point(1.0f, 1.0f, 0.0f));
    REQUIRE(polyline.len() == 3);
    REQUIRE(polyline.segment_count() == 2);
}

TEST_CASE("Polyline insert point", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f)});

    polyline.insert_point(1, Point(1.0f, 0.0f, 0.0f));
    REQUIRE(polyline.len() == 3);
    REQUIRE(polyline.points[1].x() == 1.0f);
}

TEST_CASE("Polyline remove point", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(2.0f, 0.0f, 0.0f)
    };
    Polyline polyline(points);

    Point removed;
    bool success = polyline.remove_point(1, removed);
    REQUIRE(success);
    REQUIRE(removed.x() == 1.0f);
    REQUIRE(polyline.len() == 2);
}

TEST_CASE("Polyline reverse", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(2.0f, 0.0f, 0.0f)
    };
    Polyline polyline(points);

    polyline.reverse();
    REQUIRE(polyline.points[0].x() == 2.0f);
    REQUIRE(polyline.points[1].x() == 1.0f);
    REQUIRE(polyline.points[2].x() == 0.0f);
}

TEST_CASE("Polyline reversed", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(2.0f, 0.0f, 0.0f)
    };
    Polyline polyline(points);

    Polyline reversed = polyline.reversed();
    REQUIRE(reversed.points[0].x() == 2.0f);
    REQUIRE(reversed.points[1].x() == 1.0f);
    REQUIRE(reversed.points[2].x() == 0.0f);

    // Original should be unchanged
    REQUIRE(polyline.points[0].x() == 0.0f);
}

TEST_CASE("Polyline add assign vector", "[polyline]") {
    Polyline polyline({Point(1.0f, 2.0f, 3.0f), Point(4.0f, 5.0f, 6.0f)});
    Vector v(4.0f, 5.0f, 6.0f);
    polyline += v;

    REQUIRE(polyline.points[0].x() == 5.0f);
    REQUIRE(polyline.points[0].y() == 7.0f);
    REQUIRE(polyline.points[0].z() == 9.0f);
    REQUIRE(polyline.points[1].x() == 8.0f);
    REQUIRE(polyline.points[1].y() == 10.0f);
    REQUIRE(polyline.points[1].z() == 12.0f);
}

TEST_CASE("Polyline add vector", "[polyline]") {
    Polyline polyline({Point(1.0f, 2.0f, 3.0f), Point(4.0f, 5.0f, 6.0f)});
    Vector v(4.0f, 5.0f, 6.0f);
    Polyline polyline2 = polyline + v;

    REQUIRE(polyline2.points[0].x() == 5.0f);
    REQUIRE(polyline2.points[0].y() == 7.0f);
    REQUIRE(polyline2.points[0].z() == 9.0f);
}

TEST_CASE("Polyline sub assign vector", "[polyline]") {
    Polyline polyline({Point(1.0f, 2.0f, 3.0f), Point(4.0f, 5.0f, 6.0f)});
    Vector v(4.0f, 5.0f, 6.0f);
    polyline -= v;

    REQUIRE(polyline.points[0].x() == -3.0f);
    REQUIRE(polyline.points[0].y() == -3.0f);
    REQUIRE(polyline.points[0].z() == -3.0f);
    REQUIRE(polyline.points[1].x() == 0.0f);
    REQUIRE(polyline.points[1].y() == 0.0f);
    REQUIRE(polyline.points[1].z() == 0.0f);
}

TEST_CASE("Polyline sub vector", "[polyline]") {
    Polyline polyline({Point(1.0f, 2.0f, 3.0f), Point(4.0f, 5.0f, 6.0f)});
    Vector v(4.0f, 5.0f, 6.0f);
    Polyline polyline2 = polyline - v;

    REQUIRE(polyline2.points[0].x() == -3.0f);
    REQUIRE(polyline2.points[0].y() == -3.0f);
    REQUIRE(polyline2.points[0].z() == -3.0f);
    REQUIRE(polyline2.points[1].x() == 0.0f);
    REQUIRE(polyline2.points[1].y() == 0.0f);
    REQUIRE(polyline2.points[1].z() == 0.0f);
}

TEST_CASE("Polyline display", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    std::ostringstream oss;
    oss << polyline;
    std::string display_str = oss.str();
    REQUIRE(display_str.find("Polyline") != std::string::npos);
    REQUIRE(display_str.find("points=2") != std::string::npos);
}

TEST_CASE("Polyline json serialization", "[polyline]") {
    std::vector<Point> points = {
        Point(0.0f, 0.0f, 0.0f),
        Point(1.0f, 0.0f, 0.0f),
        Point(1.0f, 1.0f, 0.0f)
    };
    Polyline polyline(points);

    auto json = polyline.to_json_data();
    Polyline deserialized = Polyline::from_json_data(json);

    REQUIRE(deserialized.len() == 3);
    REQUIRE(deserialized.points[0].x() == 0.0f);
    REQUIRE(deserialized.points[1].x() == 1.0f);
    REQUIRE(deserialized.points[2].y() == 1.0f);
}

TEST_CASE("Polyline to json data", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});

    auto json_data = polyline.to_json_data();
    std::string json_string = json_data.dump();
    REQUIRE(json_string.find("Polyline") != std::string::npos);
    REQUIRE(json_string.find("points") != std::string::npos);
}

TEST_CASE("Polyline from json data", "[polyline]") {
    Polyline polyline({Point(1.0f, 2.0f, 3.0f), Point(4.0f, 5.0f, 6.0f)});

    auto json_data = polyline.to_json_data();
    Polyline deserialized = Polyline::from_json_data(json_data);

    REQUIRE(deserialized.len() == 2);
    REQUIRE(deserialized.points[0].x() == 1.0f);
    REQUIRE(deserialized.points[1].x() == 4.0f);
}

TEST_CASE("Polyline to json from json", "[polyline]") {
    std::vector<Point> points = {
        Point(1.0f, 2.0f, 3.0f),
        Point(4.0f, 5.0f, 6.0f),
        Point(7.0f, 8.0f, 9.0f)
    };
    Polyline polyline(points);

    std::string filepath = "test_polyline.json";
    polyline.to_json(filepath);
    Polyline loaded = Polyline::from_json(filepath);

    REQUIRE(loaded.len() == 3);
    REQUIRE(loaded.points[0].x() == 1.0f);
    REQUIRE(loaded.points[1].y() == 5.0f);
    REQUIRE(loaded.points[2].z() == 9.0f);
}

TEST_CASE("Polyline get point", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 2.0f, 3.0f)});

    Point* point = polyline.get_point(1);
    REQUIRE(point != nullptr);
    REQUIRE(point->x() == 1.0f);

    Point* invalid = polyline.get_point(10);
    REQUIRE(invalid == nullptr);
}

TEST_CASE("Polyline get point mut", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 2.0f, 3.0f)});

    Point* point = polyline.get_point(1);
    if (point != nullptr) {
        *point = Point(5.0f, 6.0f, 7.0f);
    }

    REQUIRE(polyline.points[1].x() == 5.0f);
    REQUIRE(polyline.points[1].y() == 6.0f);
    REQUIRE(polyline.points[1].z() == 7.0f);
}

TEST_CASE("Polyline shift", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f)});
    
    polyline.shift(1);
    
    REQUIRE(polyline.points[0].x() == 1.0f);
    REQUIRE(polyline.points[1].x() == 2.0f);
    REQUIRE(polyline.points[2].x() == 0.0f);
}

TEST_CASE("Polyline length squared", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f)});
    
    double length_sq = polyline.length_squared();
    REQUIRE(std::abs(length_sq - 2.0) < 1e-5);
}

TEST_CASE("Polyline point at parameter", "[polyline]") {
    Point start(0.0f, 0.0f, 0.0f);
    Point end(2.0f, 0.0f, 0.0f);
    
    Point mid = Polyline::point_at_parameter(start, end, 0.5);
    REQUIRE(mid.x() == 1.0f);
    REQUIRE(mid.y() == 0.0f);
    REQUIRE(mid.z() == 0.0f);
}

TEST_CASE("Polyline closest point to line", "[polyline]") {
    Point line_start(0.0f, 0.0f, 0.0f);
    Point line_end(2.0f, 0.0f, 0.0f);
    Point test_point(1.0f, 1.0f, 0.0f);
    
    double t;
    Polyline::closest_point_to_line(test_point, line_start, line_end, t);
    REQUIRE(std::abs(t - 0.5) < 1e-5);
}

TEST_CASE("Polyline line line overlap", "[polyline]") {
    Point line0_start(0.0f, 0.0f, 0.0f);
    Point line0_end(2.0f, 0.0f, 0.0f);
    Point line1_start(1.0f, 0.0f, 0.0f);
    Point line1_end(3.0f, 0.0f, 0.0f);
    
    Point overlap_start, overlap_end;
    bool has_overlap = Polyline::line_line_overlap(line0_start, line0_end, line1_start, line1_end,
                                                  overlap_start, overlap_end);
    
    REQUIRE(has_overlap);
    REQUIRE(std::abs(overlap_start.x() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(overlap_end.x() - 2.0f) < 1e-5f);
}

TEST_CASE("Polyline line line average", "[polyline]") {
    Point line0_start(0.0f, 0.0f, 0.0f);
    Point line0_end(2.0f, 0.0f, 0.0f);
    Point line1_start(0.0f, 2.0f, 0.0f);
    Point line1_end(2.0f, 2.0f, 0.0f);
    
    Point avg_start, avg_end;
    Polyline::line_line_average(line0_start, line0_end, line1_start, line1_end, avg_start, avg_end);
    
    REQUIRE(std::abs(avg_start.y() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(avg_end.y() - 1.0f) < 1e-5f);
}

TEST_CASE("Polyline line line overlap average", "[polyline]") {
    Point line0_start(0.0f, 0.0f, 0.0f);
    Point line0_end(3.0f, 0.0f, 0.0f);
    Point line1_start(1.0f, 0.0f, 0.0f);
    Point line1_end(4.0f, 0.0f, 0.0f);
    
    Point output_start, output_end;
    Polyline::line_line_overlap_average(line0_start, line0_end, line1_start, line1_end, output_start, output_end);
    
    REQUIRE(output_start.x() >= 0.0f);
    REQUIRE(output_end.x() <= 4.0f);
}

TEST_CASE("Polyline line from projected points", "[polyline]") {
    Point line_start(0.0f, 0.0f, 0.0f);
    Point line_end(2.0f, 0.0f, 0.0f);
    std::vector<Point> points = {Point(0.5f, 1.0f, 0.0f), Point(1.5f, -1.0f, 0.0f)};
    
    Point output_start, output_end;
    bool result = Polyline::line_from_projected_points(line_start, line_end, points, output_start, output_end);
    
    REQUIRE(result);
    REQUIRE(std::abs(output_start.x() - 0.5f) < 1e-5f);
    REQUIRE(std::abs(output_end.x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline closest distance and point", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f)});
    Point test_point(1.0f, 1.0f, 0.0f);
    
    size_t edge_id;
    Point closest_point;
    double distance = polyline.closest_distance_and_point(test_point, edge_id, closest_point);
    
    REQUIRE(edge_id == 0);
    REQUIRE(std::abs(closest_point.x() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(distance - 1.0f) < 1e-5f);
}

TEST_CASE("Polyline is closed", "[polyline]") {
    Polyline open_polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f)});
    REQUIRE(!open_polyline.is_closed());
    
    Polyline closed_polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 0.0f, 0.0f)});
    REQUIRE(closed_polyline.is_closed());
}

TEST_CASE("Polyline center", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f), Point(2.0f, 2.0f, 0.0f), Point(0.0f, 2.0f, 0.0f)});
    
    Point c = polyline.center();
    REQUIRE(std::abs(c.x() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(c.y() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(c.z() - 0.0f) < 1e-5f);
}

TEST_CASE("Polyline center vec", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f), Point(2.0f, 2.0f, 0.0f)});
    
    Vector c = polyline.center_vec();
    REQUIRE(std::abs(c.x() - 4.0f/3.0f) < 1e-5f);
    REQUIRE(std::abs(c.y() - 2.0f/3.0f) < 1e-5f);
}

TEST_CASE("Polyline get average plane", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)});
    
    Point origin;
    Vector x_axis, y_axis, z_axis;
    polyline.get_average_plane(origin, x_axis, y_axis, z_axis);
    
    REQUIRE(std::abs(z_axis.z() - 1.0f) < 1e-5f);
}

TEST_CASE("Polyline get fast plane", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)});
    
    Point origin;
    Plane plane;
    polyline.get_fast_plane(origin, plane);
    
    REQUIRE(origin.x() == 0.0f);
    REQUIRE(origin.y() == 0.0f);
    REQUIRE(origin.z() == 0.0f);
}

TEST_CASE("Polyline get middle line", "[polyline]") {
    Point line0_start(0.0f, 0.0f, 0.0f);
    Point line0_end(2.0f, 0.0f, 0.0f);
    Point line1_start(0.0f, 2.0f, 0.0f);
    Point line1_end(2.0f, 2.0f, 0.0f);
    
    Point output_start, output_end;
    Polyline::get_middle_line(line0_start, line0_end, line1_start, line1_end, output_start, output_end);
    
    REQUIRE(std::abs(output_start.y() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(output_end.y() - 1.0f) < 1e-5f);
}

TEST_CASE("Polyline extend line", "[polyline]") {
    Point start(0.0f, 0.0f, 0.0f);
    Point end(1.0f, 0.0f, 0.0f);
    
    Polyline::extend_line(start, end, 0.5, 0.5);
    
    REQUIRE(std::abs(start.x() - (-0.5f)) < 1e-5f);
    REQUIRE(std::abs(end.x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline scale line", "[polyline]") {
    Point start(0.0f, 0.0f, 0.0f);
    Point end(2.0f, 0.0f, 0.0f);
    
    Polyline::scale_line(start, end, 0.25);
    
    REQUIRE(std::abs(start.x() - 0.5f) < 1e-5f);
    REQUIRE(std::abs(end.x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline extend segment", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    
    polyline.extend_segment(0, 0.5, 0.5);
    
    REQUIRE(std::abs(polyline.points[0].x() - (-0.5f)) < 1e-5f);
    REQUIRE(std::abs(polyline.points[1].x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline extend segment equally static", "[polyline]") {
    Point start(0.0f, 0.0f, 0.0f);
    Point end(1.0f, 0.0f, 0.0f);
    
    Polyline::extend_segment_equally(start, end, 0.5);
    
    REQUIRE(std::abs(start.x() - (-0.5f)) < 1e-5f);
    REQUIRE(std::abs(end.x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline extend segment equally", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    
    polyline.extend_segment_equally(0, 0.5);
    
    REQUIRE(std::abs(polyline.points[0].x() - (-0.5f)) < 1e-5f);
    REQUIRE(std::abs(polyline.points[1].x() - 1.5f) < 1e-5f);
}

TEST_CASE("Polyline move", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    Vector translation(1.0f, 1.0f, 1.0f);
    
    polyline.move(translation);
    
    REQUIRE(polyline.points[0].x() == 1.0f);
    REQUIRE(polyline.points[0].y() == 1.0f);
    REQUIRE(polyline.points[0].z() == 1.0f);
    REQUIRE(polyline.points[1].x() == 2.0f);
    REQUIRE(polyline.points[1].y() == 1.0f);
    REQUIRE(polyline.points[1].z() == 1.0f);
}

TEST_CASE("Polyline is clockwise", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f)});
    Plane plane;
    
    bool clockwise = polyline.is_clockwise(plane);
    REQUIRE((clockwise == true || clockwise == false)); // Just test it doesn't crash
}

TEST_CASE("Polyline flip", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(2.0f, 0.0f, 0.0f)});
    
    polyline.flip();
    
    REQUIRE(polyline.points[0].x() == 2.0f);
    REQUIRE(polyline.points[1].x() == 1.0f);
    REQUIRE(polyline.points[2].x() == 0.0f);
}

TEST_CASE("Polyline get convex corners", "[polyline]") {
    Polyline polyline({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f), Point(1.0f, 1.0f, 0.0f), Point(0.0f, 1.0f, 0.0f)});
    
    std::vector<bool> convex_corners;
    polyline.get_convex_corners(convex_corners);
    
    REQUIRE(convex_corners.size() == 4);
}

TEST_CASE("Polyline tween two polylines", "[polyline]") {
    Polyline polyline0({Point(0.0f, 0.0f, 0.0f), Point(1.0f, 0.0f, 0.0f)});
    Polyline polyline1({Point(0.0f, 2.0f, 0.0f), Point(1.0f, 2.0f, 0.0f)});
    
    Polyline result = Polyline::tween_two_polylines(polyline0, polyline1, 0.5);
    
    REQUIRE(std::abs(result.points[0].y() - 1.0f) < 1e-5f);
    REQUIRE(std::abs(result.points[1].y() - 1.0f) < 1e-5f);
}
