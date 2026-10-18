    pub fn close(&mut self) {self.active=false;self.pending.clear();}
    pub fn pending(&self)->usize {self.pending.len()}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiple_current_reads_finish_once_and_cancelled_sources_never_revive() {
        let mut source=Generation::default();assert!(source.request().is_err());source.start().unwrap();let first=source.request().unwrap();let second=source.request().unwrap();assert!(source.finish(second));assert!(!source.finish(second));assert!(source.current(first));source.close();assert!(!source.finish(first));assert!(source.request().is_err());source.start().unwrap();let newer=source.request().unwrap();assert_ne!(newer,first);assert!(!source.finish(first));assert!(source.finish(newer));assert_eq!(source.pending(),0);
    }
    #[test]
    fn exhausted_stream_generations_and_requests_do_not_wrap() {
        let mut source=Generation {generation:u64::MAX,active:true,..Default::default()};assert!(source.start().is_err());assert!(!source.active);assert!(source.request().is_err());
        let mut source=Generation {issued:u64::MAX,active:true,..Default::default()};assert!(source.request().is_err());assert_eq!(source.pending(),0);
    }
}
