struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let size = Size { width: 4, height: 3 };
    assert_eq!(size.area(), 12);
    assert_eq!(size.width, 4);
}
