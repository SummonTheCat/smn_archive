// - Version - //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    pub const BYTE_COUNT: usize = 2;

    pub fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }

    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        [self.major, self.minor]
    }

    pub fn to_f32(&self) -> f32 {
        format!("{}.{}", self.major, self.minor)
            .parse::<f32>()
            .expect("Failed to parse version as float")
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<[u8; Version::BYTE_COUNT]> for Version {
    fn from(bytes: [u8; Version::BYTE_COUNT]) -> Self {
        Self {
            major: bytes[0],
            minor: bytes[1],
        }
    }
}

impl From<f32> for Version {
    fn from(version: f32) -> Self {
        let major = version.trunc() as u8;
        let minor = ((version - version.trunc()) * 100.0).round() as u8;
        Self { major, minor }
    }
}

impl From<(u8, u8)> for Version {
    fn from(major_minor: (u8, u8)) -> Self {
        Self {
            major: major_minor.0,
            minor: major_minor.1,
        }
    }
}


// - LangCode - //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LangCode {
    EN = 1,
    FR = 2,
    ES = 3,
    DE = 4,
}

impl LangCode {
    pub const BYTE_COUNT: usize = 1;

    pub fn to_string(&self) -> String {
        match self {
            LangCode::EN => "EN".to_string(),
            LangCode::FR => "FR".to_string(),
            LangCode::ES => "ES".to_string(),
            LangCode::DE => "DE".to_string(),
        }
    }

    pub fn to_int(&self) -> u8 {
        *self as u8
    }

    pub fn to_byte(&self) -> u8 {
        *self as u8
    }
}

impl From<&str> for LangCode {
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
    fn from(code: LangCode) -> u8 {
        code as u8
    }
}

// - FormType - //
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FormType {
    STRING = 0,
    WORLD = 1,
}

impl FormType {
    pub const BYTE_COUNT: usize = 1;

    pub fn to_string(&self) -> String {
        match self {
            FormType::STRING => "STRING".to_string(),
            FormType::WORLD => "WORLD".to_string(),
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    pub fn to_byte(&self) -> u8 {
        *self as u8
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u8> for FormType {
    fn from(byte: u8) -> Self {
        match byte {
            0 => FormType::STRING,
            1 => FormType::WORLD,
            _ => panic!("Unknown FormType"),
        }
    }
}

impl From<&str> for FormType {
    fn from(s: &str) -> Self {
        match s {
            "STRING" => FormType::STRING,
            "WORLD" => FormType::WORLD,
            _ => panic!("Unknown FormType"),
        }
    }
}
