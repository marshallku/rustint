use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("Invalid hex color format. Use #RRGGBB.")]
    InvalidFormat,
    #[error("Invalid hex value.")]
    InvalidHexValue,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(ColorError::InvalidFormat);
        }

        let parse_component = |s: &str| -> Result<u8, ColorError> {
            u8::from_str_radix(s, 16).map_err(|_| ColorError::InvalidHexValue)
        };

        Ok(Self {
            red: parse_component(&hex[1..3])?,
            green: parse_component(&hex[3..5])?,
            blue: parse_component(&hex[5..7])?,
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
