#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn test_color_constructor() {
        let mut red = Color::new(255, 0, 0, 255);
        red.name = "red".to_string();
        assert_eq!(red.name, "red");
        assert!(!red.guid.to_string().is_empty());
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);
        assert_eq!(red.a, 255);
    }

    #[test]
    fn test_color_equality() {
        let c1 = Color::new(0, 100, 50, 200);
        let c2 = Color::new(0, 100, 50, 200);
        // Colors have different GUIDs, so they won't be equal by default
        // We'll compare the RGBA values instead
        assert_eq!(c1.r, c2.r);
        assert_eq!(c1.g, c2.g);
        assert_eq!(c1.b, c2.b);
        assert_eq!(c1.a, c2.a);

        let c3 = Color::new(0, 100, 50, 200);
        let c4 = Color::new(1, 100, 50, 200);
        assert_ne!(c3.r, c4.r);
    }

    #[test]
    fn test_color_to_json_data() {
        let mut color = Color::new(128, 64, 192, 255);
        color.name = "purple".to_string();

        let json_string = color.to_json_data().unwrap();
        let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(data["type"], "Color");
        assert_eq!(data["name"], "purple");
        assert_eq!(data["r"], 128);
        assert_eq!(data["g"], 64);
        assert_eq!(data["b"], 192);
        assert_eq!(data["a"], 255);
        assert!(data["guid"].is_string());
    }

    #[test]
    fn test_color_from_json_data() {
        let mut original_color = Color::new(200, 150, 100, 255);
        original_color.name = "bronze".to_string();

        let json_string = original_color.to_json_data().unwrap();
        let restored_color = Color::from_json_data(&json_string).unwrap();

        assert_eq!(restored_color.r, 200);
        assert_eq!(restored_color.g, 150);
        assert_eq!(restored_color.b, 100);
        assert_eq!(restored_color.a, 255);
        assert_eq!(restored_color.name, "bronze");
        assert_eq!(restored_color.guid, original_color.guid);
    }

    #[test]
    fn test_color_to_json_from_json() {
        let mut original = Color::new(255, 128, 64, 255);
        original.name = "sunset_orange".to_string();
        let filename = "test_color.json";

        original.to_json(filename).unwrap();
        let loaded = Color::from_json(filename).unwrap();

        assert_eq!(loaded.r, original.r);
        assert_eq!(loaded.g, original.g);
        assert_eq!(loaded.b, original.b);
        assert_eq!(loaded.a, original.a);
        assert_eq!(loaded.name, original.name);
        assert_eq!(loaded.guid, original.guid);
    }

    #[test]
    fn test_color_white() {
        let white = Color::white();
        assert_eq!(white.name, "white");
        assert_eq!(white.r, 255);
        assert_eq!(white.g, 255);
        assert_eq!(white.b, 255);
        assert_eq!(white.a, 255);
    }

    #[test]
    fn test_color_black() {
        let black = Color::black();
        assert_eq!(black.name, "black");
        assert_eq!(black.r, 0);
        assert_eq!(black.g, 0);
        assert_eq!(black.b, 0);
        assert_eq!(black.a, 255);
    }

    #[test]
    fn test_color_to_float_array() {
        let color = Color::new(255, 128, 64, 255);
        let float_array = color.to_float_array();
        assert_eq!(float_array, [1.0, 0.5019608, 0.2509804, 1.0]);
    }

    #[test]
    fn test_color_from_float() {
        let color = Color::from_float(1.0, 0.5, 0.25, 1.0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128); // 0.5 * 255 = 127.5, rounded to 128 in Rust
        assert_eq!(color.b, 64); // 0.25 * 255 = 63.75, rounded to 64
        assert_eq!(color.a, 255);
    }
}
