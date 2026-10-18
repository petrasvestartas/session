    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pages_keep_original_f64_coordinates_high_ids_and_fallback_rows() {
        let raw=[1_000_000_000.125f64,2.,3.,1_000_000_000.375,4.,5.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();
        let ids=[u32::MAX,7].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();let page=Page::decode(8..10,20,&raw,Some(&ids)).unwrap();
        assert_eq!(page.coordinates[0],1_000_000_000.125);assert_eq!(page.coordinates[3],1_000_000_000.375);assert_eq!(page.display[0],page.display[3]);assert_eq!(page.ids,[u32::MAX,7]);assert_eq!(Page::decode(8..10,20,&raw,None).unwrap().ids,[8,9]);
        assert!(Page::decode(8..10,20,&raw[..47],Some(&ids)).is_err());assert!(Page::decode(8..10,20,&raw,Some(&ids[..7])).is_err());assert!(Page::decode(8..10,9,&raw,None).is_err());assert!(Page::decode(10..8,20,&raw,None).is_err());
        let duplicates=[7u32,7].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();assert!(Page::decode(8..10,20,&raw,Some(&duplicates)).is_err());
    }
    #[test]
    fn invalid_original_doubles_never_become_a_display_page() {
        for value in [f64::NAN,f64::INFINITY,f64::MAX] {let raw=[value,0.,0.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();assert!(Page::decode(0..1,1,&raw,None).is_err());}
    }
}
