/// RGB value must be between 0 and 255
/// Make sure the value is between 0 and 255
pub fn sanitize_rgb(rgb: u8) -> u8 {
    rgb.clamp(0, 255)
}

/// Alpha value must be between 0.0 and 1.0
/// Make sure the value is between 0.0 and 1.0
pub fn sanitize_alpha(alpha: f32) -> f32 {
    alpha.clamp(0.0, 1.0)
}
