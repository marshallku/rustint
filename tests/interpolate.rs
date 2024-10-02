#[cfg(test)]
mod tests {
    use rustint::Color;
    #[test]
    fn test_interpolate() {
        let color1 = Color::new(0, 0, 0);
        let color2 = Color::new(255, 255, 255);

        let interpolated = color1.interpolate(&color2, 50.0);
        assert_eq!(interpolated.red, 128);
        assert_eq!(interpolated.green, 128);
        assert_eq!(interpolated.blue, 128);

        let interpolated = color1.interpolate(&color2, 25.0);
        assert_eq!(interpolated.red, 64);
        assert_eq!(interpolated.green, 64);
        assert_eq!(interpolated.blue, 64);

        let interpolated = color1.interpolate(&color2, 75.0);
        assert_eq!(interpolated.red, 191);
        assert_eq!(interpolated.green, 191);
        assert_eq!(interpolated.blue, 191);
    }
}
