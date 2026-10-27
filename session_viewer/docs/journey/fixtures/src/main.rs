mod ordered_mesh;

use prost::Message;
use session_rust::{
    element::ElementFeature, proto, BRep, Color, Element, Mesh, Point, Polyline, Xform,
};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::PathBuf};

fn document(name: &str, guid: &str, objects: proto::Objects, ids: &[&str]) -> proto::Session {
    proto::Session {
        name: name.into(),
        guid: guid.into(),
        objects: Some(objects),
        tree: Some(proto::Tree {
            root: Some(proto::TreeNode {
                name: "root".into(),
                guid: format!("{guid}-root"),
                children: ids
                    .iter()
                    .map(|id| proto::TreeNode {
                        name: (*id).into(),
                        guid: format!("{guid}-{id}-node"),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn comparison() -> proto::Session {
    let mut breps = Vec::new();
    for (i, (mut brep, z)) in [
        (BRep::create_box(1500., 1500., 100.), -500.),
        (BRep::create_box(350., 350., 700.), -100.),
        (BRep::create_cylinder(120., 450.), 250.),
        (BRep::create_cylinder(650., 120.), 700.),
    ]
    .into_iter()
    .enumerate()
    {
        brep.set_guid(format!("comparison-{i}"));
        brep.surfacecolor = Color::new(0.35, 0.65, 0.85, 1.);
        brep.transform(&Xform::translation(0., 0., z));
        breps.push(brep.to_proto());
    }
    document(
        "viewer comparison",
        "course-comparison",
        proto::Objects {
            breps,
            ..Default::default()
        },
        &[
            "comparison-0",
            "comparison-1",
            "comparison-2",
            "comparison-3",
        ],
    )
}

fn metadata() -> proto::Session {
    let mut message = comparison();
    message.guid = "course-metadata".into();
    message
        .objects
        .as_mut()
        .unwrap()
        .components
        .push(proto::Component {
            type_name: "FloorBuilder".into(),
            guid: "component-original".into(),
            name: "guide".into(),
            json_data: "{ \"origin\": [1000000000.125, 0, 0], \"custom\": true }".into(),
        });
    message
        .tree
        .as_mut()
        .unwrap()
        .root
        .as_mut()
        .unwrap()
        .children
        .push(proto::TreeNode {
            name: "component-original".into(),
            guid: "component-original-node".into(),
            ..Default::default()
        });
    message.interactions = vec![proto::InteractionEntry {
        guid: "original-edge".into(),
        interactions: vec![proto::Interaction {
            guid: "original-interaction".into(),
            name: "glue".into(),
            interaction_type: "custom-glue".into(),
            interaction_data: vec![0, 255, 1, 2, 3],
        }],
    }];
    message
}

fn feature(guid: &str, kind: &str, name: &str, points: [[f64; 3]; 2]) -> ElementFeature {
    let mut outline = Polyline::new(
        points
            .into_iter()
            .map(|p| Point::new(p[0], p[1], p[2]))
            .collect(),
    );
    outline.set_guid(format!("{guid}-outline"));
    let mut feature = ElementFeature::new(kind, -1, vec![outline], name);
    feature.set_guid(guid.into());
    feature
}

fn element(
    guid: &str,
    name: &str,
    mut mesh: Mesh,
    features: Vec<ElementFeature>,
) -> proto::Element {
    mesh.set_guid(format!("{guid}-mesh"));
    let mesh_bytes = ordered_mesh::encode(mesh.to_proto());
    let mut element = Element::new(name);
    element.set_guid(guid.into());
    element.set_geometry(mesh);
    element.features = features;
    let mut message = element.to_proto();
    message.geometry_data = mesh_bytes;
    message
}

fn instance_features() -> proto::Session {
    let mut mesh = Mesh::from_vertices_and_faces(
        vec![
            Point::new(-0.4, -0.4, 0.),
            Point::new(0.4, -0.4, 0.),
            Point::new(0., 0.4, 0.),
        ],
        vec![vec![0, 1, 2]],
    );
    mesh.set_objectcolor(Color::new(0.1, 0.3, 0.8, 1.));
    let definition = element(
        "prototype",
        "prototype",
        mesh,
        vec![feature(
            "prototype-axis",
            "axis",
            "prototype axis",
            [[-0.2, 0., 0.], [0.2, 0., 0.]],
        )],
    );
    let instances = ["first", "second"]
        .into_iter()
        .enumerate()
        .map(|(i, id)| proto::InstanceRef {
            guid: id.into(),
            name: if i == 0 { "First" } else { "Second" }.into(),
            definition_guid: "prototype".into(),
            flags: if i == 1 {
                session_rust::InstanceRef::FLAG_LOCKED
            } else {
                0
            },
            features: vec![feature(
                &format!("instance-feature-{i}"),
                "cut",
                &format!("cut {i}"),
                [[-0.2, 0.2, 0.], [0.2, 0.2, 0.]],
            )
            .to_proto()],
            ..Default::default()
        })
        .collect();
    let mut message = document(
        "Shared original definitions",
        "definition-document",
        proto::Objects {
            instances,
            ..Default::default()
        },
        &["first", "second"],
    );
    message.definitions = Some(proto::Objects {
        elements: vec![definition],
        ..Default::default()
    });
    message.xforms = [(-0.6, "first"), (0.6, "second")]
        .into_iter()
        .map(|(x, id)| proto::XformEntry {
            guid: id.into(),
            xform: Some(Xform::translation(x, 0., 0.).to_proto()),
        })
        .collect();
    message
}

fn feature_pick() -> proto::Session {
    let elements = [("pick-first", "Pick first"), ("pick-locked", "Pick locked")]
        .into_iter()
        .enumerate()
        .map(|(i, (id, name))| {
            let mut source = element(
                id,
                name,
                Mesh::create_box(0.2, 0.2, 0.2),
                vec![feature(
                    &format!("{id}-cut"),
                    "cut",
                    "pick line",
                    [[-0.8, 0.4, 0.2], [0.8, 0.4, 0.2]],
                )],
            );
            source.is_locked = (i == 1).then_some(true);
            source
        })
        .collect();
    let mut message = document(
        "Original feature picks",
        "course-feature-pick",
        proto::Objects {
            elements,
            ..Default::default()
        },
        &["pick-first", "pick-locked"],
    );
    message.xforms = vec![proto::XformEntry {
        guid: "pick-locked".into(),
        xform: Some(Xform::translation(2., 0., 0.).to_proto()),
    }];
    message
}

fn specimens() -> Vec<(&'static str, proto::Session)> {
    vec![
        ("course-comparison.pb", comparison()),
        ("course-metadata.pb", metadata()),
        ("course-instance-features.pb", instance_features()),
        ("course-feature-pick.pb", feature_pick()),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets/pb"));
    if args.next().is_some() {
        return Err("usage: viewer-course-fixtures [output-directory]".into());
    }
    let first = specimens();
    let second = specimens();
    let mut entries = Vec::new();
    fs::create_dir_all(&output)?;
    for ((name, source), (_, repeated)) in first.into_iter().zip(second) {
        let bytes = source.encode_to_vec();
        assert_eq!(
            bytes,
            repeated.encode_to_vec(),
            "nondeterministic fixture {name}"
        );
        assert_eq!(proto::Session::decode(bytes.as_slice())?, source);
        let hash = format!("{:x}", Sha256::digest(&bytes));
        fs::write(output.join(name), &bytes)?;
        entries.push(serde_json::json!({"file": name, "bytes": bytes.len(), "sha256": hash}));
        println!("{name}: {} bytes, sha256 {hash}", bytes.len());
    }
    let manifest = serde_json::json!({
        "status": "reconstructed deterministic fixtures; historical bytes unavailable; rendering acceptance pending",
        "source": "session/session_viewer/docs/journey/fixtures",
        "files": entries,
    });
    fs::write(
        output.join("course-fixtures.json"),
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    Ok(())
}
