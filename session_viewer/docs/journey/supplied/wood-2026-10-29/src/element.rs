use prost::Message;
use session_rust::element::ElementFeature;
use session_rust::{BRep, Element, Mesh, Plane, Polyline};

/// A feature of one polyline named by its type, as C++ polyline_feature; -1 is the whole element.
pub fn polyline_feature(
    feature_type: &str,
    polyline: &Polyline,
    face_index: i32,
) -> ElementFeature {
    ElementFeature::new(
        feature_type,
        face_index,
        vec![polyline.clone()],
        feature_type,
    )
}

/// A wood class carried by a kernel Element: `element_type` names it, `element_data` holds its payload.
pub trait WoodElement: Sized {

    /// The element_type the class is written under, as the C++ class registers it.
    const TYPE: &'static str;

    /// An older element_type still read as this class; empty when there is none.
    const LEGACY_TYPE: &'static str = "";

    /// The payload message element_data holds.
    type Payload: Message + Default + PartialEq + std::fmt::Debug;

    /// The class from its name and payload; None when the payload cannot describe one.
    fn from_payload(name: &str, payload: Self::Payload) -> Option<Self>;

    /// The payload message of this instance.
    fn payload(&self) -> Self::Payload;

    /// The element name.
    fn name(&self) -> &str;

    /// The plain solid as a mesh, before cut planes, solid features and joinery.
    fn solid(&self) -> Mesh;

    /// The plain solid as an exact BRep, as the C++ class writes it.
    fn brep(&self) -> BRep;

    /// The plane the element is laid out on, drawn as its attribute; None for none.
    fn base_plane(&self) -> Option<Plane>;

    /// What describes the element beside its solid, its axis, sections or outlines, as C++ compute_geometry_features writes them uncut; none by default.
    fn features(&self) -> Vec<ElementFeature> {
        Vec::new()
    }

    /// Downcast: the class an Element tagged TYPE describes; None for another type or an unreadable payload.
    fn from_element(element: &Element) -> Option<Self> {
        let tag = element.element_type_name();

        if tag != Self::TYPE && (Self::LEGACY_TYPE.is_empty() || tag != Self::LEGACY_TYPE) {
            return None;
        }

        Self::from_payload(
            &element.name,
            Self::Payload::decode(element.element_data_dumps()).ok()?,
        )
    }

    /// Upcast: a kernel Element named and tagged like the class, its payload and its BRep.
    fn to_element(&self) -> Element {
        let mut element = Element::new(self.name());
        element.element_type = Self::TYPE.to_string();
        element.element_data = self.payload().encode_to_vec();
        element.set_brep_geometry(self.brep());
        element.features = self.features();
        element
    }
}
