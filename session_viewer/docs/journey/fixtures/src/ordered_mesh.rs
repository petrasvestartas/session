use prost::Message;
use session_rust::proto;
use std::collections::BTreeMap;

// Same kernel wire schema; only map iteration order changes.
#[derive(Clone, PartialEq, Message)]
struct OrderedMesh {
    #[prost(string, tag = "1")]
    guid: String,
    #[prost(string, tag = "2")]
    name: String,
    #[prost(btree_map = "uint64, message", tag = "3")]
    vertices: BTreeMap<u64, proto::VertexData>,
    #[prost(btree_map = "uint64, message", tag = "4")]
    faces: BTreeMap<u64, proto::FaceData>,
    #[prost(message, repeated, tag = "6")]
    edge_data: Vec<proto::EdgeData>,
    #[prost(btree_map = "string, double", tag = "7")]
    default_vertex_attributes: BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "8")]
    default_face_attributes: BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "9")]
    default_edge_attributes: BTreeMap<String, f64>,
    #[prost(double, repeated, tag = "13")]
    widths: Vec<f64>,
    #[prost(message, optional, tag = "15")]
    objectcolor: Option<proto::Color>,
    #[prost(int32, tag = "16")]
    color_mode: i32,
    #[prost(btree_map = "uint64, message", tag = "17")]
    triangulation: BTreeMap<u64, proto::TriList>,
    #[prost(float, repeated, tag = "18")]
    pointcolors_rgba: Vec<f32>,
    #[prost(float, repeated, tag = "19")]
    facecolors_rgba: Vec<f32>,
    #[prost(float, repeated, tag = "20")]
    linecolors_rgba: Vec<f32>,
    #[prost(bool, optional, tag = "21")]
    is_visible: Option<bool>,
    #[prost(bool, optional, tag = "22")]
    is_locked: Option<bool>,
}

pub fn encode(mesh: proto::Mesh) -> Vec<u8> {
    let expected = mesh.clone();
    let proto::Mesh {
        guid,
        name,
        vertices,
        faces,
        edge_data,
        default_vertex_attributes,
        default_face_attributes,
        default_edge_attributes,
        widths,
        objectcolor,
        color_mode,
        triangulation,
        pointcolors_rgba,
        facecolors_rgba,
        linecolors_rgba,
        is_visible,
        is_locked,
    } = mesh;
    let bytes = OrderedMesh {
        guid,
        name,
        vertices: vertices.into_iter().collect(),
        faces: faces.into_iter().collect(),
        edge_data,
        default_vertex_attributes,
        default_face_attributes,
        default_edge_attributes,
        widths,
        objectcolor,
        color_mode,
        triangulation: triangulation.into_iter().collect(),
        pointcolors_rgba,
        facecolors_rgba,
        linecolors_rgba,
        is_visible,
        is_locked,
    }
    .encode_to_vec();
    assert_eq!(proto::Mesh::decode(bytes.as_slice()).unwrap(), expected);
    bytes
}
