use session_rust::NurbsCurve;

pub fn validate(source: &NurbsCurve) -> Result<(f64, f64), &'static str> {
    if !(2..=3).contains(&source.m_dim) || !(2..=10_000).contains(&source.m_cv_count)
        || !(2..=32).contains(&source.m_order) || source.m_order > source.m_cv_count {
        return Err("Curve dimension, count or order is invalid");
    }
    let size = source.m_dim + usize::from(source.m_is_rat);
    if source.m_cv_stride < size { return Err("Curve control stride is too short"); }
    let end = (source.m_cv_count - 1).checked_mul(source.m_cv_stride)
        .and_then(|offset| offset.checked_add(size)).ok_or("Curve control range overflows")?;
    if source.m_cv.len() < end { return Err("Curve control values are incomplete"); }
    for index in 0..source.m_cv_count {
        let start = index * source.m_cv_stride;
        let values = &source.m_cv[start..start + size];
        if values.iter().any(|v| !v.is_finite()) { return Err("Curve controls must be finite"); }
        if source.m_is_rat && values[source.m_dim] <= 0.0 { return Err("Curve display requires positive rational weights"); }
    }
    let count = source.m_cv_count + source.m_order - 2;
    if source.m_nurbsknot.len() != count || source.m_nurbsknot.iter().any(|v| !v.is_finite())
        || source.m_nurbsknot.windows(2).any(|pair| pair[0] > pair[1]) {
        return Err("Curve knot vector is incomplete or unordered");
    }
    let domain = source.domain();
    if domain.0 >= domain.1 || !(domain.1 - domain.0).is_finite() { return Err("Curve domain must have positive length"); }
    Ok(domain)
}
