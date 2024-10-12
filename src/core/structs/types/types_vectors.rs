// -----------------------------  Vec2Int -----------------------------  //
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

#[allow(unused)]
impl Vec2Int {
    /// Number of bytes for `Vec2Int` (4 bytes per i32, total 8 bytes).
    pub const BYTE_COUNT: usize = 8;

    /// Converts `Vec2Int` to a string in the format "(x, y)".
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    /// Converts `Vec2Int` to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..].copy_from_slice(&self.y.to_be_bytes());
        bytes
    }

    /// Returns the byte count for `Vec2Int` (always 8 bytes).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Vec2Int::BYTE_COUNT]> for Vec2Int {
    /// Creates a `Vec2Int` from a byte array.
    fn from(bytes: [u8; Vec2Int::BYTE_COUNT]) -> Self {
        let x = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let y = i32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vec2Int {
    /// Creates a `Vec2Int` from a tuple of x and y.
    fn from(values: (i32, i32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
        }
    }
}


// -----------------------------  Vec3Int -----------------------------  //
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec3Int {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[allow(unused)]
impl Vec3Int {
    /// Number of bytes for `Vec3Int` (4 bytes per i32, total 12 bytes).
    pub const BYTE_COUNT: usize = 12;

    /// Converts `Vec3Int` to a string in the format "(x, y, z)".
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    /// Converts `Vec3Int` to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..].copy_from_slice(&self.z.to_be_bytes());
        bytes
    }

    /// Returns the byte count for `Vec3Int` (always 12 bytes).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Vec3Int::BYTE_COUNT]> for Vec3Int {
    /// Creates a `Vec3Int` from a byte array.
    fn from(bytes: [u8; Vec3Int::BYTE_COUNT]) -> Self {
        let x = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let y = i32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let z = i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        Self { x, y, z }
    }
}

impl From<(i32, i32, i32)> for Vec3Int {
    /// Creates a `Vec3Int` from a tuple of x, y, and z.
    fn from(values: (i32, i32, i32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
            z: values.2,
        }
    }
}


// -----------------------------  Vec2Float -----------------------------  //
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2Float {
    pub x: f32,
    pub y: f32,
}

#[allow(unused)]
impl Vec2Float {
    /// Number of bytes for `Vec2Float` (4 bytes per f32, total 8 bytes).
    pub const BYTE_COUNT: usize = 8;

    /// Converts `Vec2Float` to a string in the format "(x, y)".
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    /// Converts `Vec2Float` to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..].copy_from_slice(&self.y.to_be_bytes());
        bytes
    }

    /// Returns the byte count for `Vec2Float` (always 8 bytes).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Vec2Float::BYTE_COUNT]> for Vec2Float {
    /// Creates a `Vec2Float` from a byte array.
    fn from(bytes: [u8; Vec2Float::BYTE_COUNT]) -> Self {
        let x = f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let y = f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Self { x, y }
    }
}

impl From<(f32, f32)> for Vec2Float {
    /// Creates a `Vec2Float` from a tuple of x and y.
    fn from(values: (f32, f32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
        }
    }
}


// -----------------------------  Vec3Float -----------------------------  //
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3Float {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(unused)]
impl Vec3Float {
    /// Number of bytes for `Vec3Float` (4 bytes per f32, total 12 bytes).
    pub const BYTE_COUNT: usize = 12;

    /// Converts `Vec3Float` to a string in the format "(x, y, z)".
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    /// Converts `Vec3Float` to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..4].copy_from_slice(&self.x.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_be_bytes());
        bytes[8..].copy_from_slice(&self.z.to_be_bytes());
        bytes
    }

    /// Returns the byte count for `Vec3Float` (always 12 bytes).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Vec3Float::BYTE_COUNT]> for Vec3Float {
    /// Creates a `Vec3Float` from a byte array.
    fn from(bytes: [u8; Vec3Float::BYTE_COUNT]) -> Self {
        let x = f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let y = f32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let z = f32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for Vec3Float {
    /// Creates a `Vec3Float` from a tuple of x, y, and z.
    fn from(values: (f32, f32, f32)) -> Self {
        Self {
            x: values.0,
            y: values.1,
            z: values.2,
        }
    }
}
