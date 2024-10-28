use core::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SmlColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl SmlColor {
    /// Number of bytes for SmlColor (1 byte per component, 4 bytes total).
    pub const BYTE_COUNT: usize = 4;

    /// Converts `SmlColor` to a byte array (RGBA format).
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        [self.r, self.g, self.b, self.a]
    }

    /// Creates a `SmlColor` from a byte array.
    pub fn read_from_byte_buffer(bytes: [u8; Self::BYTE_COUNT]) -> Self {
        Self {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }

    /// Converts `SmlColor` to a formatted string in the format `RGBA(r, g, b, a)`.
    pub fn to_string(&self) -> String {
        format!("RGBA({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl From<(u8, u8, u8, u8)> for SmlColor {
    /// Creates a `SmlColor` from a tuple of (r, g, b, a).
    fn from(values: (u8, u8, u8, u8)) -> Self {
        Self {
            r: values.0,
            g: values.1,
            b: values.2,
            a: values.3,
        }
    }
}

impl Display for SmlColor {
    /// Formats `SmlColor` as a string in the format `RGBA(r, g, b, a)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LrgColor {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl LrgColor {
    /// Number of bytes for LrgColor (2 bytes per component, 8 bytes total).
    pub const BYTE_COUNT: usize = 8;

    /// Converts `LrgColor` to a byte array (RGBA format, using big-endian).
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..2].copy_from_slice(&self.r.to_be_bytes());
        bytes[2..4].copy_from_slice(&self.g.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.b.to_be_bytes());
        bytes[6..].copy_from_slice(&self.a.to_be_bytes());
        bytes
    }

    /// Creates a `LrgColor` from a byte array.
    pub fn from_bytes(bytes: [u8; Self::BYTE_COUNT]) -> Self {
        Self {
            r: u16::from_be_bytes([bytes[0], bytes[1]]),
            g: u16::from_be_bytes([bytes[2], bytes[3]]),
            b: u16::from_be_bytes([bytes[4], bytes[5]]),
            a: u16::from_be_bytes([bytes[6], bytes[7]]),
        }
    }

    /// Converts `LrgColor` to a formatted string in the format `RGBA(r, g, b, a)`.
    pub fn to_string(&self) -> String {
        format!("RGBA({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl From<(u16, u16, u16, u16)> for LrgColor {
    /// Creates a `LrgColor` from a tuple of (r, g, b, a).
    fn from(values: (u16, u16, u16, u16)) -> Self {
        Self {
            r: values.0,
            g: values.1,
            b: values.2,
            a: values.3,
        }
    }
}
