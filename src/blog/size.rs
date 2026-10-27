//! The size of the blog's pictures, read from their WebP headers, so the
//! pages can give each `<img>` its width and height: the browser then keeps
//! its room before it loads, and the words under it do not jump.

use std::path::Path;

/// Returns the width and height of the picture served at `path`, a path
/// from the root, as the checkout holds it; or nothing, if it is not a WebP.
pub fn of(path: &str) -> Option<(u32, u32)> {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join(path.trim_start_matches('/'));
    webp(&std::fs::read(file).ok()?)
}

/// Returns a WebP's width and height, read from its header, whichever of
/// the three kinds it is: lossy (`VP8 `), lossless (`VP8L`) or extended
/// (`VP8X`).
pub fn webp(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.get(..4)? != b"RIFF" || bytes.get(8..12)? != b"WEBP" {
        return None;
    }
    let byte = |at: usize| bytes.get(at).copied().map(u32::from);
    match bytes.get(12..16)? {
        // The frame's tag, its start code, then each side in 14 bits.
        b"VP8 " => {
            if bytes.get(23..26)? != [0x9d, 0x01, 0x2a] {
                return None;
            }
            let side = |at: usize| Some((byte(at)? | byte(at + 1)? << 8) & 0x3fff);
            Some((side(26)?, side(28)?))
        }
        // A signature, then each side less one in 14 bits.
        b"VP8L" => {
            if byte(20)? != 0x2f {
                return None;
            }
            let bits = byte(21)? | byte(22)? << 8 | byte(23)? << 16 | byte(24)? << 24;
            Some(((bits & 0x3fff) + 1, (bits >> 14 & 0x3fff) + 1))
        }
        // The canvas: each side less one in three bytes.
        b"VP8X" => {
            let side = |at: usize| Some(1 + (byte(at)? | byte(at + 1)? << 8 | byte(at + 2)? << 16));
            Some((side(24)?, side(27)?))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a WebP's first bytes: its RIFF header, then `chunk` holding `data`.
    fn header(chunk: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut bytes = b"RIFF\0\0\0\0WEBP".to_vec();
        bytes.extend_from_slice(chunk);
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(data);
        bytes
    }

    #[test]
    fn every_kind_of_webp_tells_its_size() {
        let lossy = header(
            b"VP8 ",
            &[0, 0, 0, 0x9d, 0x01, 0x2a, 0xd0, 0x07, 0x20, 0x03],
        );
        assert_eq!(webp(&lossy), Some((2000, 800)));
        // 2000 by 800, each less one: 1999 in bits 0 to 13, 799 from bit 14.
        let bits: u32 = 1999 | 799 << 14;
        let mut data = vec![0x2f];
        data.extend_from_slice(&bits.to_le_bytes());
        assert_eq!(webp(&header(b"VP8L", &data)), Some((2000, 800)));
        let extended = header(b"VP8X", &[0, 0, 0, 0, 0xcf, 0x07, 0, 0x1f, 0x03, 0]);
        assert_eq!(webp(&extended), Some((2000, 800)));
    }

    #[test]
    fn what_is_not_a_webp_has_no_size() {
        assert_eq!(webp(b""), None);
        assert_eq!(webp(b"\x89PNG\r\n\x1a\n"), None);
        assert_eq!(
            webp(&header(b"VP8 ", &[0, 0, 0, 1, 2, 3, 0, 0, 0, 0])),
            None
        );
        assert_eq!(webp(&header(b"VP8L", &[0x2e, 0, 0, 0, 0])), None);
        assert_eq!(webp(&header(b"ALPH", &[0; 10])), None);
        assert_eq!(webp(&header(b"VP8X", &[0; 4])), None);
    }
}
