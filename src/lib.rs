use std::fmt::{self, Display, Formatter};

use utils::parse::{parse_and_validate_rgba, parse_hex, parse_rgb_from_hex};

mod utils;

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
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ColorError {
    /// Error when the hex color format is invalid.
    #[error("Invalid hex color format. Use #RRGGBB.")]
    InvalidFormat,
    /// Error when the color format is invalid.
    #[error("Invalid color format.")]
    InvalidColorFormat,
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

    /// Tries to create a `Color` from a hexadecimal color string.
    ///
    /// # Arguments
    ///
    /// * `hex` - A string slice that holds the hexadecimal color code (e.g., "#FF00FF")
    ///
    /// # Returns
    ///
    /// A `Result` containing either the created `Color` or a `ColorError`.
    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        if !hex.starts_with('#') {
            return Err(ColorError::InvalidFormat);
        }

        let hex = &hex[1..]; // Remove '#' prefix

        let (red, green, blue, alpha) = match hex.len() {
            3 => {
                let parse = |idx| parse_hex(&hex[idx..idx + 1]).map(|v| v * 17);
                (parse(0)?, parse(1)?, parse(2)?, 1.0)
            }
            6 => {
                let (r, g, b) = parse_rgb_from_hex(hex)?;
                (r, g, b, 1.0)
            }
            8 => {
                let (r, g, b) = parse_rgb_from_hex(hex)?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| ColorError::InvalidColorFormat)?;
                (r, g, b, a as f32 / 255.0)
            }
            _ => return Err(ColorError::InvalidFormat),
        };

        Ok(Color {
            red,
            green,
            blue,
            alpha,
        })
    }

    /// Tries to create a `Color` from a rgba color string.
    /// The format is "rgba(r, g, b, a)" where r, g, b are the red, green, blue components (0-255)
    /// Or "rgb(r, g, b)" where r, g, b are the red, green, blue components (0-255)
    /// and a is the alpha component (0.0-1.0).
    /// The spaces are optional.
    /// The alpha component is optional and defaults to 1.0.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the rgba color code (e.g., "rgba(255, 128, 64, 0.5)", "rgb(255, 128, 64)")
    ///
    /// # Returns
    ///
    /// A `Result` containing either the created `Color` or a `ColorError`.
    pub fn from_rgba(input: &str) -> Result<Self, ColorError> {
        let input = input.trim();

        if (!input.starts_with("rgba(") && !input.starts_with("rgb(")) || !input.ends_with(')') {
            return Err(ColorError::InvalidFormat);
        }

        let input = match input.find('(') {
            Some(idx) => &input[idx + 1..input.len() - 1],
            None => return Err(ColorError::InvalidFormat),
        };
        let colors: Vec<&str> = input.split(',').collect();

        if colors.len() < 3 || 4 < colors.len() {
            return Err(ColorError::InvalidFormat);
        }

        let red = parse_and_validate_rgba(colors[0], 0.0, 255.0)?.round() as u8;
        let green = parse_and_validate_rgba(colors[1], 0.0, 255.0)?.round() as u8;
        let blue = parse_and_validate_rgba(colors[2], 0.0, 255.0)?.round() as u8;
        let alpha = colors
            .get(3)
            .map_or(Ok(1.0), |a| parse_and_validate_rgba(a, 0.0, 1.0))
            .map_err(|_| ColorError::InvalidColorFormat)?;

        Ok(Color::with_alpha(red, green, blue, alpha))
    }

    /// Converts the `Color` to a hexadecimal color string.
    /// The format is "#RRGGBB" or "#RRGGBBAA" where RR, GG, BB are the red, green, blue components (00-FF)
    /// and AA is the alpha component (00-FF).
    ///
    /// # Returns
    ///
    /// A `String` representing the color in "#RRGGBB" or "#RRGGBBAA" format.
    pub fn to_hex(&self) -> String {
        if (self.alpha - 1.0).abs() < f32::EPSILON {
            format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        } else {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                self.red,
                self.green,
                self.blue,
                (self.alpha * 255.0) as u8
            )
        }
    }

    /// Converts the `Color` to a rgba color string.
    /// The format is "rgba(r, g, b, a)" where r, g, b are the red, green, blue components (0-255)
    /// and a is the alpha component (0.0-1.0).
    ///
    /// # Returns
    ///
    /// A `String` representing the color in "rgba(r, g, b, a)" format.
    pub fn to_rgba(&self) -> String {
        if (self.alpha - 1.0).abs() < f32::EPSILON {
            format!("rgb({}, {}, {})", self.red, self.green, self.blue)
        } else {
            format!(
                "rgba({}, {}, {}, {})",
                self.red, self.green, self.blue, self.alpha
            )
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;

    /// Tries to create a `Color` from various string formats.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that holds the color code (e.g., "#FF8040", "#FF8040FF", "rgba(255, 128, 64, 0.5)")
    ///
    /// # Returns
    ///
    /// A `Result` containing either the created `Color` or a `ColorError`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('#') {
            Color::from_hex(value)
        } else {
            Color::from_rgba(value)
        }
    }
}

impl Display for Color {
    /// Formats the `Color` as a hexadecimal string.
    ///
    /// # Returns
    ///
    /// A `String` representing the color in "#RRGGBB" format.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
