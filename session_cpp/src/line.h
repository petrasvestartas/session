#pragma once
#include "point.h"
#include "vector.h"
#include "color.h"
#include "xform.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

namespace session_cpp {

/**
 * @class Line
 * @brief A class representing a line in 3D space with display properties.
 */
class Line {
public:
    std::string guid = ::guid();       ///< Unique identifier for the line
    std::string name = "my_line";      ///< Line identifier/name
    float width = 1.0f;                ///< Line width in pixels
    Color linecolor = Color::white();  ///< Color of the line
    Xform xform;   ///< Transformation matrix
    // Mesh mesh;                      ///< Mesh for visualization (pipe) - TODO: implement later

private:
    float _x0 = 0.0f;                  ///< X coordinate of start point
    float _y0 = 0.0f;                  ///< Y coordinate of start point
    float _z0 = 0.0f;                  ///< Z coordinate of start point
    float _x1 = 0.0f;                  ///< X coordinate of end point
    float _y1 = 0.0f;                  ///< Y coordinate of end point
    float _z1 = 1.0f;                  ///< Z coordinate of end point

public:

    /// Getters for coordinates
    float x0() const { return _x0; }
    float y0() const { return _y0; }
    float z0() const { return _z0; }
    float x1() const { return _x1; }
    float y1() const { return _y1; }
    float z1() const { return _z1; }

    /// Setters for coordinates
    void set_x0(float v) { _x0 = v; }
    void set_y0(float v) { _y0 = v; }
    void set_z0(float v) { _z0 = v; }
    void set_x1(float v) { _x1 = v; }
    void set_y1(float v) { _y1 = v; }
    void set_z1(float v) { _z1 = v; }

    /**
     * @brief Default constructor.
     */
    Line();

    /**
     * @brief Constructor with coordinates.
     * @param x0 first point x coordinate.
     * @param y0 first point y coordinate.
     * @param z0 first point z coordinate.
     * @param x1 second point x coordinate.
     * @param y1 second point y coordinate.
     * @param z1 second point z coordinate.
     */
    Line(float x0, float y0, float z0, float x1, float y1, float z1);

    /**
     * @brief Constructor from two points.
     * @param p1 The first point.
     * @param p2 The second point.
     */
    static Line from_points(const Point& p1, const Point& p2);

    /**
     * @brief Constructor with name and coordinates.
     * @param name The name for the line.
     * @param x0 first point x coordinate.
     * @param y0 first point y coordinate.
     * @param z0 first point z coordinate.
     * @param x1 second point x coordinate.
     * @param y1 second point y coordinate.
     * @param z1 second point z coordinate.
     */
    static Line with_name(const std::string& name, float x0, float y0, float z0, float x1, float y1, float z1);

    /// Convert line to string representation
    std::string to_string() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON-serializable object
    nlohmann::ordered_json jsondump() const;

    /// Create line from JSON data
    static Line jsonload(const nlohmann::json& data);

    /// Serialize to JSON file

    /// Deserialize from JSON file

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Subscript operator for non-const access.
     * @param index The index (0=x0, 1=y0, 2=z0, 3=x1, 4=y1, 5=z1).
     * @return A reference to the coordinate.
     */
    float& operator[](int index);

    /**
     * @brief Subscript operator for const access.
     * @param index The index (0=x0, 1=y0, 2=z0, 3=x1, 4=y1, 5=z1).
     * @return A const reference to the coordinate.
     */
    const float& operator[](int index) const;

    Line& operator+=(const Vector& other);
    Line& operator-=(const Vector& other);
    Line& operator*=(float factor);
    Line& operator/=(float factor);

    Line operator+(const Vector& other) const;
    Line operator-(const Vector& other) const;
    Line operator*(float factor) const;
    Line operator/(float factor) const;

    Vector to_vector() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Geometry Methods
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Calculates the length of the line.
     * @return The length of the line.
     */
    float length() const;

    /**
     * @brief Calculates the squared length of the line.
     * @return The squared length of the line.
     */
    float squared_length() const;

    /**
     * @brief Calculates the point at a given parameter t.
     * @param t The parameter value.
     * @return The point at the given parameter value.
     */
    Point point_at(float t) const;

    /**
     * @brief Get start point.
     */
    Point start() const;

    /**
     * @brief Get end point.
     */
    Point end() const;

    // /// Updates the mesh representation using thickness.
    // Line& update_mesh();
    // 
    // /// Gets the mesh representation of this line as a pipe.
    // Mesh* get_mesh();
    //
    // /// Returns a transform that maps the canonical unit pipe onto this line segment.
    // Xform to_pipe_transform() const;

};

/**
 * @brief Output stream operator for Line.
 * @param os The output stream.
 * @param line The Line to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream& operator<<(std::ostream& os, const Line& line);

}  // namespace session_cpp

// fmt formatter specialization for Line
template <> struct fmt::formatter<session_cpp::Line> {
  constexpr auto parse(fmt::format_parse_context& ctx) { return ctx.begin(); }

  auto format(const session_cpp::Line& o, fmt::format_context& ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};