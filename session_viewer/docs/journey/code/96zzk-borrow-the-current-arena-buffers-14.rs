#[test]
fn earlier_geometry_reads_current_shared_buffers_and_does_not_pin_originals() {
    use std::{cell::RefCell,rc::Rc};
    let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
    let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
    let a=Rc::new(crate::mesh::Mesh::from_kernel(&crate::cap_gpu_tests::source(false)).unwrap());
    let weak=Rc::downgrade(&a);
    let first=crate::gpu_geometry::GpuGeometry::shared(&ctx,a,arena.clone()).unwrap();
    let previous=first.buffers();
    let b=Rc::new(crate::mesh::Mesh::from_kernel(&crate::cap_gpu_tests::source(false)).unwrap());
    let second=crate::gpu_geometry::GpuGeometry::shared(&ctx,b,arena.clone()).unwrap();
    let current=first.buffers();assert_ne!(previous,current);assert_eq!(current,second.buffers());
    assert_eq!(first.index_start,0);assert_eq!(second.index_start,first.index_count);
    let expected=[crate::packed_vertex::bytes(&first.source),crate::packed_vertex::bytes(&second.source)].concat();
    assert_eq!(crate::growing_buffer_tests::read(&ctx,&current.0,expected.len()as u64),expected);
    assert_eq!(first.allocated_bytes()+second.allocated_bytes(),0);
    drop(first);assert!(weak.upgrade().is_none());
    assert_eq!(current,second.buffers());
}
