use super::*;
use crate::app::mesh_preview::MeshPreview;

use crate::app::surface_preview::SurfacePreview;

/// A dragged row's previews, captured from a walk of that row alone.
pub(crate) struct Previews {
    pub mesh: Option<MeshPreview>,       // vertex keys of a mesh
    pub surface: Option<SurfacePreview>, // surface parameters of a BRep or surface
    span: Span,                          // where the captured rows sit
}

impl Scene {
    /// Capture the drag previews of `row` from a walk of it alone; nothing when its rows differ.
    pub(crate) fn capture_preview(&mut self, row: u32) {
        let span = self
            .spans
            .span(*self.feet.get(row as usize).unwrap_or(&Footprint::None));

        if self
            .preview
            .as_ref()
            .is_some_and(|(held, previews)| *held == row && previews.span == span)
        {
            return;
        }

        self.preview = None; // register:editing
        let Some((doc, guid)) = self.identity_of(row) else {
            return;
        };

        if self.docs.get(doc).is_none_or(|file| file.display_only) {
            return;
        }

        let session = Rc::clone(&self.docs[doc].session);
        let Some(geometry) = session.lookup.get(guid.as_ref()) else {
            return;
        };
        let Some(place) = self.placement_of(row) else {
            return;
        };
        let (up, _) = self.walk_one(doc, row, geometry, &place);

        if Counts::of(&up) != span.count {
            return;
        }

        let previews = Previews {
            mesh: MeshPreview::capture(&up, span, Counts::default(), geometry),
            surface: SurfacePreview::capture(&up, span, Counts::default(), geometry),
            span,
        };
        self.preview = Some((row, previews));
    }

    /// The mesh preview of a dragged row.
    pub(crate) fn mesh_preview(&self, row: u32) -> Option<&MeshPreview> {
        let (held, previews) = self.preview.as_ref()?;
        (*held == row).then_some(previews.mesh.as_ref()).flatten()
    }

    /// Drop the drag previews.
    pub(crate) fn drop_preview(&mut self) {
        self.preview = None; // register:editing
    }

    /// Re-evaluate a dragged surface into its own rows; false when the rows moved or the surface cannot.
    pub(crate) fn patch_surface(&self, row: u32, geometry: &Geometry, gpu: &mut Gpu) -> bool {
        let Some((held, previews)) = self.preview.as_ref() else {
            return false;
        };
        let span = self.spans.span(self.feet[row as usize]);

        if *held != row || previews.span != span {
            return false;
        }

        let Some(surface) = &previews.surface else {
            return false;
        };
        let Some((vertices, pipes, bounds)) = surface.evaluate(geometry) else {
            return false;
        };
        let Some(place) = self.placement_of(row) else {
            return false;
        };

        gpu.arena
            .patch_vertices(&gpu.ctx, span.start.verts, &vertices);
        gpu.segments.patch_pipes(&gpu.ctx, span.start.pipes, &pipes);
        gpu.objects
            .set_geometry_bounds(&gpu.ctx, row, bounds, 0.0, &place);
        gpu.grew_bounds(row);
        true
    }

    /// Memory held by the drag previews.
    pub fn preview_cache_bytes(&self) -> usize {
        self.preview.as_ref().map_or(0, |(_, previews)| {
            previews
                .mesh
                .as_ref()
                .map_or(0, MeshPreview::allocated_bytes)
                + previews
                    .surface
                    .as_ref()
                    .map_or(0, SurfacePreview::allocated_bytes)
        })
    }
}

impl Scene {
    /// Drop the drag preview of `row`, if it holds one.
    pub(super) fn forget_preview(&mut self, row: u32) {
        if self.preview.as_ref().is_some_and(|(held, _)| *held == row) {
            self.preview = None;
        }
    }
}
