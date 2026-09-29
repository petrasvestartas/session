use std::sync::Arc;

trait Measure {
    fn amount(&self) -> usize;
}

impl Measure for Vec<u32> {
    fn amount(&self) -> usize {
        self.len()
    }
}

fn main() {
    let values = Arc::new(vec![2, 4, 6]);
    let shared = Arc::clone(&values);
    let report = move || shared.amount();

    assert_eq!(report(), 3);
    assert_eq!(values.amount(), 3);
    assert_eq!(Arc::strong_count(&values), 2);
}
