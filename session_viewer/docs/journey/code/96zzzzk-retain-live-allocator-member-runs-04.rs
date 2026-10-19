pub fn bounded(members:&[Members],arena:u32)->bool {
 let ids=members.iter().map(|member|member.rows.len()as u64*(member.faces.len()as u64/3)).sum::<u64>();
 u64::from(arena)*2+ids*2+8192*(members.len()as u64+1)<u64::from(crate::primitive_image::CAP_START)
}
pub fn collect<'a>