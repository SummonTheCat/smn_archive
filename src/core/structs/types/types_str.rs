use std::fmt;
use std::fs::File;
use std::io::{self, Read};

// -----------------------------  StrSml -----------------------------  //
#[derive(PartialEq, Eq, Clone)]
pub struct StrSml {
    value: Vec<u16>,
}

impl StrSml {
    /// Number of bytes used to store the character count (1 byte).
    pub const CHAR_COUNT_BYTE_SIZE: usize = 1;

    /// Converts `StrSml` to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::CHAR_COUNT_BYTE_SIZE + self.value.len() * 2);
        bytes.push(self.value.len() as u8);  // Store length in 1 byte
        for &ch in &self.value {
            bytes.extend_from_slice(&ch.to_be_bytes());  // Add UTF-16 characters
        }
        bytes
    }

    /// Converts `StrSml` to a string (assumes valid UTF-16).
    pub fn to_string(&self) -> String {
        String::from_utf16(&self.value).expect("Invalid UTF-16 sequence")
    }

    /// Returns the byte count of the serialized `StrSml`.
    pub fn get_byte_count(&self) -> usize {
        Self::CHAR_COUNT_BYTE_SIZE + self.value.len() * 2
    }
}

impl From<&str> for StrSml {
    /// Converts a UTF-8 string into a `StrSml`, ensuring no surrogate pairs.
    fn from(s: &str) -> Self {
        let utf16: Vec<u16> = s.encode_utf16().collect();

        // Check for characters that require surrogate pairs.
        for &code_unit in &utf16 {
            if code_unit >= 0xD800 && code_unit <= 0xDFFF {
                panic!("StrSml cannot contain characters that require more than 2 bytes in UTF-16.");
            }
        }

        if utf16.len() > u8::MAX as usize {
            panic!("StrSml can only contain up to 255 characters.");
        }
        Self { value: utf16 }
    }
}

impl From<String> for StrSml {
    /// Converts a `String` into `StrSml`.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[allow(unused)]
impl StrSml {
    /// Reads `StrSml` from a file, assuming UTF-16 encoding.
    pub fn read_from_bytes(file: &mut File) -> io::Result<Self> {
        let mut length_buffer = [0u8; 1];
        file.read_exact(&mut length_buffer)?;

        let char_count = length_buffer[0] as usize;
        let mut value = Vec::with_capacity(char_count);

        for _ in 0..char_count {
            let mut char_buffer = [0u8; 2];
            file.read_exact(&mut char_buffer)?;
            value.push(u16::from_be_bytes(char_buffer));
        }

        Ok(Self { value })
    }

    /// Reads `StrSml` from a byte buffer.
    pub fn read_from_byte_buffer(bytes: &[u8]) -> io::Result<(Self, usize)> {
        let mut offset = 0;

        if bytes.len() < 1 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for string length"));
        }
        let char_count = bytes[offset] as usize;
        offset += 1;

        let mut value = Vec::with_capacity(char_count);

        for _ in 0..char_count {
            if bytes.len() < offset + 2 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for string characters"));
            }
            let char_buffer: [u8; 2] = bytes[offset..offset + 2].try_into().unwrap();
            value.push(u16::from_be_bytes(char_buffer));
            offset += 2;
        }

        Ok((Self { value }, offset))
    }
}

impl fmt::Display for StrSml {
    /// Formats the `StrSml` as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for StrSml {
    /// Formats the `StrSml` with additional details for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StrSml {{ char_count: {}, content: \"{}\", byte_count: {} }}",
            self.value.len(),
            self.to_string(),
            self.get_byte_count()
        )
    }
}


// -----------------------------  StrLrg -----------------------------  //
#[derive(PartialEq, Eq, Clone)]
pub struct StrLrg {
    value: Vec<u16>,
}

impl StrLrg {
    /// Number of bytes used to store the character count (2 bytes).
    pub const CHAR_COUNT_BYTE_SIZE: usize = 2;

    /// Converts `StrLrg` to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::CHAR_COUNT_BYTE_SIZE + self.value.len() * 2);
        bytes.extend_from_slice(&(self.value.len() as u16).to_be_bytes());  // Store length in 2 bytes
        for &ch in &self.value {
            bytes.extend_from_slice(&ch.to_be_bytes());  // Add UTF-16 characters
        }
        bytes
    }

    /// Converts `StrLrg` to a string (assumes valid UTF-16).
    pub fn to_string(&self) -> String {
        String::from_utf16(&self.value).expect("Invalid UTF-16 sequence")
    }

    /// Returns the byte count of the serialized `StrLrg`.
    pub fn get_byte_count(&self) -> usize {
        Self::CHAR_COUNT_BYTE_SIZE + self.value.len() * 2
    }
}

impl From<&str> for StrLrg {
    /// Converts a UTF-8 string into a `StrLrg`, ensuring no surrogate pairs.
    fn from(s: &str) -> Self {
        let utf16: Vec<u16> = s.encode_utf16().collect();

        // Check for characters that require surrogate pairs.
        for &code_unit in &utf16 {
            if code_unit >= 0xD800 && code_unit <= 0xDFFF {
                panic!("StrLrg cannot contain characters that require more than 2 bytes in UTF-16.");
            }
        }

        if utf16.len() > u16::MAX as usize {
            panic!("StrLrg can only contain up to 65,535 characters.");
        }
        Self { value: utf16 }
    }
}

impl From<String> for StrLrg {
    /// Converts a `String` into `StrLrg`.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[allow(unused)]
impl StrLrg {
    /// Reads `StrLrg` from a file, assuming UTF-16 encoding.
    pub fn read_from_bytes(file: &mut File) -> io::Result<Self> {
        let mut length_buffer = [0u8; 2];
        file.read_exact(&mut length_buffer)?;

        let char_count = u16::from_be_bytes(length_buffer) as usize;
        let mut value = Vec::with_capacity(char_count);

        for _ in 0..char_count {
            let mut char_buffer = [0u8; 2];
            file.read_exact(&mut char_buffer)?;
            value.push(u16::from_be_bytes(char_buffer));
        }

        Ok(Self { value })
    }
    
    /// Reads `StrLrg` from a byte buffer.
    pub fn read_from_byte_buffer(bytes: &[u8]) -> io::Result<(Self, usize)> {
        let mut offset = 0;

        if bytes.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for string length"));
        }
        let length_buffer: [u8; 2] = bytes[offset..offset + 2].try_into().unwrap();
        let char_count = u16::from_be_bytes(length_buffer) as usize;

        offset += 2;

        let mut value = Vec::with_capacity(char_count);

        for _ in 0..char_count {
            if bytes.len() < offset + 2 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for string characters"));
            }
            let char_buffer: [u8; 2] = bytes[offset..offset + 2].try_into().unwrap();

            value.push(u16::from_be_bytes(char_buffer));
            offset += 2;
        }

        Ok((Self { value }, offset))
    }
}

impl fmt::Display for StrLrg {
    /// Formats the `StrLrg` as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for StrLrg {
    /// Formats the `StrLrg` with additional details for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StrLrg {{ char_count: {}, content: \"{}\", byte_count: {} }}",
            self.value.len(),
            self.to_string(),
            self.get_byte_count()
        )
    }
}
