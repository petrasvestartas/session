pub(crate) fn inverse(m: &Xform) -> Option<Xform> {
    if m.m.iter().any(|v|!v.is_finite()){return None;}