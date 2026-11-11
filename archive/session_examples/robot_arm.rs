use session_rust::{read_file_obj, Color, Session, Xform};
use std::path::Path;

struct Joint {
    xyz: [f64; 3],
    rpy: [f64; 3],
    angle: f64,
}

const JOINTS: [Joint; 7] = [
    Joint {
        xyz: [0.0, 0.0, 0.1575],
        rpy: [0.0, 0.0, 0.0],
        angle: 25.0,
    },
    Joint {
        xyz: [0.0, 0.0, 0.2025],
        rpy: [90.0, 0.0, 180.0],
        angle: -35.0,
    },
    Joint {
        xyz: [0.0, 0.2045, 0.0],
        rpy: [90.0, 0.0, 180.0],
        angle: 20.0,
    },
    Joint {
        xyz: [0.0, 0.0, 0.2155],
        rpy: [90.0, 0.0, 0.0],
        angle: 60.0,
    },
    Joint {
        xyz: [0.0, 0.1845, 0.0],
        rpy: [-90.0, 180.0, 0.0],
        angle: -20.0,
    },
    Joint {
        xyz: [0.0, 0.0, 0.2155],
        rpy: [90.0, 0.0, 0.0],
        angle: 35.0,
    },
    Joint {
        xyz: [0.0, 0.081, 0.0],
        rpy: [-90.0, 180.0, 0.0],
        angle: 10.0,
    },
];

fn local_xform(joint: &Joint) -> Xform {
    Xform::translation(joint.xyz[0], joint.xyz[1], joint.xyz[2])
        * Xform::rotation_z(joint.rpy[2], true)
        * Xform::rotation_y(joint.rpy[1], true)
        * Xform::rotation_x(joint.rpy[0], true)
        * Xform::rotation_z(joint.angle, true)
}

fn main() {
    let data = Path::new("session_data/robot_arm");
    let output = "session_data/robot_arm_rust.pb";
    let mut session = Session::new("robot_arm");
    let mut parent = None;
    let mut guids = Vec::new();
    let mut locals = Vec::new();

    for i in 0..8 {
        let mut mesh = read_file_obj(data.join(format!("link_{i}.obj")).to_str().unwrap()).unwrap();
        mesh.name = format!("link_{i}");
        mesh.set_guid(format!("robot-link-{i}"));
        mesh.set_objectcolor(if i % 2 == 0 {
            Color::orange()
        } else {
            Color::lightgrey()
        });
        let node = session.add_mesh(mesh, parent.as_ref()).unwrap();
        let local = if i == 0 {
            Xform::identity()
        } else {
            local_xform(&JOINTS[i - 1])
        };
        let guid = node.borrow().name.clone();
        session.set_xform(&guid, local.clone());
        guids.push(guid);
        locals.push(local);
        parent = Some(node);
    }

    let mut expected = Xform::identity();

    for (guid, local) in guids.iter().zip(locals.iter()) {
        expected *= local.clone();
        assert!(session.xform(guid) == *local);
        assert!(session.world_xform(guid) == expected);
    }

    session.pb_dump(output);
    let loaded = Session::pb_load(output).unwrap();
    assert!(loaded.world_xform(guids.last().unwrap()) == expected);
    println!("{output}");
}

/*
|||||||| DESCRIPTION ||||||||
A KUKA iiwa robot arm built from eight downloaded OBJ link meshes. Every mesh stays in its own
link coordinates. Session.xforms stores the LOCAL joint transform, while the nested tree makes
world_xform multiply the chain from the base through the tool. Assertions prove every cumulative
transform before and after the protobuf roundtrip.

|||||||| DIRECTORY ||||||||
cd wood_research/session

|||||||| CARGO RUN && CLOUDFLARE ||||||||
cargo run --manifest-path session_rust/Cargo.toml --example session_robot_arm_rust --release && ../bash/publish-scene.sh "$PWD/session_data/robot_arm_rust.pb" --no-notify

|||||||| VIEW ||||||||
https://petrasvestartas.github.io/session/
*/
