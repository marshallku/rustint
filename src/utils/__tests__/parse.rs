#[cfg(test)]
mod tests {
    use crate::{
        utils::parse::{parse_hex, parse_rgb_from_hex},
        ColorError,
    };

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(parse_hex("00"), Ok(0));
        assert_eq!(parse_hex("FF"), Ok(255));
        assert_eq!(parse_hex("80"), Ok(128));
        assert_eq!(parse_hex("0A"), Ok(10));
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert!(matches!(parse_hex(""), Err(ColorError::InvalidColorFormat)));
        assert!(matches!(
            parse_hex("G0"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_hex("FFF"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_hex("-1"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_hex("100"),
            Err(ColorError::InvalidColorFormat)
        ));
    }

    #[test]
    fn test_parse_rgb_from_hex_valid() {
        assert_eq!(parse_rgb_from_hex("000000"), Ok((0, 0, 0)));
        assert_eq!(parse_rgb_from_hex("FFFFFF"), Ok((255, 255, 255)));
        assert_eq!(parse_rgb_from_hex("FF8000"), Ok((255, 128, 0)));
        assert_eq!(parse_rgb_from_hex("0A1B2C"), Ok((10, 27, 44)));
    }

    #[test]
    fn test_parse_rgb_from_hex_invalid() {
        assert!(matches!(
            parse_rgb_from_hex(""),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_rgb_from_hex("12345"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_rgb_from_hex("1234567"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_rgb_from_hex("GGGGGG"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_rgb_from_hex("00FFFF0"),
            Err(ColorError::InvalidColorFormat)
        ));
        assert!(matches!(
            parse_rgb_from_hex("00FFFF00F"),
            Err(ColorError::InvalidColorFormat)
        ));
    }

    #[test]
    fn test_parse_rgb_from_hex_mixed_case() {
        assert_eq!(parse_rgb_from_hex("aAbBcC"), Ok((170, 187, 204)));
    }

    #[test]
    fn test_parse_rgb_from_hex_boundary() {
        assert_eq!(parse_rgb_from_hex("000001"), Ok((0, 0, 1)));
        assert_eq!(parse_rgb_from_hex("FFFFFE"), Ok((255, 255, 254)));
    }
}
