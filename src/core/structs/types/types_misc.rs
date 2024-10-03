// -----------------------------  Version -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    major: u8,
    minor: u8,
}

#[allow(unused)]
impl Version {
    /// Number of bytes for `Version` (2 bytes: 1 for major, 1 for minor).
    pub const BYTE_COUNT: usize = 2;

    /// Converts `Version` to a string in the format "major.minor".
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }

    /// Converts `Version` to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        [self.major, self.minor]
    }

    /// Converts `Version` to an `f32` float.
    pub fn to_f32(&self) -> f32 {
        format!("{}.{}", self.major, self.minor)
            .parse::<f32>()
            .expect("Failed to parse version as float")
    }

    /// Returns the byte count for `Version` (always 2).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Version::BYTE_COUNT]> for Version {
    /// Creates a `Version` from a byte array.
    fn from(bytes: [u8; Version::BYTE_COUNT]) -> Self {
        Self {
            major: bytes[0],
            minor: bytes[1],
        }
    }
}

impl From<f32> for Version {
    /// Creates a `Version` from an `f32` float, splitting into major and minor.
    fn from(version: f32) -> Self {
        let major = version.trunc() as u8;
        let minor = ((version - version.trunc()) * 100.0).round() as u8;
        Self { major, minor }
    }
}

impl From<(u8, u8)> for Version {
    /// Creates a `Version` from a tuple of major and minor.
    fn from(major_minor: (u8, u8)) -> Self {
        Self {
            major: major_minor.0,
            minor: major_minor.1,
        }
    }
}


// -----------------------------  LangCode -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LangCode {
    EN = 1,
    FR = 2,
    ES = 3,
    DE = 4,
}

impl LangCode {
    /// Number of bytes for `LangCode` (1 byte).
    pub const BYTE_COUNT: usize = 1;

    /// Converts `LangCode` to its string representation.
    pub fn to_string(&self) -> String {
        match self {
            LangCode::EN => "EN".to_string(),
            LangCode::FR => "FR".to_string(),
            LangCode::ES => "ES".to_string(),
            LangCode::DE => "DE".to_string(),
        }
    }

    /// Converts `LangCode` to its integer representation.
    pub fn to_int(&self) -> u8 {
        *self as u8
    }

    /// Converts `LangCode` to a byte.
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }
}

impl From<&str> for LangCode {
    /// Creates a `LangCode` from a 2-letter language code string.
    fn from(code: &str) -> Self {
        match code {
            "EN" => LangCode::EN,
            "FR" => LangCode::FR,
            "ES" => LangCode::ES,
            "DE" => LangCode::DE,
            _ => panic!("Invalid language code"),
        }
    }
}

impl From<u8> for LangCode {
    /// Creates a `LangCode` from a byte.
    fn from(byte: u8) -> Self {
        match byte {
            1 => LangCode::EN,
            2 => LangCode::FR,
            3 => LangCode::ES,
            4 => LangCode::DE,
            _ => panic!("Invalid language code"),
        }
    }
}

impl From<LangCode> for u8 {
    /// Converts `LangCode` to its byte representation.
    fn from(code: LangCode) -> u8 {
        code as u8
    }
}


