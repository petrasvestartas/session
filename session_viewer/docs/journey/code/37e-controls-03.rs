use crate::controls::Control;
use session_rust::NurbsCurve;
use std::rc::Rc;

#[test]
fn rational_controls_share_original_identity_and_keep_homogeneous_precision() {
    let mut curve = NurbsCurve::new(3, true, 2, 2);
    curve.m_cv = vec![0.24691357802469, 0.8, 0.0, 2.0, 2.0, 0.0, 0.0, 1.0];
    let source = Rc::new(curve); let guid = source.guid().to_owned(); let weak = Rc::downgrade(&source);
    let control = Control::new(Rc::clone(&source), 0).unwrap(); let other = Control::new(Rc::clone(&source), 1).unwrap();
    assert!(Rc::ptr_eq(&control.source, &other.source)); assert_eq!(control.index, 0); assert_eq!(other.index, 1);
    assert_eq!(control.position().unwrap(), [0.123456789012345, 0.4, 0.0]);
    assert_eq!(control.source.m_cv[0], 0.24691357802469); assert_eq!(control.source.guid(), guid);
    assert_ne!(control.marker().unwrap().center[0] as f64, control.position().unwrap()[0]);
    drop(source); drop(control); assert!(weak.upgrade().is_some()); drop(other); assert!(weak.upgrade().is_none());
}

#[test]
fn control_layout_checks_refuse_malformed_ranges_without_panicking() {
    for mutate in [0, 1, 2, 3, 4, 5, 6] {
        let mut curve = NurbsCurve::new(3, true, 2, 2);
        curve.m_cv = vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];
        match mutate {
            0 => curve.m_dim = 4, 1 => curve.m_cv_stride = 2, 2 => curve.m_cv.clear(),
            3 => curve.m_cv_stride = usize::MAX, 4 => curve.m_cv[7] = 0.0,
            5 => curve.m_cv[7] = f64::NAN, _ => curve.m_cv[4] = f64::INFINITY,
        }
        assert!(Control::new(Rc::new(curve), 1).is_err());
    }
    let source = Rc::new(NurbsCurve::new(3, false, 2, 2)); assert!(Control::new(source, 2).is_err());
    let mut curve = NurbsCurve::new(2, false, 2, 2); curve.m_cv = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(Control::new(Rc::new(curve), 1).unwrap().position().unwrap(), [3.0, 4.0, 0.0]);
    let mut curve = NurbsCurve::new(3, false, 2, 2); curve.m_cv[0] = 1e100;
    let control = Control::new(Rc::new(curve), 0).unwrap(); assert!(control.marker().is_err());
}
