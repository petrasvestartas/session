fn doubled(value: f32) -> f32 {
    value * 2.0
}

fn main() {
    let width = 320.0;
    let mut height = 200.0;
    height += 40.0;

    assert_eq!(doubled(width), 640.0);
    assert_eq!(height, 240.0);
}
