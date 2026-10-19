    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,