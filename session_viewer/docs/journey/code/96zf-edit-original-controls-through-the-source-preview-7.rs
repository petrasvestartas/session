fn same_source(a:&Scene,b:&Scene,id:ObjectId) -> bool {
    crate::source_edit::original(a,id).zip(crate::source_edit::original(b,id)).is_some_and(|(a,b)|crate::source_edit::same(&a,&b))
}
