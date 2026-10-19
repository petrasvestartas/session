    pub fn used(&self)->usize {self.budget.used()}
}

#[cfg(test)]
mod tests {
    use super::*;
    fn add(cache:&mut Cache<u32,Vec<u8>>,key:u32,bytes:usize)->Rc<Held<Vec<u8>>> {let charge=cache.reserve(bytes).unwrap();cache.insert(key,vec![key as u8;bytes],charge).unwrap()}
    #[test]
    fn lru_eviction_reuses_room_without_dropping_live_renderer_pages() {
        let budget=Budget::new(12);let mut cache=Cache::new(budget.clone());let first=add(&mut cache,1,4);drop(add(&mut cache,2,4));drop(add(&mut cache,3,4));assert_eq!(budget.used(),12);
        assert!(cache.remove(&1));assert_eq!(first.value,[1;4]);assert_eq!(budget.used(),12);
        drop(add(&mut cache,4,4));assert!(cache.get(&2).is_none());assert!(cache.get(&3).is_some());assert_eq!(budget.used(),12);
        cache.clear();assert_eq!(budget.used(),4);drop(first);assert_eq!(budget.used(),0);
    }
    #[test]
    fn pinned_pages_and_failed_reservations_preserve_the_current_cache() {
        let budget=Budget::new(8);let mut cache=Cache::new(budget.clone());let first=add(&mut cache,1,4);let second=add(&mut cache,2,4);
        assert!(cache.reserve(4).is_err());assert_eq!(cache.len(),2);assert_eq!(budget.used(),8);
        drop(first);drop(second);let charge=cache.reserve(6).unwrap();assert_eq!(cache.len(),0);assert_eq!(budget.used(),6);drop(charge);assert_eq!(budget.used(),0);
    }
    #[test]
    fn pending_bytes_are_reserved_before_allocation_and_failed_work_releases_them() {
        let budget=Budget::new(12);let mut pending=budget.reserve(10).unwrap();assert_eq!(budget.used(),10);assert!(pending.resize(13).is_err());assert_eq!(pending.bytes(),10);pending.resize(6).unwrap();assert_eq!(budget.used(),6);drop(pending);assert_eq!(budget.used(),0);
        let other=Budget::new(12);let mut cache=Cache::new(budget.clone());assert!(cache.insert(1u32,vec![0u8],other.reserve(1).unwrap()).is_err());assert_eq!(other.used(),0);assert_eq!(budget.used(),0);
    }
}
