#[cfg(test)]
mod tests {
    use rustint::Color;

    #[test]
    fn test_display() {
        let color = Color::new(255, 128, 64);
        assert_eq!(format!("{}", color), "#FF8040");

        let color = Color::with_alpha(255, 128, 64, 0.8);
        assert_eq!(format!("{}", color), "#FF8040CC");
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(Color::with_alpha(0, 0, 0, 1.0).to_hex(), "#000000");
        assert_eq!(Color::with_alpha(255, 255, 255, 1.0).to_hex(), "#FFFFFF");
        assert_eq!(Color::with_alpha(255, 128, 64, 1.0).to_hex(), "#FF8040");
        assert_eq!(Color::with_alpha(10, 27, 44, 1.0).to_hex(), "#0A1B2C");
        assert_eq!(Color::with_alpha(255, 128, 64, 0.8).to_hex(), "#FF8040CC");
        assert_eq!(Color::with_alpha(255, 128, 64, 0.0).to_hex(), "#FF804000");
    }

    #[test]
    fn test_to_rgb() {
        assert_eq!(Color::with_alpha(0, 0, 0, 1.0).to_rgba(), "rgb(0, 0, 0)");
        assert_eq!(
            Color::with_alpha(255, 255, 255, 1.0).to_rgba(),
            "rgb(255, 255, 255)"
        );
        assert_eq!(
            Color::with_alpha(255, 255, 255, 0.99).to_rgba(),
            "rgba(255, 255, 255, 0.99)"
        );
        assert_eq!(
            Color::with_alpha(255, 255, 255, 0.0).to_rgba(),
            "rgba(255, 255, 255, 0)"
        );
    }
}
