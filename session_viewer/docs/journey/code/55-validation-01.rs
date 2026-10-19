use session_rust::NurbsSurface;

pub fn validate(source:&NurbsSurface)->Result<[[f64;2];2],&'static str> {
    if !(2..=3).contains(&source.m_dim) {return Err("Surface dimension must be two or three");}
    let count=source.m_cv_count[0].checked_mul(source.m_cv_count[1]).ok_or("Surface control count overflows")?;
    if count>65_536||source.m_cv_count.iter().any(|n|*n<2) {return Err("Surface control count exceeds the display budget");}
    let size=source.m_dim+usize::from(source.m_is_rat);
    if source.m_cv_stride.iter().any(|stride|*stride<size) {return Err("Surface control stride is too short");}
    let row_major=source.m_cv_stride[1].checked_mul(source.m_cv_count[1]).is_some_and(|span|source.m_cv_stride[0]>=span);
    let column_major=source.m_cv_stride[0].checked_mul(source.m_cv_count[0]).is_some_and(|span|source.m_cv_stride[1]>=span);
    if !row_major&&!column_major {return Err("Surface controls overlap in storage");}
    let end=(source.m_cv_count[0]-1).checked_mul(source.m_cv_stride[0]).and_then(|n|(source.m_cv_count[1]-1).checked_mul(source.m_cv_stride[1]).and_then(|m|n.checked_add(m))).and_then(|n|n.checked_add(size)).ok_or("Surface control range overflows")?;
    if end>source.m_cv.len() {return Err("Surface controls are incomplete");}
    for i in 0..source.m_cv_count[0] {for j in 0..source.m_cv_count[1] {
        let start=i*source.m_cv_stride[0]+j*source.m_cv_stride[1];let values=&source.m_cv[start..start+size];
        if values.iter().any(|v|!v.is_finite())||source.m_is_rat&&values[source.m_dim]<=0. {return Err("Surface controls need finite coordinates and positive weights");}
    }}
    let mut domains=[[0.;2];2];
    for dir in 0..2 {
        let order=source.m_order[dir];let count=source.m_cv_count[dir];let knots=&source.m_nurbsknot[dir];
        if !(2..=32).contains(&order)||order>count||knots.len()!=count+order-2||knots.iter().any(|v|!v.is_finite())||knots.windows(2).any(|pair|pair[0]>pair[1]) {return Err("Surface order or knots are invalid");}
        let (a,b)=source.domain(dir).ok_or("Surface domain is missing")?;
        if a>=b||!(b-a).is_finite() {return Err("Surface domain must have finite positive length");}domains[dir]=[a,b];
    }
    Ok(domains)
}

