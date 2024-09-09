use crate::archive_tools::structs::{ArchiveID, FormID, FormType, StrLrg, Version};

#[derive(Debug)]
pub struct IOStructByteStarts {
    pub bytestart_index: u32,
    pub bytestart_data: u32,
}

#[derive(Debug)]
pub struct IOStructHeader {
    pub archive_id: ArchiveID,
    pub version: Version,
    pub description: StrLrg,
    pub form_count: u16,
}

#[derive(Debug)]
pub struct IOStructIndex {
    pub indexes: Vec<IOStructIndexItem>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct IOStructIndexItem {
    pub form_id: FormID,
    pub form_type: FormType,
    pub data_start_offset: u32,
}