from pathlib import Path

from session_py import Color
from session_py import Session
from session_py import Xform
from session_py import read_file_obj


JOINTS = [
    ((0.0, 0.0, 0.1575), (0.0, 0.0, 0.0), 25.0),
    ((0.0, 0.0, 0.2025), (90.0, 0.0, 180.0), -35.0),
    ((0.0, 0.2045, 0.0), (90.0, 0.0, 180.0), 20.0),
    ((0.0, 0.0, 0.2155), (90.0, 0.0, 0.0), 60.0),
    ((0.0, 0.1845, 0.0), (-90.0, 180.0, 0.0), -20.0),
    ((0.0, 0.0, 0.2155), (90.0, 0.0, 0.0), 35.0),
    ((0.0, 0.081, 0.0), (-90.0, 180.0, 0.0), 10.0),
]


def local_xform(joint):
    xyz, rpy, angle = joint
    return (
        Xform.translation(*xyz)
        * Xform.rotation_z(rpy[2], True)
        * Xform.rotation_y(rpy[1], True)
        * Xform.rotation_x(rpy[0], True)
        * Xform.rotation_z(angle, True)
    )


def main():
    data = Path("session_data/robot_arm")
    output = Path("session_data/robot_arm_py.pb")
    session = Session("robot_arm")
    parent = None
    guids = []
    locals_ = []

    for i in range(8):
        mesh = read_file_obj(str(data / f"link_{i}.obj"))
        mesh.name = f"link_{i}"
        mesh.guid = f"robot-link-{i}"
        mesh.set_objectcolor(Color.orange() if i % 2 == 0 else Color.lightgrey())
        node = session.add_mesh(mesh, parent)
        assert node is not None

        local = Xform.identity() if i == 0 else local_xform(JOINTS[i - 1])
        session.set_xform(node.name, local)
        guids.append(node.name)
        locals_.append(local)
        parent = node

    expected = Xform.identity()

    for guid, local in zip(guids, locals_):
        expected *= local
        assert session.xform(guid) == local
        assert session.world_xform(guid) == expected

    session.pb_dump(output)
    loaded = Session.pb_load(output)
    assert loaded.world_xform(guids[-1]) == expected
    print(output)


if __name__ == "__main__":
    main()


"""
|||||||| DESCRIPTION ||||||||
A KUKA iiwa robot arm built from eight downloaded OBJ link meshes. Every mesh stays in its own
link coordinates. Session.xforms stores the LOCAL joint transform, while the nested tree makes
world_xform multiply the chain from the base through the tool. Assertions prove every cumulative
transform before and after the protobuf roundtrip.

|||||||| DIRECTORY ||||||||
cd wood_research/session

|||||||| RUN && CLOUDFLARE ||||||||
PYTHONPATH=session_py/src ./uvsession/bin/python session_examples/robot_arm.py && ../bash/publish-scene.sh "$PWD/session_data/robot_arm_py.pb" --no-notify

|||||||| VIEW ||||||||
https://petrasvestartas.github.io/session/
"""
