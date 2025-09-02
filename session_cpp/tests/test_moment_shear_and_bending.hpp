#pragma once
#include <iostream>  // For std::cout
#include <vector>    // For std::vector
#include <array>     // For std::array
#include "load_point.hpp"  // For geo::LoadPoint
#include "load_line.hpp"   // For geo::LoadLine
#include "moment_shear_and_bending.hpp"

std::array<geo::LoadPoint, 2> compute_reactions(const std::vector<geo::LoadPoint>& reactions, const geo::LoadPoint& point_load) {

    double lever_arm_0 = reactions[0].p[0] - point_load.p[0];
    double moment_0 = point_load.l[1] * lever_arm_0;
    double lever_arm_1 = reactions[1].p[0] - reactions[0].p[0];

    //result reactions
    std::array<geo::LoadPoint, 2> reactions_temp = {{reactions[0], reactions[1]}};
    reactions_temp[1].l[1] = moment_0 / lever_arm_1;
    reactions_temp[0].l[1] = -point_load.l[1] - reactions_temp[1].l[1];

    return reactions_temp;
}


void test_moment_shear_and_bending() {
    
    // DIMENSIONS AND RESOLUTION
    double span = 17;
    int divisions = 10000;

    // LOADS
    std::vector<geo::LoadPoint> point_loads {{geo::Point(6, 0, 0), geo::Vector(0, -90, 0)}};
    std::vector<geo::LoadPoint> point_moments {{geo::Point(17, 0, 0), geo::Vector(0, 0, 50)}};
    std::vector<geo::LoadLine> distributed_loads {{geo::Line(8, 0, 0, 17, 0, 0), geo::Vector(-10, 0, 0), geo::Vector(0, 0, 0)}};

    // REACTIONS
    std::vector<geo::LoadPoint> reactions {
        {geo::Point(3, 0, 0), geo::Vector(0, 0, 0)}, // Va and Ha
        {geo::Point(13, 0, 0), geo::Vector(0, 0, 0)}, // Vb
    };

    // SHEAR AND BENDING MOMENT DIAGRAMSs
    double distance_between_data_points = span / divisions;
    std::vector<double> X;
    X.reserve(divisions + 1);
    for (double x = 0; x <= span; x += distance_between_data_points) 
        X.push_back(x);

    std::vector<geo::LoadPoint> shear_force (divisions + 1);
    std::vector<geo::LoadPoint> bending_moment (divisions + 1);


    // COMPUTE REACTIONS
    std::vector<std::array<geo::LoadPoint, 2> > reactions_record;
    reactions_record.reserve(point_loads.size());
    if (point_loads.size() > 0) {

        for (size_t i = 0; i < point_loads.size(); i++) {
            std::array<geo::LoadPoint, 2> temp_reactions = compute_reactions(reactions, point_loads[i]);
            reactions_record.push_back(temp_reactions);

            // Add reactions to record
            reactions[0].l += temp_reactions[0].l;
            reactions[1].l += temp_reactions[1].l;
        }
        
    }

    // OUTPUT

    std::cout << "Point loads:\n";

    for (auto& pointload : point_loads)
        std::cout << pointload << std::endl;

    std::cout << "Reactions:\n";

    for (auto& reaction : reactions)
        std::cout << reaction << std::endl;
}

int test_moment_shear_and_bending_main() {
    std::cout << "Running test_moment_shear_and_bending..." << std::endl;
    test_moment_shear_and_bending();
    return 0;
}