#include "file_obj.h"
#include "session.h"
#include <array>
#include <filesystem>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

using namespace session_cpp;

namespace {

struct Joint {
    std::array<double, 3> xyz;
    std::array<double, 3> rpy;
    double angle;
};

const std::array<Joint, 7> JOINTS = {{
    {{0.0, 0.0, 0.1575}, {0.0, 0.0, 0.0}, 25.0},
    {{0.0, 0.0, 0.2025}, {90.0, 0.0, 180.0}, -35.0},
    {{0.0, 0.2045, 0.0}, {90.0, 0.0, 180.0}, 20.0},
    {{0.0, 0.0, 0.2155}, {90.0, 0.0, 0.0}, 60.0},
    {{0.0, 0.1845, 0.0}, {-90.0, 180.0, 0.0}, -20.0},
    {{0.0, 0.0, 0.2155}, {90.0, 0.0, 0.0}, 35.0},
    {{0.0, 0.081, 0.0}, {-90.0, 180.0, 0.0}, 10.0},
}};

Xform local_xform(const Joint &joint) {
    return Xform::translation(joint.xyz[0], joint.xyz[1], joint.xyz[2]) * Xform::rotation_z(joint.rpy[2], true) *
           Xform::rotation_y(joint.rpy[1], true) * Xform::rotation_x(joint.rpy[0], true) *
           Xform::rotation_z(joint.angle, true);
}

void require(bool condition, const std::string &message) {
    if (!condition)
        throw std::runtime_error(message);
}

} // namespace

int main() {
    const std::filesystem::path data = "session_data/robot_arm";
    const std::filesystem::path output = "session_data/robot_arm_cpp.pb";
    Session session("robot_arm");
    std::shared_ptr<TreeNode> parent;
    std::vector<std::string> guids;
    std::vector<Xform> locals;

    for (size_t i = 0; i < 8; i++) {
        Mesh mesh = file_obj::read_file_obj((data / ("link_" + std::to_string(i) + ".obj")).string());
        mesh.name = "link_" + std::to_string(i);
        mesh.guid() = "robot-link-" + std::to_string(i);
        mesh.set_objectcolor(i % 2 == 0 ? Color::orange() : Color::lightgrey());
        const std::shared_ptr<TreeNode> node = session.add_mesh(std::make_shared<Mesh>(std::move(mesh)), parent);
        require(node != nullptr, "Failed to add link_" + std::to_string(i));

        const Xform local = i == 0 ? Xform::identity() : local_xform(JOINTS[i - 1]);
        session.set_xform(node->name, local);
        guids.push_back(node->name);
        locals.push_back(local);
        parent = node;
    }

    Xform expected = Xform::identity();

    for (size_t i = 0; i < guids.size(); i++) {
        expected *= locals[i];
        require(session.xform(guids[i]) == locals[i], "Wrong local transform at link_" + std::to_string(i));
        require(session.world_xform(guids[i]) == expected, "Wrong world transform at link_" + std::to_string(i));
    }

    session.pb_dump(output.string());
    const Session loaded = Session::pb_load(output.string());
    require(loaded.world_xform(guids.back()) == expected, "Protobuf roundtrip changed the tool transform");
    std::cout << output << '\n';
}

/*
|||||||| DESCRIPTION ||||||||
A KUKA iiwa robot arm built from eight downloaded OBJ link meshes. Every mesh
stays in its own link coordinates. Session.xforms stores the LOCAL joint
transform, while the nested tree makes world_xform multiply the chain from the
base through the tool. Assertions prove every cumulative transform before and
after the protobuf roundtrip.

|||||||| DIRECTORY ||||||||
cd wood_research/session

|||||||| CMAKE CONFIGURE ||||||||
cmake -S session_cpp -B session_cpp/build -DCMAKE_BUILD_TYPE=Release

|||||||| CMAKE BUILD && RUN && CLOUDFLARE ||||||||
cmake --build session_cpp/build --target session_robot_arm_cpp --parallel 6 && ./session_cpp/build/session_robot_arm_cpp && ../bash/publish-scene.sh "$PWD/session_data/robot_arm_cpp.pb" --no-notify

|||||||| VIEW ||||||||
https://petrasvestartas.github.io/session/
*/
