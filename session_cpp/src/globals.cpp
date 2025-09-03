#include "globals.hpp"

namespace geo {

double GLOBALS::SCALE = 1e6;
double GLOBALS::ANGLE = 0.11;

std::string generate_uuid() {
    UUIDv4::UUIDGenerator<std::mt19937_64> uuidGenerator;
    UUIDv4::UUID uuid = uuidGenerator.getUUID();
    return uuid.str();
}

}  // namespace geo