#pragma once
#include <vector>
#include <point.hpp>
#include <line.hpp>

namespace geo {

    /**
    * @class MomentShearAndBending
    * @brief A class representing moment, shear and bending for a single beam.
    */
    class MomentShearAndBending {
        public:

        /**
         * @brief Default constructor.
         */
        MomentShearAndBending();

        /**
         * @brief Constructor that initializes the moment, shear and bending with the given values.
         * @param span The span of the beam.
         * @param A The distance to the left support.
         * @param B The distance to the right support.
         * @param point_loads The point loads.
         * @param point_moments The point moments.
         * @param distributed_loads The distributed loads.
         */
        MomentShearAndBending(
            const double& span, 
            const double& A, 
            const double& B, 
            const std::vector<Point>& point_loads,
            const std::vector<Point>& point_moments,
            const std::vector<Line>& distributed_loads
            );



    };


}