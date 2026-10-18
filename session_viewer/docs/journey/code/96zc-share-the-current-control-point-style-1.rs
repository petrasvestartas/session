pub struct ControlMarker { pub parent: ObjectId, pub part: crate::pick_sources::Part, pub display: crate::marker::Marker }
impl ControlMarker {
    fn new(parent: ObjectId, part: crate::pick_sources::Part, center: [f32; 3]) -> Self {
        Self { parent, part, display: crate::marker::Marker { center, diameter: 7., colour: [0.15, 0.35, 0.9, 1.] } }
    }
}
