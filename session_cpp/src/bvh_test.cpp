#include "catch_amalgamated.hpp"
#include "bvh.h"
#include <random>
#include <chrono>
#include <cmath>
#include <algorithm>
#include <utility>

using namespace session_cpp;

TEST_CASE("Expand bits for Morton codes", "[bvh]") {
    // Test bit expansion for Morton codes
    REQUIRE(expand_bits(0) == 0);
    REQUIRE(expand_bits(1) == 1);
    REQUIRE(expand_bits(2) == 8);
    REQUIRE(expand_bits(3) == 9);
    
    // 1023 in binary is 0b1111111111 (10 bits)
    // After expansion, should have pattern with zeros inserted
    uint32_t result = expand_bits(1023);
    REQUIRE(result > 0);  // Should be non-zero
}

TEST_CASE("Morton code at origin", "[bvh]") {
    uint32_t code = calculate_morton_code(0.0f, 0.0f, 0.0f, 100.0f);
    REQUIRE(code < (1u << 30)); // 30-bit code
}

TEST_CASE("Morton codes at corners", "[bvh]") {
    float world_size = 100.0f;

    // Corner at (-50, -50, -50) should give code 0
    uint32_t code_min = calculate_morton_code(-50.0f, -50.0f, -50.0f, world_size);
    REQUIRE(code_min == 0);

    // Corner at (50, 50, 50) should give maximum code
    uint32_t code_max = calculate_morton_code(50.0f, 50.0f, 50.0f, world_size);
    REQUIRE(code_max == 0x3FFFFFFF);  // Maximum 30-bit value
}

TEST_CASE("Morton code spatial locality", "[bvh]") {
    // Two nearby points should have similar codes
    uint32_t code1 = calculate_morton_code(10.0f, 10.0f, 10.0f);
    uint32_t code2 = calculate_morton_code(10.1f, 10.1f, 10.1f);

    // Two far apart points should have different codes
    uint32_t code3 = calculate_morton_code(-40.0f, -40.0f, -40.0f);

    // Nearby points should be closer in Morton space
    uint32_t diff_nearby = (code1 > code2) ? (code1 - code2) : (code2 - code1);
    uint32_t diff_far = (code1 > code3) ? (code1 - code3) : (code3 - code1);
    REQUIRE(diff_nearby < diff_far);
}

TEST_CASE("BVH node creation", "[bvh]") {
    BVHNode node;
    REQUIRE(node.left == nullptr);
    REQUIRE(node.right == nullptr);
    REQUIRE(node.object_id == -1);
    REQUIRE(!node.is_leaf());
}

TEST_CASE("BVH node leaf", "[bvh]") {
    BVHNode node;
    REQUIRE(!node.is_leaf());

    node.object_id = 5;
    REQUIRE(node.is_leaf());
}

TEST_CASE("BVH creation", "[bvh]") {
    BVH bvh(100.0f);
    REQUIRE(!bvh.guid.empty());
    REQUIRE(bvh.name == "my_bvh");
    REQUIRE(bvh.root == nullptr);
    REQUIRE(bvh.world_size == 100.0f);
}

TEST_CASE("BVH build empty", "[bvh]") {
    std::vector<BoundingBox> boxes;
    BVH bvh = BVH::from_boxes(boxes, 100.0f);
    REQUIRE(bvh.root == nullptr);
}

TEST_CASE("BVH build single", "[bvh]") {
    BoundingBox bbox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    std::vector<BoundingBox> boxes = {bbox};
    
    BVH bvh = BVH::from_boxes(boxes, 100.0f);
    
    REQUIRE(bvh.root != nullptr);
    REQUIRE(bvh.root->is_leaf());
    REQUIRE(bvh.root->object_id == 0);
}

TEST_CASE("BVH build multiple", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(-10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(0, 10, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    REQUIRE(bvh.root != nullptr);
    REQUIRE(!bvh.root->is_leaf());
    REQUIRE(bvh.root->left != nullptr);
    REQUIRE(bvh.root->right != nullptr);
}

TEST_CASE("BVH AABB intersect", "[bvh]") {
    BVH bvh(100.0f);

    // Overlapping boxes
    BoundingBox bbox1(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    BoundingBox bbox2(Point(0.5f, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    REQUIRE(bvh.aabb_intersect(bbox1, bbox2));

    // Non-overlapping boxes
    BoundingBox bbox3(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    REQUIRE(!bvh.aabb_intersect(bbox1, bbox3));
}

TEST_CASE("BVH find collisions no collision", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(-10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, checks] = bvh.find_collisions(0, bboxes[0], bboxes);
    REQUIRE(collisions.size() == 0);
    REQUIRE(checks > 0);
}

TEST_CASE("BVH find collisions with collision", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(2, 2, 2)),
        BoundingBox(Point(1, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(2, 2, 2))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, checks] = bvh.find_collisions(0, bboxes[0], bboxes);
    REQUIRE(collisions.size() == 1);
    REQUIRE(collisions[0] == 1);
}

TEST_CASE("BVH check all collisions", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(0.5f, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, colliding_indices, checks] = bvh.check_all_collisions(bboxes);

    // Boxes 0 and 1 should collide
    REQUIRE(collisions.size() == 1);
    REQUIRE(collisions[0].first == 0);
    REQUIRE(collisions[0].second == 1);
    REQUIRE(colliding_indices.size() == 2);
    REQUIRE(colliding_indices[0] == 0);
    REQUIRE(colliding_indices[1] == 1);
    REQUIRE(checks > 0);
}

TEST_CASE("BVH merge AABB", "[bvh]") {
    BVH bvh(100.0f);

    BoundingBox bbox1(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    BoundingBox bbox2(Point(5, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));

    BoundingBox merged = bvh.merge_aabb(bbox1, bbox2);

    // Merged box should encompass both
    REQUIRE(std::abs(merged.center.x() - 2.5f) < 0.001f);  // Midpoint between 0 and 5
    REQUIRE(std::abs(merged.half_size.x() - 3.5f) < 0.001f);  // Half of distance from -1 to 6
}

TEST_CASE("BVH performance many boxes", "[bvh]") {
    std::mt19937 gen(42);
    std::uniform_real_distribution<float> pos_dist(-40.0f, 40.0f);
    std::uniform_real_distribution<float> size_dist(0.5f, 2.0f);
    
    std::vector<BoundingBox> bboxes;
    for (int i = 0; i < 100; ++i) {
        Point center(pos_dist(gen), pos_dist(gen), pos_dist(gen));
        Vector half_size(size_dist(gen), size_dist(gen), size_dist(gen));
        BoundingBox bbox(center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size);
        bboxes.push_back(bbox);
    }

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);
    auto [collisions, colliding_indices, checks] = bvh.check_all_collisions(bboxes);

    // BVH should perform fewer checks than naive O(n²)
    int naive_checks = static_cast<int>(bboxes.size()) * (static_cast<int>(bboxes.size()) - 1) / 2;
    REQUIRE(checks < naive_checks);
}


TEST_CASE("BVH fixed 100 boxes collisions", "[bvh][fixed100]") {
    std::vector<BoundingBox> boxes;
    boxes.reserve(100);

    auto add = [&](float min_x, float min_y, float min_z, float max_x, float max_y, float max_z) {
        Point c((min_x + max_x) * 0.5f, (min_y + max_y) * 0.5f, (min_z + max_z) * 0.5f);
        Vector h((max_x - min_x) * 0.5f, (max_y - min_y) * 0.5f, (max_z - min_z) * 0.5f);
        boxes.emplace_back(c, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), h);
    };

    // 100 boxes: (min_x min_y min_z max_x max_y max_z)
    add(-53.1254f, -0.98185f, 20.5516f, -46.8089f, 5.89927f, 26.5331f);
    add(44.4446f, -1.5359f, -1.49382f, 50.7301f, 3.99953f, 7.58362f);
    add(36.9359f, -7.76782f, -28.7694f, 43.173f, -1.82645f, -22.1528f);
    add(-44.2654f, 26.3949f, 0.745263f, -35.0431f, 35.0799f, 6.13693f);
    add(0.239448f, -40.5791f, 32.6275f, 7.56243f, -33.2192f, 39.8776f);
    add(-31.6363f, -53.5568f, -52.162f, -21.6687f, -43.9796f, -43.2328f);
    add(3.72143f, 23.485f, 9.18924f, 10.4425f, 30.3631f, 15.5248f);
    add(-17.4583f, 10.2729f, -16.5162f, -12.1943f, 17.9162f, -10.7277f);
    add(-7.27998f, -22.0384f, -34.5872f, -1.95631f, -12.1058f, -26.8567f);
    add(-45.341f, 46.3634f, -10.4862f, -36.8332f, 52.2971f, -2.76774f);
    add(46.0445f, -34.6013f, 14.0587f, 53.0414f, -27.4064f, 22.7938f);
    add(-34.9367f, 28.5039f, 27.7749f, -29.4494f, 33.6524f, 33.4448f);
    add(9.97675f, -15.7696f, -27.8198f, 17.5104f, -8.16385f, -22.3021f);
    add(45.1965f, -19.307f, 22.0449f, 51.5233f, -10.9748f, 31.6205f);
    add(-7.03031f, -10.8607f, 38.8429f, 0.306212f, -0.974567f, 45.443f);
    add(25.5248f, 31.9848f, 20.436f, 33.3122f, 41.1186f, 28.0921f);
    add(-22.8772f, -19.5722f, -22.9988f, -15.6443f, -11.7384f, -14.7361f);
    add(-46.2318f, -5.27625f, -7.84674f, -41.1843f, 3.22896f, -0.905452f);
    add(-8.8814f, 40.3852f, -41.0122f, -1.73994f, 46.8478f, -33.9574f);
    add(-30.4719f, -15.9782f, 17.3287f, -20.7941f, -10.8891f, 24.7185f);
    add(28.6586f, 0.44821f, -41.9327f, 35.6602f, 6.09223f, -32.8706f);
    add(-14.173f, -45.5086f, 6.29666f, -7.48969f, -39.2406f, 13.229f);
    add(-21.8039f, 6.68129f, -32.5692f, -15.3816f, 16.6269f, -26.5873f);
    add(13.3659f, -1.97758f, 25.4002f, 19.0017f, 4.81311f, 31.5121f);
    add(-24.433f, -37.1532f, 41.849f, -15.8042f, -29.2066f, 49.4371f);
    add(-4.54629f, -16.9216f, -24.2439f, 2.40272f, -9.87919f, -17.0974f);
    add(-22.1316f, -18.2577f, -41.6624f, -13.4863f, -11.2109f, -36.6118f);
    add(-19.5562f, -1.13082f, -35.7364f, -10.2048f, 8.43363f, -25.912f);
    add(26.4514f, -31.3635f, -3.53901f, 32.4376f, -22.007f, 5.52268f);
    add(44.2805f, -20.3072f, 10.0337f, 52.6535f, -10.845f, 15.6482f);
    add(15.1756f, 46.2379f, 44.9662f, 20.8272f, 53.0835f, 50.1683f);
    add(1.39766f, -37.0106f, -2.59787f, 7.17823f, -28.0455f, 3.65286f);
    add(-31.882f, -21.1354f, 20.6053f, -24.8106f, -11.3482f, 28.4804f);
    add(-8.54435f, 10.0787f, 41.0063f, -1.08096f, 17.3793f, 46.4334f);
    add(21.317f, -38.2325f, 3.71512f, 29.3482f, -31.5114f, 10.6611f);
    add(-31.9136f, 27.8033f, -4.48008f, -23.6666f, 35.3487f, 0.804813f);
    add(8.52067f, 14.4157f, -37.4169f, 17.5301f, 20.4823f, -32.1696f);
    add(-7.88355f, 21.208f, 42.2586f, -0.205483f, 26.4206f, 50.4889f);
    add(-15.322f, -4.75221f, -17.9083f, -8.4181f, 4.47693f, -8.67731f);
    add(37.1268f, 2.17059f, -48.8049f, 45.7917f, 8.4744f, -40.7264f);
    add(-52.3809f, -6.49423f, 8.92399f, -42.9845f, 0.188961f, 18.343f);
    add(41.5732f, -7.42366f, -4.54156f, 51.0067f, -2.29871f, 0.643029f);
    add(-5.78252f, 0.645065f, -13.4131f, 1.93946f, 8.96885f, -5.49512f);
    add(7.58556f, -41.9641f, 23.8841f, 16.6142f, -32.1089f, 31.049f);
    add(-46.102f, -9.30967f, 44.8527f, -36.2572f, -2.2869f, 51.5056f);
    add(45.8031f, 27.0115f, -17.4386f, 52.3382f, 32.367f, -7.79126f);
    add(8.21008f, 39.3673f, 20.643f, 17.4628f, 45.1004f, 28.0194f);
    add(-47.9111f, -24.7374f, -29.2773f, -40.7686f, -16.0819f, -20.6671f);
    add(-29.8193f, -10.8358f, 24.5871f, -21.6958f, -3.36907f, 33.5925f);
    add(26.9713f, -26.2038f, -31.9261f, 35.2619f, -20.0422f, -25.0245f);
    add(-29.7903f, 8.92347f, -40.826f, -21.7701f, 15.776f, -35.2006f);
    add(-1.39845f, -13.7028f, -13.4383f, 8.26331f, -8.56298f, -7.95241f);
    add(-27.3862f, 17.0337f, 30.1216f, -19.7585f, 22.0732f, 39.076f);
    add(-15.102f, -39.6467f, -37.4648f, -8.16651f, -34.4574f, -31.1032f);
    add(14.1428f, -34.4961f, -47.6358f, 22.6478f, -25.6985f, -42.1577f);
    add(32.7187f, -0.0187469f, -2.54834f, 41.5605f, 9.91946f, 3.89622f);
    add(18.869f, -24.3319f, -0.588445f, 27.1926f, -18.2572f, 6.42131f);
    add(4.33372f, 6.78191f, -26.4923f, 12.7318f, 13.5283f, -19.058f);
    add(-3.88995f, -20.8689f, 18.4182f, 4.99471f, -11.484f, 25.6025f);
    add(-10.2896f, -22.7252f, -40.4815f, -3.08794f, -13.9661f, -30.6919f);
    add(30.2898f, 7.94805f, -2.19314f, 35.3154f, 17.6367f, 5.55489f);
    add(-33.8415f, 21.4915f, -16.5747f, -26.6066f, 27.2365f, -10.8669f);
    add(-22.4042f, 38.4298f, 21.7984f, -13.9447f, 47.0733f, 28.4925f);
    add(-6.87762f, 2.83366f, 10.2831f, -0.784998f, 11.5311f, 18.5943f);
    add(-34.4398f, -36.757f, 27.0559f, -27.6572f, -27.51f, 36.7491f);
    add(35.4006f, -17.8502f, -21.4524f, 41.7323f, -10.0449f, -12.5719f);
    add(28.1073f, 31.8896f, -16.4485f, 33.4307f, 37.9012f, -9.80763f);
    add(13.5936f, 25.9705f, 8.3269f, 22.4543f, 32.3162f, 16.4279f);
    add(28.2281f, -51.9913f, -14.7078f, 35.0256f, -42.5897f, -6.77297f);
    add(-27.4511f, -21.3243f, 42.9791f, -18.7936f, -14.3339f, 50.3538f);
    add(-42.0679f, -47.6033f, -33.2027f, -32.8703f, -38.8405f, -26.6373f);
    add(-52.2085f, -52.5573f, -33.0963f, -45.8755f, -44.5128f, -23.5496f);
    add(-11.2779f, -9.99167f, 24.9689f, -5.92983f, -0.191222f, 31.1336f);
    add(33.121f, 2.70727f, -33.8816f, 38.3024f, 10.367f, -26.2656f);
    add(-5.30061f, -39.8595f, 33.6105f, 4.23731f, -31.0826f, 42.5769f);
    add(-0.704829f, -26.0593f, -30.9797f, 4.64116f, -16.105f, -24.9783f);
    add(37.3045f, 34.9896f, 2.13491f, 46.4151f, 40.7296f, 10.6969f);
    add(-27.6823f, 41.9125f, -36.4809f, -17.7935f, 47.2728f, -26.7252f);
    add(34.666f, 27.0233f, 23.9605f, 44.5308f, 33.3f, 30.9151f);
    add(-37.3694f, -40.3928f, -6.27422f, -28.0124f, -31.5777f, -0.670845f);
    add(-34.1601f, 33.6584f, -28.8227f, -27.286f, 42.4497f, -22.2408f);
    add(-30.329f, -4.34317f, -43.1085f, -23.815f, 5.64745f, -35.7657f);
    add(-31.824f, 8.78623f, 25.1597f, -24.1868f, 17.2063f, 31.7098f);
    add(8.9247f, -12.5921f, 35.2262f, 16.9325f, -5.38381f, 44.3014f);
    add(-11.6258f, 44.3936f, -29.2716f, -3.07673f, 49.3977f, -20.2529f);
    add(-27.9412f, 32.9874f, -20.8262f, -22.5216f, 39.9326f, -12.0579f);
    add(39.7539f, -22.0106f, 31.131f, 46.0297f, -14.2677f, 40.1578f);
    add(-10.4385f, 20.3835f, 5.16852f, -5.23064f, 28.6092f, 14.2703f);
    add(19.9106f, -32.364f, 8.76233f, 25.9003f, -24.1348f, 16.1047f);
    add(-0.62887f, 18.0559f, 41.0991f, 5.37937f, 23.5869f, 49.7166f);
    add(20.6713f, -12.7322f, -19.7395f, 28.0693f, -3.71518f, -11.0217f);
    add(42.2797f, -30.3842f, 8.4357f, 51.5113f, -24.6986f, 15.3918f);
    add(-18.9658f, -26.1333f, -9.25188f, -12.9283f, -17.8373f, -3.68668f);
    add(32.8414f, -44.7499f, -3.96548f, 41.3729f, -35.5501f, 1.88547f);
    add(-12.0107f, -43.9043f, 15.2958f, -6.24849f, -38.452f, 21.6608f);
    add(-28.9449f, 35.0651f, -45.8908f, -23.5524f, 42.0763f, -39.3406f);
    add(25.2023f, -12.4615f, 8.84863f, 30.8803f, -6.57652f, 18.4333f);
    add(31.7285f, 31.0991f, -7.73725f, 39.8767f, 38.2288f, 0.932107f);
    add(-35.1346f, -8.00369f, 14.4611f, -27.1614f, -1.58541f, 21.4893f);
    add(13.9228f, -49.9973f, -2.77406f, 23.104f, -41.5596f, 4.89623f);

    REQUIRE(boxes.size() == 100);

    BVH bvh = BVH::from_boxes(boxes, 100.0f);
    auto [pairs, colliding_indices, checks] = bvh.check_all_collisions(boxes);

    std::sort(pairs.begin(), pairs.end());
    const std::vector<std::pair<int,int>> expected = {
        {4,74}, {8,59}, {10,91}, {13,86}, {19,32}, {20,73}, {22,27},
        {28,56}, {34,88}, {37,89}, {52,82}, {55,60}, {77,80}
    };
    std::vector<std::pair<int,int>> expected_sorted = expected;
    std::sort(expected_sorted.begin(), expected_sorted.end());

    REQUIRE(pairs.size() == expected_sorted.size());
    for (size_t i = 0; i < expected_sorted.size(); ++i) {
        REQUIRE(pairs[i].first == expected_sorted[i].first);
        REQUIRE(pairs[i].second == expected_sorted[i].second);
    }
}
