pub fn bytes() -> Vec<u8> {
    use session_rust::{Color, Mesh, Session, Xform};
    let mut session = Session::new("Three-piece frame");
    for (name, size, centre) in [
        ("Left post", [0.25, 0.35, 1.0], [-0.65, 0.0, 0.5]),
        ("Right post", [0.25, 0.35, 1.0], [0.65, 0.0, 0.5]),
        ("Top beam", [1.8, 0.35, 0.25], [0.0, 0.0, 1.125]),
    ] {
        let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
        mesh.name = name.to_owned();
        mesh.transform(&Xform::translation(centre[0], centre[1], centre[2]));
        mesh.set_objectcolor(Color::new(0.75, 0.35, 0.1, 1.0));
        assert!(session.add_mesh(mesh, None).is_some());
    }
    session.pb_dumps()
}

pub fn precise_bytes() -> Vec<u8> {
    use prost::Message;
    let mut message = session_rust::proto::Session::decode(bytes().as_slice()).unwrap();
    let mesh = &mut message.objects.as_mut().unwrap().meshes[0];
    mesh.vertices.get_mut(&0).unwrap().x = -0.775000000123456;
    message.encode_to_vec()
}
