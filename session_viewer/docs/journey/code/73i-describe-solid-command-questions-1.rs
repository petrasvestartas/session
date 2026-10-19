use crate::primitive_options as options;
use session_rust::{BRep,Geometry,Mesh,Plane,Point,Polyline,Vector,Xform};
use std::{collections::HashMap,rc::Rc,sync::OnceLock};

pub const MESH_QUALITY: (f64, f64) = (10.0, 0.002); // tessellation of a Mesh option: degrees, chord factor
pub const RING: usize = 64; // preview samples of a full circle

pub const BREP_MESH: &[(&str, &str)] = &[("Brep", "Brep"), ("Mesh", "Mesh"), ("Cancel", "Escape")];
pub const MESH_ONLY: &[(&str, &str)] = &[("Mesh", "Mesh"), ("Cancel", "Escape")];
pub const CURVE: &[(&str, &str)] = &[("Cancel", "Escape")];

/// One answer to a shape's question.
#[derive(Clone, Debug)]
pub enum Answer {
    Point(Point), // clicked or typed coordinates
    Number(f64),  // a typed value
}

/// How a question reads a bare number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Point,  // a distance along the rubber band
    Size,   // the value itself
    Height, // the value itself; clicks follow the normal through the first point
}

/// One question a shape asks.
#[derive(Clone, Copy, Debug)]
pub struct Ask {
    pub prompt: &'static str, // what the answer is for
    pub kind: Kind,           // how a number and a click count
    pub default: Option<f64>, // what Enter takes
}

impl Ask {
    /// A point question.
    pub fn point(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Point,
            default: None,
        }
    }

    /// A click or a number.
    pub fn size(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Size,
            default: None,
        }
    }

    /// A click along the normal or a number.
    pub fn height(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Height,
            default: None,
        }
    }

    /// Enter takes `value`.
    pub fn or(self, value: f64) -> Self {
        Self {
            default: Some(value),
            ..self
        }
    }
}

