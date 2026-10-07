use std::rc::Rc;
use session_rust::NurbsCurve;
use crate::marker::Marker;

#[derive(Clone)]
pub struct Control { pub source: Rc<NurbsCurve>, pub index: usize }

impl Control {
    pub fn new(source: Rc<NurbsCurve>, index: usize) -> Result<Self, &'static str> {
        let control = Self { source, index }; control.position()?; Ok(control)
    }

    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        let c = &self.source;
        if !(2..=3).contains(&c.m_dim) || !(1..=10_000).contains(&c.m_cv_count) || self.index >= c.m_cv_count {
            return Err("Control dimension, count or index is invalid");
        }
        let size = c.m_dim + usize::from(c.m_is_rat);
        if c.m_cv_stride < size { return Err("Control stride is too short"); }
        let start = self.index.checked_mul(c.m_cv_stride).ok_or("Control offset overflows")?;
        let end = start.checked_add(size).ok_or("Control range overflows")?;
        let values = c.m_cv.get(start..end).ok_or("Control values are incomplete")?;
        let weight = if c.m_is_rat { values[c.m_dim] } else { 1.0 };
        if !weight.is_finite() || weight == 0.0 { return Err("Control weight must be finite and nonzero"); }
        let mut point = [0.0; 3];
        for axis in 0..c.m_dim { point[axis] = values[axis] / weight; }
        if point.iter().any(|v| !v.is_finite()) { return Err("Control position must be finite"); }
        Ok(point)
    }

    pub fn marker(&self) -> Result<Marker, &'static str> {
        let marker = Marker { center: self.position()?.map(|v| v as f32), diameter: 6.0, colour: [0.0, 0.0, 0.0, 1.0] };
        if marker.values().iter().any(|v| !v.is_finite()) { return Err("Control marker exceeds the display range"); }
        Ok(marker)
    }
}
