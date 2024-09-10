// -----------------------------  FormID -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct FormID {
    value: u16, 
}

#[allow(unused)]
impl FormID {
    pub const BYTE_COUNT: usize = 2;

    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        self.value.to_be_bytes()
    }

    pub fn to_u16(&self) -> u16 {
        self.value
    }

    pub fn to_string(&self) -> String {
        format!("{:05}", self.value)
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u16> for FormID {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<&str> for FormID {
    fn from(s: &str) -> Self {
        if s.len() != 5 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("FormID string must be exactly 5 digits long and numeric.");
        }
        let value = s.parse::<u16>().expect("Invalid FormID string.");
        Self { value }
    }
}

impl From<String> for FormID {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; FormID::BYTE_COUNT]> for FormID {
    fn from(bytes: [u8; FormID::BYTE_COUNT]) -> Self {
        let value = u16::from_be_bytes(bytes);
        Self { value }
    }
}

impl From<&GlobalID> for FormID {
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
    pub const BYTE_COUNT: usize = 1;

    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        [self.value]
    }

    pub fn to_u8(&self) -> u8 {
        self.value
    }

    pub fn to_string(&self) -> String {
        format!("{:03}", self.value)
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<u8> for ArchiveID {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<&str> for ArchiveID {
    fn from(s: &str) -> Self {
        if s.len() != 3 || !s.chars().all(|c| c.is_digit(10)) {
            panic!("ArchiveID string must be exactly 3 digits long and numeric.");
        }
        let value = s.parse::<u8>().expect("Invalid ArchiveID string.");
        Self { value }
    }
}

impl From<String> for ArchiveID {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; ArchiveID::BYTE_COUNT]> for ArchiveID {
    fn from(bytes: [u8; ArchiveID::BYTE_COUNT]) -> Self {
        Self { value: bytes[0] }
    }
}

impl From<&GlobalID> for ArchiveID {
    fn from(global_id: &GlobalID) -> Self {
        global_id.archive_id
    }
}


// -----------------------------  GlobalID -----------------------------  //
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GlobalID {
    archive_id: ArchiveID,
    form_id: FormID,
}

#[allow(unused)]
impl GlobalID {
    pub const BYTE_COUNT: usize = ArchiveID::BYTE_COUNT + FormID::BYTE_COUNT;

    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        bytes[..ArchiveID::BYTE_COUNT].copy_from_slice(&self.archive_id.to_bytes());
        bytes[ArchiveID::BYTE_COUNT..].copy_from_slice(&self.form_id.to_bytes());
        bytes
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.archive_id.to_string(), self.form_id.to_string())
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT
    }
}

impl From<(ArchiveID, FormID)> for GlobalID {
    fn from(ids: (ArchiveID, FormID)) -> Self {
        Self {
            archive_id: ids.0,
            form_id: ids.1,
        }
    }
}

impl From<&str> for GlobalID {
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
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<[u8; GlobalID::BYTE_COUNT]> for GlobalID {
    fn from(bytes: [u8; GlobalID::BYTE_COUNT]) -> Self {
        let archive_id = ArchiveID::from([bytes[0]]);
        let form_id = FormID::from([bytes[1], bytes[2]]);
        Self { archive_id, form_id }
    }
}

impl From<(&ArchiveID, &FormID)> for GlobalID {
    fn from(ids: (&ArchiveID, &FormID)) -> Self {
        Self {
            archive_id: *ids.0,
            form_id: *ids.1,
        }
    }
}
