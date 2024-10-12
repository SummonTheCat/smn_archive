use core::fmt;

// -----------------------------  FormID -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct FormID {
    value: u16, 
}

#[allow(unused)]
impl FormID {
    /// Number of bytes for FormID (2 bytes).
    pub const BYTE_COUNT: usize = 2;

    /// Converts FormID to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        self.value.to_be_bytes()
    }

    /// Returns the internal u16 value of FormID.
    pub fn to_u16(&self) -> u16 {
        self.value
    }

    /// Converts FormID to a formatted string.
    pub fn to_string(&self) -> String {
        format!("{:05}", self.value)
    }

    /// Returns the byte count for FormID (always 2).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u16> for FormID {
    /// Creates a FormID from a u16 value.
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<&str> for FormID {
    /// Creates a FormID from a 5-digit string.
    fn from(s: &str) -> Self {
        if s.len() != 5 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("FormID string must be exactly 5 digits long and numeric.");
        }
        let value = s.parse::<u16>().expect("Invalid FormID string.");
        Self { value }
    }
}

impl From<String> for FormID {
    /// Converts a String to FormID.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; FormID::BYTE_COUNT]> for FormID {
    /// Converts a byte array to FormID.
    fn from(bytes: [u8; FormID::BYTE_COUNT]) -> Self {
        let value = u16::from_be_bytes(bytes);
        Self { value }
    }
}

impl From<&GlobalID> for FormID {
    /// Extracts FormID from GlobalID.
    fn from(global_id: &GlobalID) -> Self {
        global_id.form_id
    }
}


// -----------------------------  ArchiveID -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ArchiveID {
    value: u8,
}

#[allow(unused)]
impl ArchiveID {
    /// Number of bytes for ArchiveID (1 byte).
    pub const BYTE_COUNT: usize = 1;

    /// Converts ArchiveID to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        [self.value]
    }

    /// Returns the internal u8 value of ArchiveID.
    pub fn to_u8(&self) -> u8 {
        self.value
    }

    /// Converts ArchiveID to a formatted string.
    pub fn to_string(&self) -> String {
        format!("{:03}", self.value)
    }

    /// Returns the byte count for ArchiveID (always 1).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u8> for ArchiveID {
    /// Creates an ArchiveID from a u8 value.
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<&str> for ArchiveID {
    /// Creates an ArchiveID from a 3-digit string.
    fn from(s: &str) -> Self {
        if s.len() != 3 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("ArchiveID string must be exactly 3 digits long and numeric.");
        }
        let value = s.parse::<u8>().expect("Invalid ArchiveID string.");
        Self { value }
    }
}

impl From<String> for ArchiveID {
    /// Converts a String to ArchiveID.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; ArchiveID::BYTE_COUNT]> for ArchiveID {
    /// Converts a byte array to ArchiveID.
    fn from(bytes: [u8; ArchiveID::BYTE_COUNT]) -> Self {
        Self { value: bytes[0] }
    }
}

impl From<&GlobalID> for ArchiveID {
    /// Extracts ArchiveID from GlobalID.
    fn from(global_id: &GlobalID) -> Self {
        global_id.archive_id
    }
}


// -----------------------------  GlobalID -----------------------------  //
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GlobalID {
    archive_id: ArchiveID,
    form_id: FormID,
}

#[allow(unused)]
impl GlobalID {
    /// Number of bytes for GlobalID (3 bytes: 1 for ArchiveID + 2 for FormID).
    pub const BYTE_COUNT: usize = ArchiveID::BYTE_COUNT + FormID::BYTE_COUNT;

    /// Converts GlobalID to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..ArchiveID::BYTE_COUNT].copy_from_slice(&self.archive_id.to_bytes());
        bytes[ArchiveID::BYTE_COUNT..].copy_from_slice(&self.form_id.to_bytes());
        bytes
    }

    /// Converts GlobalID to a formatted string.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.archive_id.to_string(), self.form_id.to_string())
    }

    /// Returns the byte count for GlobalID (always 3).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<(ArchiveID, FormID)> for GlobalID {
    /// Creates a GlobalID from an ArchiveID and FormID tuple.
    fn from(ids: (ArchiveID, FormID)) -> Self {
        Self {
            archive_id: ids.0,
            form_id: ids.1,
        }
    }
}

impl From<&str> for GlobalID {
    /// Creates a GlobalID from an 8-digit string (3 digits for ArchiveID + 5 digits for FormID).
    fn from(s: &str) -> Self {
        if s.len() != 8 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("GlobalID string must be exactly 8 digits long and numeric.");
        }
        let archive_id = ArchiveID::from(&s[..3]);
        let form_id = FormID::from(&s[3..]);
        Self { archive_id, form_id }
    }
}

impl From<String> for GlobalID {
    /// Converts a String to GlobalID.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; GlobalID::BYTE_COUNT]> for GlobalID {
    /// Converts a byte array to GlobalID.
    fn from(bytes: [u8; GlobalID::BYTE_COUNT]) -> Self {
        let archive_id = ArchiveID::from([bytes[0]]);
        let form_id = FormID::from([bytes[1], bytes[2]]);
        Self { archive_id, form_id }
    }
}

impl From<(&ArchiveID, &FormID)> for GlobalID {
    /// Creates a GlobalID from references to ArchiveID and FormID.
    fn from(ids: (&ArchiveID, &FormID)) -> Self {
        Self {
            archive_id: *ids.0,
            form_id: *ids.1,
        }
    }
}

impl fmt::Display for GlobalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format ArchiveID and FormID with the desired padding.
        write!(
            f,
            "{} {}",
            format!("{:03}", self.archive_id.to_u8()),  // ArchiveID formatted to 3 digits
            format!("{:05}", self.form_id.to_u16())      // FormID formatted to 5 digits
        )
    }
}

impl fmt::Debug for GlobalID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For debug, use the same format as Display
        write!(f, "{}", self)
    }
}


// -----------------------------  EntID -----------------------------  //
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EntID {
    global_id: GlobalID,
    reference_id: FormID,
}

#[allow(unused)]
impl EntID {
    /// Number of bytes for EntID (5 bytes: 3 for GlobalID + 2 for reference FormID).
    pub const BYTE_COUNT: usize = GlobalID::BYTE_COUNT + FormID::BYTE_COUNT;

    /// Converts EntID to a byte array.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..GlobalID::BYTE_COUNT].copy_from_slice(&self.global_id.to_bytes());
        bytes[GlobalID::BYTE_COUNT..].copy_from_slice(&self.reference_id.to_bytes());
        bytes
    }

    /// Converts EntID to a formatted string.
    pub fn to_string(&self) -> String {
        format!("{}{}", self.global_id.to_string(), self.reference_id.to_string())
    }

    /// Returns the byte count for EntID (always 5).
    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<(GlobalID, FormID)> for EntID {
    /// Creates an EntID from a GlobalID and a reference FormID tuple.
    fn from(ids: (GlobalID, FormID)) -> Self {
        Self {
            global_id: ids.0,
            reference_id: ids.1,
        }
    }
}

impl From<&str> for EntID {
    /// Creates an EntID from a 13-digit string (8 digits for GlobalID + 2 digits for reference FormID).
    fn from(s: &str) -> Self {
        if s.len() != 13 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("EntID string must be exactly 10 digits long and numeric.");
        }
        let global_id = GlobalID::from(&s[..8]);
        let reference_id = FormID::from(&s[8..]);
        Self { global_id, reference_id }
    }
}

impl From<String> for EntID {
    /// Converts a String to EntID.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; EntID::BYTE_COUNT]> for EntID {
    /// Converts a byte array to EntID.
    fn from(bytes: [u8; EntID::BYTE_COUNT]) -> Self {
        let global_id = GlobalID::from([bytes[0], bytes[1], bytes[2]]);
        let reference_id = FormID::from([bytes[3], bytes[4]]);
        Self { global_id, reference_id }
    }
}

impl EntID {
    /// Extracts the GlobalID from the EntID.
    pub fn global_id(&self) -> GlobalID {
        self.global_id
    }

    /// Extracts the reference FormID from the EntID.
    pub fn reference_id(&self) -> FormID {
        self.reference_id
    }
}


impl fmt::Display for EntID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format GlobalID and reference FormID with the desired padding.
        write!(
            f,
            "{} {}",   // Call the Display of GlobalID first
            self.global_id,
            format!("{:05}", self.reference_id.to_u16())  // FormID formatted to 5 digits for the reference ID
        )
    }
}

impl fmt::Debug for EntID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For debug, use the same format as Display
        write!(f, "{}", self)
    }
}
