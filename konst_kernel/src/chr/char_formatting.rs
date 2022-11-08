pub const fn encode_utf8(char: char) -> Utf8Encoded {
    let u32 = char as u32;
    match u32 {
        0..=127 => Utf8Encoded {
            encoded: [u32 as u8, 0, 0, 0],
            len: 1,
        },
        0x80..=0x7FF => {
            let b0 = 0b1100_0000 | (u32 >> 6) as u8;
            let b1 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            Utf8Encoded {
                encoded: [b0, b1, 0, 0],
                len: 2,
            }
        }
        0x800..=0xFFFF => {
            let b0 = 0b1110_0000 | (u32 >> 12) as u8;
            let b1 = 0b1000_0000 | ((u32 >> 6) & 0b0011_1111) as u8;
            let b2 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            Utf8Encoded {
                encoded: [b0, b1, b2, 0],
                len: 3,
            }
        }
        0x10000..=u32::MAX => {
            let b0 = 0b1111_0000 | (u32 >> 18) as u8;
            let b1 = 0b1000_0000 | ((u32 >> 12) & 0b0011_1111) as u8;
            let b2 = 0b1000_0000 | ((u32 >> 6) & 0b0011_1111) as u8;
            let b3 = 0b1000_0000 | (u32 & 0b0011_1111) as u8;
            Utf8Encoded {
                encoded: [b0, b1, b2, b3],
                len: 4,
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Utf8Encoded {
    encoded: [u8; 4],
    len: u8,
}

impl Utf8Encoded {
    /// Gets the utf8-encoded char as a `&str`
    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Gets the utf8-encoded char as a `&[u8]`
    pub const fn as_bytes(&self) -> &[u8] {
        crate::slice::slice_up_to(&self.encoded, self.len as usize)
    }
}

#[cfg(all(tests, not(miri)))]
mod tests {
    use super::{encode_utf8, Utf8Encoded};

    fn as_bytes(fmt: &Utf8Encoded) -> &[u8] {
        &fmt.encoded()[..fmt.len()]
    }

    #[test]
    fn char_to_utf8_encoding_test() {
        for c in '\0'..=core::char::MAX {
            let mut utf8_std = [0u8; 4];
            let utf8_std = c.encode_utf8(&mut utf8_std);

            let utf8_konst = encode_utf8(c);
            assert_eq!(utf8_std.as_bytes(), as_bytes(&utf8_konst));
            assert_eq!(utf8_std.as_bytes(), utf8_konst.as_bytes());

            {
                core::str::from_utf8(utf8_std.as_bytes()).unwrap();
                assert_eq!(utf8_std, utf8_konst.as_str());
            }
        }
    }
}
