use std::fmt::{self, Display, Formatter};

/// Represents an RGB color.
#[derive(Debug, Clone)]
pub struct Color {
    /// Red component (0-255)
    pub red: u8,
    /// Green component (0-255)
    pub green: u8,
    /// Blue component (0-255)
    pub blue: u8,
}

/// Error type for color parsing
#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    /// Error when the hex color format is invalid.
    #[error("Invalid hex color format. Use #RRGGBB.")]
    InvalidFormat,
    /// Error when the hex value cannot be parsed.
    #[error("Invalid hex value.")]
    InvalidHexValue,
}

impl Color {
    /// Creates a new `Color` instance.
    ///
    /// # Arguments
    ///
    /// * `red` - The red component (0-255)
    /// * `green` - The green component (0-255)
    /// * `blue` - The blue component (0-255)
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    /// Interpolates between two RGB colors based on percentage.
    ///
    /// # Arguments
    ///
    /// * `color` - The target color to interpolate towards
    /// * `percentage` - The interpolation percentage (0.0 to 100.0)
    ///
    /// # Returns
    ///
    /// A new `Color` instance representing the interpolated color.
    pub fn interpolate(&self, color: &Color, percentage: f32) -> Self {
        let ratio = percentage / 100.0;
        let r = (self.red as f32 + (color.red as f32 - self.red as f32) * ratio).round() as u8;
        let g =
            (self.green as f32 + (color.green as f32 - self.green as f32) * ratio).round() as u8;
        let b = (self.blue as f32 + (color.blue as f32 - self.blue as f32) * ratio).round() as u8;

        Color::new(r, g, b)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;

    /// Tries to create a `Color` from a hexadecimal color string.
    ///
    /// # Arguments
    ///
    /// * `hex` - A string slice that holds the hexadecimal color code (e.g., "#FF00FF")
    ///
    /// # Returns
    ///
    /// A `Result` containing either the created `Color` or a `ColorError`.
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
    /// Formats the `Color` as a hexadecimal string.
    ///
    /// # Returns
    ///
    /// A `String` representing the color in "#RRGGBB" format.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
