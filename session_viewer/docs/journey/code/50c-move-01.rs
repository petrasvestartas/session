use crate::{marker::PreparedPoint, stroke::PreparedLine};
use std::rc::Rc;

pub enum Built { Point(PreparedPoint), Line(PreparedLine), Move([f64; 3]) }

pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
