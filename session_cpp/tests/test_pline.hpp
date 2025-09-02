#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "pline.hpp"
#include "test_methods.hpp"

void test_cut() {
    std::vector<geo::Point> points = {
        {0, 0, 0},
        {1, 0, 0},
        {1, 0, 1},
        {0, 0, 1},
    };

    geo::Plane plane(geo::Point(0, 0, 0.5), geo::Vector(0, 0, 1));
    std::vector<geo::Point> points_cut;
    bool result = geo::Pline::cut(points, plane, points_cut);
    
    // Use result to avoid warning
    std::string status = result ? "success" : "failure";
    geo::log("Cut operation returned: " + status, 
             result ? geo::LogLevel::INFO : geo::LogLevel::WARNING);
    geo::log("WARNING: does not work, revise the cut algorithm!", geo::LogLevel::WARNING);


    for (auto& p : points_cut){
        std::cout << p[0] << " " << p[1] << " " << p[2] << std::endl;

    }
       
    // my_assert(points_cut[0][0] == 1 && points_cut[0][1] == 0 && points_cut[0][2] == 0.5 &&
    //           points_cut[1][0] == 1 && points_cut[1][1] == 0 && points_cut[1][2] == 1 &&
    //           points_cut[2][0] == 0 && points_cut[2][1] == 0 && points_cut[2][2] == 1 &&
    //           points_cut[3][0] == 0 && points_cut[3][1] == 0 && points_cut[3][2] == 0.5);

}

int test_pline_main() {
    std::cout << "Running test_pline..." << std::endl;
    test_cut();
    return 0;
}