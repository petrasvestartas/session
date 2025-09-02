#pragma once
#include "core.hpp"
#include "point.hpp"
#include "vector.hpp"

namespace geo {

/**
 * @brief Get the moment sign object. Origin is 0, 0, 0.
 * CW = 1, CCW = -1, COLLINEAR = 0
 *
 * /+\    |    /-\   |      |      |
 *  |     |     |    |      |      |
 *  |     |     |   \-/     |     \+/
 *
 * —————— + ————>    <————— - —————
 * ——————————————    ——————————————
 * —————— - ————>    <————— + —————
 *
 * @param p Point
 * @param f Vector
 * @param cw_is_positive bool, change to false if you want CCW to be positive
 * @return std::array<int, 2>
 */
std::array<int, 2> get_moment_sign(const Point& p, const Vector& f, bool cw_is_positive = true);

/**
 * @brief Get the moment object. Origin is 0, 0, 0.
 * @param p Point
 * @param f Vector
 * @return std::array<double, 2>
 */
std::array<double, 2> get_moment(const Point& p, const Vector& f);

/**
 * @brief Get the moment with eccentricity object. Origin is 0, 0, 0.
 *
 *   fx  | fy
 *   ——> V
 *       *    e
 *       | \
 *     deg  *    p
 *
 * @param p Point
 * @param f Vector force applied at the point
 * @param e eccentricity, a distance from the point p to the applied force
 * @param deg angle in degrees
 * @return std::array<double, 2>
 */
std::array<double, 2> get_moment_with_eccentricity(const Point& p, const Vector& f, const double& e,
                                                   const double& deg);

/**
 * @brief Get the vertical reaction forces given two eccentricities of a frame and a horizontal
 * force. Resource: https://www.sciencedirect.com/science/article/pii/S0141029606002124
 *
 * horizontal reaction force
 * fx      ———————————————————————————————————————————
 * ————>   *                                         *
 *         | e0 - distance                           |  e1 - distance
 *         * center0                                 *  center1
 *         |                                         |
 *         ———————————————————————————————————————————
 *
 * @param F0x Vector horizontal force
 * @param e0 Point eccentricity of the first frame
 * @param en Point eccentricity of the second frame
 */
double global_equilibrium_of_frame(const Vector& F0x, const double& e0, const double& en);

/**
 * @brief Get the local equilibrium of a block given the following parameters:
 * Resource: https://www.sciencedirect.com/science/article/pii/S0141029606002124
 *
 * F0y  |         —————————
 *     \ / —————————  W     *
 * F0x —>   *      pw *      *  p1
 *      p0   *                —
 *           | —        ————————
 *           |  ————————
 *       deg0 ∠
 */
double local_equilibrium_of_block(const Point& p0, const double& e0, const Vector& F0x,
                                  const Vector& F0y, const double& deg0, const Point& pw,
                                  const Vector& W, const Point& p1, const double& deg1);
}  // namespace geo