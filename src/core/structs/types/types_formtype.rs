// ----------------------------- FormType ----------------------------- //
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FormType {
    STRING = 0,
    WORLD = 1,
    REFGROUP = 2,
    WORLDPART = 3,  
    WEATHER = 4,
}

#[allow(unused)]
impl FormType {
    /// Number of bytes for `FormType` (1 byte).
    pub const BYTE_COUNT: usize = 1;

    /// Converts `FormType` to its string representation.
    pub fn to_string(&self) -> String {
        match self {
            FormType::STRING => "STRING".to_string(),
            FormType::WORLD => "WORLD".to_string(),
            FormType::REFGROUP => "REFGROUP".to_string(),
            FormType::WORLDPART => "WORLDPART".to_string(),
            FormType::WEATHER => "WEATHER".to_string(),
        }
    }

    /// Converts `FormType` to its integer representation.
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Converts `FormType` to a byte.
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }

    /// Returns the byte count for `FormType` (always 1).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u8> for FormType {
    /// Creates a `FormType` from a byte.
    fn from(byte: u8) -> Self {
        match byte {
            0 => FormType::STRING,
            1 => FormType::WORLD,
            2 => FormType::REFGROUP,
            3 => FormType::WORLDPART,
            4 => FormType::WEATHER,
            _ => panic!("Unknown FormType"),
        }
    }
}

impl From<&str> for FormType {
    /// Creates a `FormType` from a string.
    fn from(s: &str) -> Self {
        match s {
            "STRING" => FormType::STRING,
            "WORLD" => FormType::WORLD,
            "REFGROUP" => FormType::REFGROUP,
            "WORLDPART" => FormType::WORLDPART,
            "WEATHER" => FormType::WEATHER,
            _ => panic!("Unknown FormType"),
        }
    }
}
