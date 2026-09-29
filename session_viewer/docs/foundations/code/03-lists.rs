fn total(values: &[u32]) -> u32 {
    let mut sum = 0;
    for value in values {
        sum += *value;
    }
    sum
}

fn main() {
    let fixed = [2, 3, 5];
    let mut growing = vec![2, 3];
    growing.push(5);

    assert_eq!(total(&fixed), 10);
    assert_eq!(total(&growing), 10);
    assert_eq!(total(&growing[1..]), 8);
}
