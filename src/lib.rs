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
    /// Alpha component (0-1)
    pub alpha: f32,
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
        Color {
            red,
            green,
            blue,
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` instance with an alpha component.
    /// The alpha component is a floating-point value between 0.0 and 1.0.
    /// A value of 0.0 represents full transparency, while 1.0 is fully opaque.
    /// Values between 0.0 and 1.0 represent varying levels of transparency.
    ///
    /// # Arguments
    ///
    /// * `red` - The red component (0-255)
    /// * `green` - The green component (0-255)
    /// * `blue` - The blue component (0-255)
    /// * `alpha` - The alpha component (0.0-1.0)
    ///
    /// # Returns
    ///
    /// A new `Color` instance.
    pub fn with_alpha(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha: alpha.clamp(0.0, 1.0),
        }
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
        let red = (self.red as f32 + (color.red as f32 - self.red as f32) * ratio).round() as u8;
        let green =
            (self.green as f32 + (color.green as f32 - self.green as f32) * ratio).round() as u8;
        let blue =
            (self.blue as f32 + (color.blue as f32 - self.blue as f32) * ratio).round() as u8;
        let alpha = self.alpha + (color.alpha - self.alpha) * ratio;

        Color::with_alpha(red, green, blue, alpha)
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
        if (hex.len() != 7 && hex.len() != 9 && hex.len() != 4) || !hex.starts_with('#') {
            return Err(ColorError::InvalidFormat);
        }

        if hex.len() == 4 {
            let red = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| ColorError::InvalidHexValue)?;
            let green = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| ColorError::InvalidHexValue)?;
            let blue = u8::from_str_radix(&hex[3..4].repeat(2), 16)
                .map_err(|_| ColorError::InvalidHexValue)?;

            return Ok(Color {
                red,
                green,
                blue,
                alpha: 1.0,
            });
        }

        let red = u8::from_str_radix(&hex[1..3], 16).map_err(|_| ColorError::InvalidHexValue)?;
        let green = u8::from_str_radix(&hex[3..5], 16).map_err(|_| ColorError::InvalidHexValue)?;
        let blue = u8::from_str_radix(&hex[5..7], 16).map_err(|_| ColorError::InvalidHexValue)?;
        let alpha = if hex.len() == 9 {
            u8::from_str_radix(&hex[7..9], 16).map_err(|_| ColorError::InvalidHexValue)? as f32
                / 255.0
        } else {
            1.0
        };

        Ok(Color {
            red,
            green,
            blue,
            alpha,
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
        if (self.alpha - 1.0).abs() < f32::EPSILON {
            write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        } else {
            write!(
                f,
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red,
                self.green,
                self.blue,
                (self.alpha * 255.0) as u8
            )
        }
    }
}
