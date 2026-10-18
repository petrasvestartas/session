use crate::{instance_plan::Owner, scene::{ObjectId, Scene}};
use session_rust::{Element, Geometry};

pub fn append(scene: &mut Scene, id: ObjectId, owner: &Owner) -> Result<(), &'static str> {
    let mut display = Element::new(&owner.instance.name);
    display.set_guid(owner.instance.guid().to_owned());
    if let Geometry::Element(definition) = &owner.definition {
        display.features.extend(definition.features.iter().cloned());
    }
    display.features.extend(owner.instance.features.iter().cloned());
    crate::feature_display::append(scene, id, &owner.document, &display)
}
