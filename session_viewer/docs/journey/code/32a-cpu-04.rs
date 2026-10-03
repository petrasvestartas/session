use crate::{mesh::Mesh, scene::Scene};
use std::{collections::HashSet, rc::Rc};

pub fn cpu<'a>(scenes: impl IntoIterator<Item = &'a Scene>,
    gpu_displays: impl IntoIterator<Item = &'a Rc<Mesh>>) -> [usize; 6]
{
    let (mut sources, mut documents, mut displays) = (HashSet::new(), HashSet::new(), HashSet::new());
    let mut count = [0; 6];
    for scene in scenes {
        count[5] += 1;
        for object in scene.objects() {
            count[0] += 1;
            sources.insert(Rc::as_ptr(&object.geometry));
            if displays.insert(Rc::as_ptr(&object.mesh)) { count[4] += object.mesh.payload_bytes(); }
            if let Some(source) = &object.source {
                if documents.insert(Rc::as_ptr(&source.document)) {
                    for mesh in &source.document.objects.meshes { sources.insert(Rc::as_ptr(mesh)); }
                }
            }
        }
    }
    for display in gpu_displays {
        if displays.insert(Rc::as_ptr(display)) { count[4] += display.payload_bytes(); }
    }
    count[1] = sources.len(); count[2] = documents.len(); count[3] = displays.len();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_snapshots_count_spare_capacity_once() {
        let scene = Scene::demo(); let saved = scene.clone();
        let baseline = cpu([&scene], std::iter::empty());
        let shared = cpu([&scene, &saved], std::iter::empty());
        assert_eq!(shared[0], 4); assert_eq!(shared[5], 2);
        assert_eq!(&shared[1..5], &baseline[1..5]);
        let mut vertices = Vec::with_capacity(16); vertices.extend([[0.0; 6]; 3]);
        let mut indices = Vec::with_capacity(16); indices.extend([0, 1, 2]);
        let extra = Rc::new(Mesh::new(vertices, indices).unwrap());
        let retained = cpu([&scene], [&extra, &extra]);
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(retained[4], baseline[4] + 16 * 24 + 16 * 2);
    }

    #[test]
    fn a_document_retains_sources_whose_display_row_was_removed() {
        let mut scene = Scene::demo();
        scene.import(crate::document::load(&crate::specimen::bytes()).unwrap()).unwrap();
        let id = scene.objects()[2].id; scene.remove(id);
        let count = cpu([&scene], std::iter::empty());
        assert_eq!(&count[..4], &[4, 5, 1, 4]);
    }
}
