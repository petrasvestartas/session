
impl crate::page_cache::ByteSize for Page {fn bytes(&self)->usize {Page::bytes(self)}}

#[cfg(test)]
mod attribute_tests {
    use super::*;
    #[test]
    fn original_page_attributes_keep_exact_words_and_refuse_partial_columns() {
        let raw=[0.125f64,0.,0.,0.375,0.,0.].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();
        let page=Page::decode(0..2,2,&raw,None).unwrap().attributes(vec![0xff00_00ff,0x80ff_0000],vec![0x1234_5678,0xffff_0000]).unwrap();assert_eq!(page.colors,[0xff00_00ff,0x80ff_0000]);assert_eq!(page.normals,[0x1234_5678,0xffff_0000]);assert_eq!(page.coordinates[0],0.125);assert_eq!(page.bytes(),96);let tail=page.slice(1..2).unwrap();assert_eq!(tail.coordinates,[0.375,0.,0.]);assert_eq!(tail.ids,[1]);assert_eq!(tail.colors,[0x80ff_0000]);assert_eq!(tail.normals,[0xffff_0000]);assert_eq!(tail.bytes(),48);assert!(page.slice(2..3).is_err());
        assert!(Page::decode(0..2,2,&raw,None).unwrap().attributes(vec![1],vec![]).is_err());assert!(Page::decode(0..2,2,&raw,None).unwrap().attributes(vec![],vec![1]).is_err());
    }
}
