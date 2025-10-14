#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{Line, Point, Vector};

    #[test]
    fn test_line_default_constructor() {
        let line = Line::default();
        assert_eq!(line.z1(), 1.0);
        assert_eq!(line.name, "my_line");
    }

    #[test]
    fn test_line_constructor() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(line.x0(), 1.0);
        assert_eq!(line.z1(), 6.0);
    }

    #[test]
    fn test_line_from_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(4.0, 5.0, 6.0);
        let line = Line::from_points(&p1, &p2);
        assert_eq!(line.y0(), 2.0);
        assert_eq!(line.y1(), 5.0);
    }

    #[test]
    fn test_line_with_name() {
        let line = Line::with_name("custom", 0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert_eq!(line.name, "custom");
    }

    #[test]
    fn test_line_to_string() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let s = format!("{line}");
        assert!(s.contains("1"));
    }

    #[test]
    fn test_line_operator_subscript() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(line[0], 1.0);
        assert_eq!(line[5], 6.0);
    }

    #[test]
    fn test_line_operator_subscript_mutable() {
        let mut line = Line::default();
        line[0] = 10.0;
        assert_eq!(line.x0(), 10.0);
    }

    #[test]
    fn test_line_operator_add_assign() {
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        line += &v;
        assert_eq!(line.x0(), 1.0);
        assert_eq!(line.z1(), 4.0);
    }

    #[test]
    fn test_line_operator_sub_assign() {
        let mut line = Line::new(1.0, 2.0, 3.0, 2.0, 3.0, 4.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        line -= &v;
        assert_eq!(line.x0(), 0.0);
    }

    #[test]
    fn test_line_operator_mul_assign() {
        let mut line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        line *= 2.0;
        assert_eq!(line.x0(), 2.0);
        assert_eq!(line.z1(), 12.0);
    }

    #[test]
    fn test_line_operator_div_assign() {
        let mut line = Line::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0);
        line /= 2.0;
        assert_eq!(line.x0(), 1.0);
        assert_eq!(line.z1(), 6.0);
    }

    #[test]
    fn test_line_operator_add() {
        let line = Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = line + &v;
        assert_eq!(result.y0(), 2.0);
    }

    #[test]
    fn test_line_operator_sub() {
        let line = Line::new(1.0, 2.0, 3.0, 2.0, 3.0, 4.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = line - &v;
        assert_eq!(result.x0(), 0.0);
    }

    #[test]
    fn test_line_operator_mul() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let result = line * 2.0;
        assert_eq!(result.x0(), 2.0);
    }

    #[test]
    fn test_line_operator_div() {
        let line = Line::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0);
        let result = line / 2.0;
        assert_eq!(result.z1(), 6.0);
    }

    #[test]
    fn test_line_to_vector() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 6.0, 9.0);
        let v = line.to_vector();
        assert_eq!(v.x(), 3.0);
        assert_eq!(v.z(), 6.0);
    }

    #[test]
    fn test_line_length() {
        let line = Line::new(0.0, 0.0, 0.0, 3.0, 4.0, 0.0);
        assert!((line.length() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_line_squared_length() {
        let line = Line::new(0.0, 0.0, 0.0, 3.0, 4.0, 0.0);
        assert!((line.squared_length() - 25.0).abs() < 1e-5);
    }

    #[test]
    fn test_line_point_at() {
        let line = Line::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let p = line.point_at(0.5);
        assert!((p.x() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_line_start() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let p = line.start();
        assert_eq!(p.x(), 1.0);
    }

    #[test]
    fn test_line_end() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let p = line.end();
        assert_eq!(p.x(), 4.0);
    }

    #[test]
    fn test_line_to_json_data() {
        let mut line = Line::new(1.5, 2.5, 3.5, 4.5, 5.5, 6.5);
        line.name = "test".to_string();
        let json = line.jsondump().unwrap();
        assert!(json.contains("Line"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_line_from_json_data() {
        let orig = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let json = orig.jsondump().unwrap();
        let restored = Line::jsonload(&json).unwrap();
        assert_eq!(restored.name, "my_line");
        assert_eq!(restored.x0(), 1.0);
    }

    #[test]
    fn test_line_to_json_from_json() {
        let mut orig = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        orig.name = "serialized".to_string();
        json_dump(&orig, "test_line.json", true).unwrap();
        let loaded: Line = json_load("test_line.json").unwrap();
        assert_eq!(loaded.name, orig.name);
        assert_eq!(loaded.x0(), orig.x0());
        assert_eq!(loaded.z1(), orig.z1());
    }
}
