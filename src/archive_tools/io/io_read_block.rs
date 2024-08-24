use std::{fs::File, io::{self, Read}};

use crate::archive_tools::types::*;
use crate::archive_tools::io::*;

pub fn read_block_header(file: &mut File) -> io::Result<IOStructHeader> {
    // Read ArchiveID
    let mut archive_id_buf = [0u8; ArchiveID::BYTE_COUNT];
    file.read_exact(&mut archive_id_buf)?;
    let archive_id = ArchiveID::from(archive_id_buf);

    // Read Version
    let mut version_buf = [0u8; Version::BYTE_COUNT];
    file.read_exact(&mut version_buf)?;
    let version = Version::from(version_buf);

    // Read Archive Description (StrLrg)
    let description = StrLrg::read_from_bytes(file)?;

    // Read Form Count (u16)
    let mut form_count_buf = [0u8; 2]; // 2 bytes for u16
    file.read_exact(&mut form_count_buf)?;
    let form_count = u16::from_be_bytes(form_count_buf);

    // Return the IoStructHeader with the extracted data
    Ok(IOStructHeader {
        archive_id,
        version,
        description,
        form_count,
    })
}

pub fn read_block_bytestarts(file: &mut File) -> io::Result<IOStructByteStarts> {
    // Read BYTESTART Index
    let mut bytestart_index_buf = [0u8; 4]; // 4 bytes for u32
    file.read_exact(&mut bytestart_index_buf)?;
    let bytestart_index = u32::from_be_bytes(bytestart_index_buf);

    // Read BYTESTART Data
    let mut bytestart_data_buf = [0u8; 4]; // 4 bytes for u32
    file.read_exact(&mut bytestart_data_buf)?;
    let bytestart_data = u32::from_be_bytes(bytestart_data_buf);

    // Return the IoStructByteStarts with the extracted data
    Ok(IOStructByteStarts {
        bytestart_index,
        bytestart_data,
    })
}

pub fn read_block_index(file: &mut File, form_count: u16) -> io::Result<IOStructIndex> {
    // Create a new IOStructIndex
    let mut index = IOStructIndex {
        indexes: Vec::new(),
    };

    // Iterate over the form_count
    for _ in 0..form_count {
        // Read FormID
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        // Read FormType
        let mut form_type_buf = [0u8; 1]; // 1 byte for FormType
        file.read_exact(&mut form_type_buf)?;
        let form_type = FormType::from(form_type_buf[0]);

        // Read Data Start Offset
        let mut data_start_offset_buf = [0u8; 4]; // 4 bytes for u32
        file.read_exact(&mut data_start_offset_buf)?;
        let data_start_offset = u32::from_be_bytes(data_start_offset_buf);

        // Create a new IOStructIndexItem and add it to the index
        index.indexes.push(IOStructIndexItem {
            form_id,
            form_type,
            data_start_offset,
        });
    }

    // Return the IOStructIndex with the extracted data
    Ok(index)
}