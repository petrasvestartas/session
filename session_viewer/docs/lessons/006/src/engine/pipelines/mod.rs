// --8<-- [start:006-cache]

use crate::engine::gpu::buffers::GpuCtx;
use std::cell::{Cell, LazyCell, RefCell};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// `thread_local!` = a global, one per thread; `Cell` lets it change through a shared reference. The browser runs one thread.
thread_local! {
    /// Render and compute pipelines compiled on this thread so far.
    static PIPELINES: Cell<u32> = const { Cell::new(0) };

    /// Shader modules compiled on this thread so far.
    static SHADERS: Cell<u32> = const { Cell::new(0) };
}

/// Count one compiled pipeline.
pub fn count_pipeline() {
    PIPELINES.set(PIPELINES.get() + 1);
}

/// Count one compiled shader module.
pub fn count_shader() {
    SHADERS.set(SHADERS.get() + 1);
}

/// (pipelines, shader modules) compiled on this thread so far.
pub fn created() -> (u32, u32) {
    (PIPELINES.get(), SHADERS.get())
}

// `Rc` = shared ownership, freed when the last clone is dropped; `LazyCell` runs its closure on first use and keeps the result.
// `Box<dyn FnOnce() -> T>` = any closure that runs once, whatever its concrete type, kept on the heap.
/// A GPU object made on its first use; clones share it, and equal means the same one.
pub struct Lazy<T>(Rc<LazyCell<T, Box<dyn FnOnce() -> T>>>);

impl<T> Lazy<T> {
    /// Made by `make` when first used.
    // `impl FnOnce() -> T` accepts any closure of that shape; `'static` = it borrows nothing that could be dropped first.
    pub fn new(make: impl FnOnce() -> T + 'static) -> Self {
        Self(Rc::new(LazyCell::new(Box::new(make))))
    }
}

impl<T> Clone for Lazy<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> std::ops::Deref for Lazy<T> {
    type Target = T;

    /// The object, made now if this is its first use.
    fn deref(&self) -> &T {
        LazyCell::force(&self.0)
    }
}

impl<T> PartialEq for Lazy<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Lazy<T> {}

impl<T> Hash for Lazy<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

/// A render pipeline, compiled when a pass first sets it.
pub type Pipeline = Lazy<wgpu::RenderPipeline>;

/// A shader module, compiled when a pipeline first needs it.
pub type Shader = Lazy<wgpu::ShaderModule>;

// `RefCell` moves the borrow check to run time, so methods on `&self` can still insert.
// `#[derive(Default)]` writes the Default impl for us: every field starts empty.
/// Shaders, layouts and pipelines by description: asking again, or a second lane of the same
/// kind asking, returns the first one, so an MSAA flip back and forth compiles nothing twice.
#[derive(Default)]
pub struct Cache {
    shaders: RefCell<HashMap<ShaderKey, Shader>>, // by label and source digest
    layouts: RefCell<HashMap<LayoutKey, wgpu::BindGroupLayout>>, // by label and entries
    pipelines: RefCell<HashMap<PipelineKey, Pipeline>>, // by everything they compile from
}

impl Cache {
    /// Compile every pipeline asked for so far; returns how many there are.
    pub fn compile_all(&self) -> usize {
        let pipelines = self.pipelines.borrow();

        for pipeline in pipelines.values() {
            let _: &wgpu::RenderPipeline = pipeline;
        }

        pipelines.len()
    }
}

/// How a pipeline uses depth; reverse-Z, so nearer is greater.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DepthMode {
    Opaque,        // write, nearer wins
    OpaqueEqual,   // write, nearer or equal wins
    ReadOnly,      // test only, nearer wins
    ReadOnlyEqual, // test only, nearer or equal wins
    Always,        // no test, no write
    Detached,      // no depth attachment at all
}

impl DepthMode {
    /// The (write, compare) pair wgpu wants.
    fn state(self) -> (bool, wgpu::CompareFunction) {
        match self {
            DepthMode::Opaque => (true, wgpu::CompareFunction::Greater),
            DepthMode::OpaqueEqual => (true, wgpu::CompareFunction::GreaterEqual),
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnlyEqual => (false, wgpu::CompareFunction::GreaterEqual),
            DepthMode::Always | DepthMode::Detached => (false, wgpu::CompareFunction::Always),
        }
    }
}

/// How a pipeline writes color.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ColorWrite {
    Opaque,  // overwrite
    Blended, // alpha blend
    Nothing, // write nothing; the fragment shader has side effects
}

impl ColorWrite {
    /// The (blend, write mask) pair wgpu wants.
    fn state(self) -> (Option<wgpu::BlendState>, wgpu::ColorWrites) {
        match self {
            ColorWrite::Opaque => (None, wgpu::ColorWrites::ALL),
            ColorWrite::Blended => (
                Some(wgpu::BlendState::ALPHA_BLENDING),
                wgpu::ColorWrites::ALL,
            ),
            ColorWrite::Nothing => (None, wgpu::ColorWrites::empty()),
        }
    }
}

/// A shader's label, source length and source hash: the text itself is not kept.
type ShaderKey = (String, usize, u64);

/// A bind group layout's label and entries.
type LayoutKey = (String, Vec<wgpu::BindGroupLayoutEntry>);

/// Everything a render pipeline compiles from, owned, so the compile can wait for its first use.
#[derive(Clone, PartialEq, Eq, Hash)]
struct PipelineKey {
    label: String,                      // name shown in GPU errors
    shader: Shader,                     // vertex and fragment code
    vs: String,                         // vertex entry point
    fs: String,                         // fragment entry point
    groups: Vec<wgpu::BindGroupLayout>, // bind group layouts, in slot order
    buffers: Vec<(u64, wgpu::VertexStepMode, Vec<wgpu::VertexAttribute>)>, // vertex buffer layouts
    topology: wgpu::PrimitiveTopology,  // triangles or lines
    color: ColorWrite,                  // how color is written
    depth: DepthMode,                   // how depth is used
    target: Target,                     // color format and samples
}

/// Where a pipeline draws: color format and MSAA sample count.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Target {
    pub format: wgpu::TextureFormat, // color format
    pub samples: u32,                // MSAA samples
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    /// Clones share one object, made once, on its first use.
    fn lazy_objects_are_made_once_on_first_use() {
        let made = Rc::new(Cell::new(0));
        let counter = made.clone();
        let lazy = Lazy::new(move || {
            counter.set(counter.get() + 1);
            7
        });
        let copy = lazy.clone();
        assert_eq!(made.get(), 0, "nothing is made before its first use");
        assert_eq!(*copy, 7);
        assert_eq!(*lazy, 7);
        assert_eq!(made.get(), 1, "clones share the one made");
        assert!(lazy == copy);
        assert!(lazy != Lazy::new(|| 7), "equal means the same object");
    }
}
// --8<-- [end:006-cache]
