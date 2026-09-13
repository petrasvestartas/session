use session_rust::Line;
use session_rust::Point;
use session_rust::Polyline;
use session_rust::Session;
use session_rust::TreeNode;
use session_rust::Xform;

fn main() {
    let mut session = Session::new("Editing lessons");
    let assembly = session.add_group("Assembly");
    let nested = TreeNode::new("Nested");
    session.add(&nested, Some(&assembly));
    let mut a = Line::new(-100.0, 0.0, 0.0, -20.0, 0.0, 0.0);
    a.name = "Beam A".into();
    let a = session.add_line(a, Some(&nested));
    let mut b = Line::new(-100.0, 60.0, 0.0, -20.0, 60.0, 0.0);
    b.name = "Beam B".into();
    let b = session.add_line(b, Some(&nested));
    session.add_edge(&a.borrow().name, &b.borrow().name, "joint");
    let mut line = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(60.0, 0.0, 0.0),
        Point::new(60.0, 60.0, 0.0),
    ]);
    line.name = "Placed polyline".into();
    let node = session
        .add_polyline(line, Some(&assembly))
        .expect("three points");
    session.set_xform(
        &node.borrow().name,
        &Xform::translation(20.0, 0.0, 0.0) * &Xform::scale_xyz(1.5, 1.5, 1.5),
    );
    session.pb_dump("docs/extensions/nested.pb");
}
