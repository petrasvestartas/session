use session_rust::Mesh;
use session_rust::brep::{BRep,BRepOrientation};

fn sample_order(a: &f64, b: &f64) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

fn parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    sample_order(&a.0, &b.0).then(a.1.cmp(&b.1))
}

fn total_parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    a.0.total_cmp(&b.0).then(a.1.cmp(&b.1))
}

fn same_parameter(a: &mut (f64, usize), b: &mut (f64, usize)) -> bool {
    a.0 == b.0
}

fn parse_sample_index(value: &str) -> Option<usize> {
    value.parse().ok()
}

pub struct EdgeUse {
    pub edge: usize,
    pub face: usize,
    pub orientation: BRepOrientation,
}

fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);

    if ci < 0 {
        return None;
    }

    let c = b.m_curves_2d.get(ci as usize)?;

    if c.degree() != 1 || c.is_rational() || c.cv_count() != 2 {
        return None;
    }

    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

fn sample_values(fm: &Mesh, name: &str) -> Vec<f64> {
    let mut vals = Vec::new();

    for vertex in fm.vertex.values() {
        if let Some(value) = vertex.attributes.get(name) {
            vals.push(*value);
        }
    }

    vals.sort_unstable_by(sample_order);
    vals.dedup();
    vals
}

fn nearest_sample(vals: &[f64], target: f64, wrap: Option<(f64, f64)>) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;

    for &v in vals {
        let mut d = (v - target).abs();

        if let Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }

        if match best {
            Some((bd, _)) => d < bd,
            None => true,
        } {
            best = Some((d, v));
        }
    }

    Some(best?.1)
}

pub struct EdgeChain {
    pub edge: usize,
    pub face: usize,
    pub keys: Vec<usize>,
    pub other: Option<usize>,
}

