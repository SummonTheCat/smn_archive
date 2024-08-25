use crate::archive_tools::types::{ArchiveID, FormID, FormType, StrLrg, StrSml, Version};
use crate::archive_tools::structs::FormTrait;
use std::fmt;

pub struct Archive {
    // Required fields
    pub archive_id: ArchiveID,
    pub version: Version,
    pub description: StrLrg,
    pub form_count: u16,

    // Read fields
    #[allow(unused)]
    pub forms: Vec<Box<dyn FormTrait>>,
    #[allow(unused)]
    pub bytestart_index: u32,
    #[allow(unused)] 
    pub bytestart_data: u32,   
}

impl Archive {
    pub fn new(archive_id: ArchiveID, version: Version, description: StrLrg) -> Self {
        Self {
            archive_id,
            version,
            description,
            form_count: 0,
            forms: Vec::new(),
            bytestart_index: 0, 
            bytestart_data: 0,   
        }
    }

    pub fn get_form_count(&self) -> u16 {
        self.form_count as u16
    }

    pub fn get_header_byte_count(&self) -> usize {
        let byte_count = ArchiveID::BYTE_COUNT
            + Version::BYTE_COUNT
            + self.description.get_byte_count()
            + 2; 
        byte_count
    }
    

    pub fn header_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.get_header_byte_count());
        bytes.extend_from_slice(&self.archive_id.to_bytes());
        bytes.extend_from_slice(&self.version.to_bytes());
        bytes.extend_from_slice(&self.description.to_bytes());
        bytes.extend_from_slice(&(self.get_form_count() as u16).to_be_bytes());
        bytes
    }
}

impl fmt::Display for Archive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Archive ID: {}, Version: {}, Description: {}, Form Count: {}, ByteStartIndex: {}, ByteStartData: {}",
            self.archive_id.to_string(),
            self.version.to_string(),
            self.description.to_string(),
            self.get_form_count(),
            self.bytestart_index,
            self.bytestart_data
        )
    }
}

impl fmt::Debug for Archive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Archive ID: {}, Version: {}, Description: {}, Form Count: {}, ByteStartIndex: {}, ByteStartData: {}",
            self.archive_id.to_string(),
            self.version.to_string(),
            self.description.to_string(),
            self.get_form_count(),
            self.bytestart_index,
            self.bytestart_data
        )
    }
}


pub struct LiteArchive {
    pub archive_id: ArchiveID,
    pub version: Version,
    pub description: StrLrg,
    pub form_count: u16,

    pub archive_items: Vec<LiteArchiveItem>,
}

impl fmt::Debug for LiteArchive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Archive ID: {}, Version: {}, Description: {}, Form Count: {}, Archive Items: {:?}",
            self.archive_id.to_string(),
            self.version.to_string(),
            self.description.to_string(),
            self.form_count,
            self.archive_items
        )
    }
}

pub struct LiteArchiveItem {
    pub form_id: FormID,
    pub form_name: StrSml,
    pub form_type: FormType,
}

impl fmt::Debug for LiteArchiveItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Form ID: {}, Form Name: {}, Form Type: {}",
            self.form_id.to_string(),
            self.form_name.to_string(),
            self.form_type.to_string()
        )
    }
}