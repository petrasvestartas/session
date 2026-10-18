use session_rust::{Element, Mesh, Point, Polyline, Session, TreeNode, Xform};
use session_rust::element::ElementFeature;
use session_rust::{Interaction, InteractionUnknown};
use prost::Message;

pub fn session() -> Session {
    let line = Polyline::new(vec![Point::new(0., 0., 0.), Point::new(100., 0., 0.)]);
    let mut session = Session::new("interaction exercise");
    let mut host = Element::new("host");
    host.set_geometry(Mesh::create_box(100., 100., 100.));
    host.add_feature(ElementFeature::new("axis", -1, vec![line.clone()], "axis"));
    let contact = InteractionUnknown::new("InteractionContactFace", &[0, 255, 7], "seam");
    let mut touch = ElementFeature::new("contact", 2, vec![line.clone(), line.clone()], "side_side");
    touch.set_guid(contact.guid().to_owned());
    host.add_feature(touch);
    let mut source = Element::new("connector");
    source.set_geometry(Mesh::create_box(20., 20., 20.));
    let mut drill = ElementFeature::new("drill", -1, vec![line.clone()], "d10");
    drill.set_guid(format!("{}-drill-0", source.guid()));
    host.add_feature(drill);
    host.add_feature(ElementFeature::new("cut", 5, vec![line.clone()], "unclaimed"));
    let host = session.add_element(host, None);
    session.add(&TreeNode::new("attributes"), Some(&host));
    let attributes = host.borrow().children().last().unwrap().clone();
    let mut child = Element::new("child");
    child.set_geometry(Mesh::create_box(15., 15., 15.));
    child.add_feature(ElementFeature::new("axis", -1, vec![line], "axis"));
    let child = session.add_element(child, Some(&attributes));
    let source = session.add_element(source, None);
    let host = host.borrow().name.clone();
    let child = child.borrow().name.clone();
    let source = source.borrow().name.clone();
    session.xforms.insert(host.clone(), Xform::translation(10., 20., 30.));
    session.xforms.insert(child, Xform::translation(2., 3., 4.));
    session.xforms.insert(source.clone(), Xform::translation(130., 20., 30.));
    session.graph.add_edge(&source, &host, "connection");
    let edge = session.graph.edges[&source][&host].guid().to_owned();
    session.interactions.insert(edge, vec![Box::new(contact), Box::new(
        InteractionUnknown::new("InteractionFeatureSolid", &[8, 9], ""))]);
    session
}

pub fn bytes() -> Vec<u8> { session().to_proto().encode_to_vec() }
