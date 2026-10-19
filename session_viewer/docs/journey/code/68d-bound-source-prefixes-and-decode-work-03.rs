}
pub fn temporary_page_bytes(count:u32)->Option<usize> {usize::try_from(count).ok()?.checked_mul(160)}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn current_prefix_chunk_ceiling_and_overflow_rules_are_explicit() {
        assert_eq!(prefix(10_000_000,2_000_000,6_000_000),2_000_000);assert_eq!(prefix(10_000_000,2_000_000,0),MIN_PREFIX);assert_eq!(prefix(100,2_000_000,0),100);assert_eq!(next(2_000_000,10_000_000,500_000),Some(2_000_000..2_500_000));assert_eq!(next(u32::MAX-3,u32::MAX,10),Some(u32::MAX-3..u32::MAX));assert_eq!(next(0,1,0),None);assert_eq!(temporary_page_bytes(2_000_000),Some(320_000_000));
    }
}
