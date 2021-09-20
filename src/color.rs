//! Functions for working with color in Harmony API.

/// Encode an RGB value.
pub fn encode_rgb(color: impl Into<[u8; 3]>) -> i32 {
    let color = color.into();
    let mut c = color[0] as i32;
    c = (c << 8) + color[1] as i32;
    c = (c << 8) + color[2] as i32;
    c as i32
}

/// Decode an RGB value.
pub fn decode_rgb(color: impl Into<i32>) -> [u8; 3] {
    let color = color.into();
    [
        ((color >> 16) & 255) as u8,
        ((color >> 8) & 255) as u8,
        (color & 255) as u8,
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode() {
        assert_eq!(encode_rgb([0, 0, 0]), 0);
    }

    #[test]
    fn decode() {
        assert_eq!(decode_rgb(0), [0, 0, 0]);
    }
}