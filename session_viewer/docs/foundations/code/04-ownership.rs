fn append(values: &mut Vec<u32>, value: u32) {
    values.push(value);
}

fn consume(values: Vec<u32>) -> usize {
    values.len()
}

fn main() {
    let mut values = vec![4, 8];
    append(&mut values, 12);
    assert_eq!(values.len(), 3);

    let count = consume(values);
    assert_eq!(count, 3);
}
