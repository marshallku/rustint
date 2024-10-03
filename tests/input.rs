#[cfg(test)]
mod tests {
    use rustint::Color;

    #[test]
    fn test_from_new() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 1.0);
    }

    #[test]
    fn test_from_hex_valid() {
        let color = Color::try_from("#FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 1.0);
    }

    #[test]
    fn test_from_hex_valid_short() {
        let color = Color::try_from("#F80").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 136);
        assert_eq!(color.blue, 0);

        let color = Color::try_from("#f80").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 136);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_from_hex_with_alpha() {
        let color = Color::try_from("#FF8040FF").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 1.0);

        let color = Color::try_from("#FF8040CC").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
        assert_eq!(color.alpha, 0.8);
    }

    #[test]
    fn test_try_from_hex_invalid() {
        assert!(Color::try_from("FF8040").is_err());
        assert!(Color::try_from("#FF804").is_err());
        assert!(Color::try_from("#FF80ZZ").is_err());
        assert!(Color::try_from("FF8040CC").is_err());
        assert!(Color::try_from("#FF8040C").is_err());
        assert!(Color::try_from("FF8040ZZ").is_err());
        assert!(Color::try_from("#F8Z").is_err());
        assert!(Color::try_from("F8Z").is_err());
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 128, 64);
        assert_eq!(format!("{}", color), "#FF8040");

        let color = Color::with_alpha(255, 128, 64, 0.8);
        assert_eq!(format!("{}", color), "#FF8040CC");
    }
}
