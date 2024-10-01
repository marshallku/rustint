#[cfg(test)]
mod tests {
    use rustint::Color;

    #[test]
    fn test_from_new() {
        let color = Color::new(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_from_hex_valid() {
        let color = Color::try_from("#FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_try_from_hex_invalid() {
        assert!(Color::try_from("FF8040").is_err());
        assert!(Color::try_from("#FF804").is_err());
        assert!(Color::try_from("#FF80ZZ").is_err());
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 128, 64);
        assert_eq!(format!("{}", color), "#FF8040");
    }
}
