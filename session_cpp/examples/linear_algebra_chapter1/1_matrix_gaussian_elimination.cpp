#include <iomanip>
#include <array>
#include <vector>
#include "../../src/matrix_gaussian_elimination.hpp"
#include "../../src/matrix.hpp"
#include "../../src/logger.hpp"

int main() {
    // Solving a 3x3 system of linear equations:
    // 2x + y - z = 8
    // -3x - y + 2z = -11
    // -2x + y + 2z = -3

    // Create the coefficient matrix A and right-hand side vector b
    geo::Matrix A{{2, 1, -1},    // First equation: 2x + y - z = 8
                  {-3, -1, 2},   // Second equation: -3x - y + 2z = -11
                  {-2, 1, 2}};   // Third equation: -2x + y + 2z = -3
    std::vector<double> b{8, -11, -3};

    // Print the system of equations
    geo::log("Solving the system of equations:");
    geo::log("2x + y - z = 8");
    geo::log("-3x - y + 2z = -11");
    geo::log("-2x + y + 2z = -3");
    geo::log("");

    // Solve the system using Gaussian elimination
    std::vector<double> solution;
    try {
        solution = geo::gauss_partial(A, b);

        // Print the solution
        geo::log("Solution:");
        geo::log("x = " + std::to_string(solution[0]));
        geo::log("y = " + std::to_string(solution[1]));
        geo::log("z = " + std::to_string(solution[2]));

        // Verify the solution by substituting back into the equations
        geo::log("\nVerification:");
        double eq1 = 2*solution[0] + solution[1] - solution[2];
        double eq2 = -3*solution[0] - solution[1] + 2*solution[2];
        double eq3 = -2*solution[0] + solution[1] + 2*solution[2];

        geo::log("Equation 1: " + std::to_string(eq1) + " = 8");
        geo::log("Equation 2: " + std::to_string(eq2) + " = -11");
        geo::log("Equation 3: " + std::to_string(eq3) + " = -3");
    } catch (const std::runtime_error& e) {
        geo::log("Error: " + std::string(e.what()), geo::LogLevel::ERROR);
    }

    return 0;
}