use std::fmt;

/// The `Color` type. Outputs as a #aarrggbb hex string.
#[derive(PartialEq)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    /// Create a color with RGB and alpha channels specified.
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color(red, green, blue, alpha)
    }

    /// Create a 100% opaque color with RGB channels only.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color(red, green, blue, 255)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Color(red, green, blue, alpha) = *self;

        write!(f, "#{:x}{:x}{:x}{:x}", alpha, red, green, blue)
    }
}

mod tests {
    use super::*;

    #[test]
    fn color_rgba_to_string() {
        let color = Color::rgba(24, 24, 24, 0x88);

        assert_eq!(color.to_string(), "#88181818");
    }

    #[test]
    fn color_rgb_to_string() {
        let color = Color::rgb(24, 24, 24);

        assert_eq!(color.to_string(), "#ff181818");
    }
}
