        for x in [bounds.min[0],bounds.max[0]] {for y in [bounds.min[1],bounds.max[1]] {for z in [bounds.min[2],bounds.max[2]] {let p=self.model.transform_point(&session_rust::Point::new(x,y,z));result.include([p[0],p[1],p[2]]);}}}result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source()->Rc<Source> {
        Rc::new(Source {properties:Default::default(),guid:"original-cloud".into(),name:"Remote original".into(),url:"points.pb".into(),fields:crate::stream::CloudFields {end:100,coords_at:10,coords_len:48,colors_at:58,colors_len:0,normals_at:0,normals_len:0,count:2,ids_at:60,ids_len:8,revision:Some("revision1".into())},lod:crate::stream::CloudLod {min:vec![1_000_000_000.125,0.,0.],size:vec![1.],spacing:vec![0.5],level:vec![0],first:vec![0],count:vec![2],children:vec![-1;8]}})
    }
    fn page()->Page {
        let raw=[1_000_000_000.125f64,0.,0.,1_000_000_000.375,0.,0.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let ids=[u32::MAX,7].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();Page::decode(0..2,2,&raw,Some(&ids)).unwrap()
    }
    #[test]
    fn streamed_rows_keep_original_descriptor_precise_bounds_and_page_identities() {
        let source=source();let weak=Rc::downgrade(&source);let budget=Budget::new(1024);let prepared=Prepared::new(source.clone(),budget.clone()).unwrap();let page=page();let charge=budget.reserve(page.bytes()).unwrap();let held=prepared.adopt(page,charge).unwrap();assert_eq!(prepared.position(u32::MAX).unwrap()[0],1_000_000_000.125);assert_eq!(prepared.position(7).unwrap()[0],1_000_000_000.375);assert_eq!(prepared.bounds.min[0],1_000_000_000.125);assert!(Rc::ptr_eq(&prepared.source,&source));
        prepared.pages.borrow_mut().clear();assert!(prepared.position(7).is_none());assert_eq!(held.value.ids,[u32::MAX,7]);assert!(budget.used()>0);drop(held);assert_eq!(budget.used(),0);drop(source);drop(prepared);assert!(weak.upgrade().is_none());
    }
    #[test]
    fn invalid_unversioned_sources_and_overlapping_pages_are_refused() {
        let budget=Budget::new(1024);let prepared=Prepared::new(source(),budget.clone()).unwrap();let original=page();let charge=budget.reserve(original.bytes()).unwrap();drop(prepared.adopt(original,charge).unwrap());let raw=[1_000_000_000.375f64,0.,0.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let overlap=Page::decode(1..2,2,&raw,None).unwrap();let charge=budget.reserve(overlap.bytes()).unwrap();assert!(prepared.adopt(overlap,charge).is_err());assert_eq!(prepared.pages.borrow().len(),1);
        let mut unversioned=source();Rc::get_mut(&mut unversioned).unwrap().fields.revision=None;assert!(Prepared::new(unversioned,budget).is_err());
    }
}
