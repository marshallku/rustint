use crate::ColorError;

pub fn parse_hex(value: &str) -> Result<u8, ColorError> {
    u8::from_str_radix(value, 16).map_err(|_| ColorError::InvalidColorFormat)
}

pub fn parse_rgb_from_hex(hex: &str) -> Result<(u8, u8, u8), ColorError> {
    let parse = |start, end| parse_hex(&hex[start..end]);
    Ok((parse(0, 2)?, parse(2, 4)?, parse(4, 6)?))
}

pub fn parse_and_validate_rgba(input: &str, min: f32, max: f32) -> Result<f32, ColorError> {
    let value: f32 = input
        .trim()
        .parse()
        .map_err(|_| ColorError::InvalidColorFormat)?;
    if value < min || max < value {
        Err(ColorError::InvalidColorFormat)
    } else {
        Ok(value)
    }
}
