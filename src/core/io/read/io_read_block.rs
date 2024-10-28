use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use crate::core::io::{
    IOStructByteStarts, IOStructHeader, IOStructIndex, IOStructIndexItem,
};
use crate::core::structs::types::*;

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
    let form_count = u16::from_le_bytes(form_count_buf);

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
    let mut bytestart_index_buf = [0u8; 4];
    file.read_exact(&mut bytestart_index_buf)?;
    let bytestart_index = u32::from_le_bytes(bytestart_index_buf);

    // Read BYTESTART Data
    let mut bytestart_data_buf = [0u8; 4];
    file.read_exact(&mut bytestart_data_buf)?;
    let bytestart_data = u32::from_le_bytes(bytestart_data_buf);

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
        let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

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

pub fn binary_search_for_index_item(
    file: &mut File,
    target_form_id: FormID,
    form_count: u16,
) -> io::Result<Option<IOStructIndexItem>> {
    let item_size = FormID::BYTE_COUNT + 1 + 4; // 7 bytes per index item
    let mut left = file.stream_position()?; // Start at current file position (index block start)
    let mut right = left + (item_size * form_count as usize) as u64;

    let mut passes = 0;
    let max_passes = 30; // Number of binary search passes before switching to linear search

    while passes < max_passes {
        passes += 1;
        let mid: u64;

        // Check if the range is too small to continue
        if right - left <= item_size as u64 {
            break;
        }

        // Calculate mid position and align to the nearest multiple of item_size (7 bytes)
        mid = left + (((right - left) / 2) / item_size as u64) * item_size as u64;

        // Seek to the middle index item
        file.seek(std::io::SeekFrom::Start(mid))?;

        // Read FormID at mid
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id == target_form_id {
            // We found the target FormID, now read the whole index item
            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;
            let form_type = FormType::from(form_type_buf[0]);

            let mut data_start_offset_buf = [0u8; 4];
            file.read_exact(&mut data_start_offset_buf)?;
            let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

            return Ok(Some(IOStructIndexItem {
                form_id,
                form_type,
                data_start_offset,
            }));
        } else if form_id < target_form_id {
            // Move left up to the next item
            left = mid + item_size as u64;
        } else {
            // Move right down to this item's start
            right = mid;
        }
    }

    // After binary search passes, perform a linear search from the left position to the right position
    file.seek(std::io::SeekFrom::Start(left))?;
    while left < right {
        // Read FormID
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id == target_form_id {
            // Read FormType
            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;
            let form_type = FormType::from(form_type_buf[0]);

            // Read Data Start Offset
            let mut data_start_offset_buf = [0u8; 4];
            file.read_exact(&mut data_start_offset_buf)?;
            let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

            return Ok(Some(IOStructIndexItem {
                form_id,
                form_type,
                data_start_offset,
            }));
        }

        // Move to the next item
        left += item_size as u64;
        file.seek(std::io::SeekFrom::Start(left))?;
    }
    // If we reach here, the target FormID was not found
    Ok(None)
}

pub fn binary_search_for_index_item_and_position(
    file: &mut File,
    target_form_id: FormID,
    form_count: u16,
) -> io::Result<Option<(u64, IOStructIndexItem)>> {
    let item_size = FormID::BYTE_COUNT + 1 + 4; // 7 bytes per index item
    let start = file.stream_position()?; // Start at current file position (index block start)
    let mut left = file.stream_position()?; // Start at current file position (index block start)
    let mut right = left + (item_size * form_count as usize) as u64;

    let mut passes = 0;
    let max_passes = 10; // Number of binary search passes before switching to linear search

    while passes < max_passes {
        passes += 1;
        let mid: u64;

        // Check if the range is too small to continue
        if right - left <= item_size as u64 {
            break;
        }

        // Calculate mid position and align to the nearest multiple of item_size (7 bytes)
        mid = left + (((right - left) / 2) / item_size as u64) * item_size as u64;

        // Seek to the middle index item
        file.seek(std::io::SeekFrom::Start(mid))?;

        // Read FormID at mid
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id == target_form_id {
            // We found the target FormID, now read the whole index item
            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;
            let form_type = FormType::from(form_type_buf[0]);

            let mut data_start_offset_buf = [0u8; 4];
            file.read_exact(&mut data_start_offset_buf)?;
            let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

            // Calculate the position in the index
            let index_position = (mid - left) / item_size as u64;

            return Ok(Some((
                index_position,
                IOStructIndexItem {
                    form_id,
                    form_type,
                    data_start_offset,
                },
            )));
        } else if form_id < target_form_id {
            // Move left up to the next item
            left = mid + item_size as u64;
        } else {
            // Move right down to this item's start
            right = mid;
        }
    }

    // After binary search passes, perform a linear search from the left position to the right position
    file.seek(std::io::SeekFrom::Start(left))?;
    while left < right {
        // Index position is (left - start)/ item_size(7)
        let index_position = (left - start) / item_size as u64;

        // Read FormID
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id == target_form_id {
            // Read FormType
            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;
            let form_type = FormType::from(form_type_buf[0]);

            // Read Data Start Offset
            let mut data_start_offset_buf = [0u8; 4];
            file.read_exact(&mut data_start_offset_buf)?;
            let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

            return Ok(Some((
                index_position,
                IOStructIndexItem {
                    form_id,
                    form_type,
                    data_start_offset,
                },
            )));
        }

        // Move to the next item
        left += item_size as u64;
        file.seek(SeekFrom::Start(left))?;
    }

    // If we reach here, the target FormID was not found
    Ok(None)
}

#[allow(unused)]
pub fn binary_search_for_index_item_inmem(
    target_form_ids: Vec<FormID>,
    index: &IOStructIndex,
) -> io::Result<Option<IOStructIndex>> {
    let mut result_indexes = Vec::new();

    // For each target FormID, perform binary search over the index
    for target_form_id in target_form_ids {
        match index
            .indexes
            .binary_search_by_key(&target_form_id, |item| item.form_id)
        {
            Ok(pos) => {
                // Found the item at position `pos`
                let index_item = index.indexes[pos].clone();
                result_indexes.push(index_item);
            }
            Err(_) => {
                // FormID not found; return an error or handle as needed
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("FormID {} not found in archive.", target_form_id.to_string()),
                ));
            }
        }
    }

    if result_indexes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(IOStructIndex {
            indexes: result_indexes,
        }))
    }
}
