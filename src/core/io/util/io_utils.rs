use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::core::structs::types::*;

use super::IOStructIndexItem;

/// Reads the first form ID from the index block using Little-Endian byte order.
pub fn get_index_form_id_first(file: &mut File) -> Result<FormID, std::io::Error> {
    // Read the first form ID (2 bytes) in Little-Endian.
    let first_form_id_buffer = &mut [0u8; 2];
    file.read_exact(first_form_id_buffer)?;
    let first_form_id_raw = u16::from_le_bytes(*first_form_id_buffer);
    let first_form_id = FormID::from(first_form_id_raw);
    
    // Seek back the 2 bytes read to the start of the form.
    file.seek(std::io::SeekFrom::Current(-2))?;
    Ok(first_form_id)
}

/// Reads the last form ID from the index block using Little-Endian byte order.
pub fn get_index_form_id_last(file: &mut File, form_count: u16) -> Result<FormID, std::io::Error> {
    // Calculate the byte position of the last form ID:
    // (form_count * 7 bytes per form) - 7 bytes to get to the last form.
    file.seek(std::io::SeekFrom::Current((form_count as i64 * 7) - 7))?;

    // Read the last form ID (2 bytes) in Little-Endian.
    let last_form_id_buffer = &mut [0u8; 2];
    file.read_exact(last_form_id_buffer)?;
    let last_form_id_raw = u16::from_le_bytes(*last_form_id_buffer);
    let last_form_id = FormID::from(last_form_id_raw);

    Ok(last_form_id)
}

/// Reads the form ID at a specific index position using Little-Endian byte order.
pub fn get_new_form_index_pos(
    file: &mut File, 
    form_count: u16, 
    target_form_id: FormID
) -> io::Result<Option<(u64, IOStructIndexItem)>> {
    let item_size = FormID::BYTE_COUNT + 1 + 4; // 7 bytes per index item
    let starting_position = file.stream_position()?; // Start at current file position (index block start)
    let mut left = starting_position;
    let mut right = left + (item_size * form_count as usize) as u64;

    let mut last_valid_position = None;
    let mut _last_valid_item = None;

    while right - left > item_size as u64 {
        // Calculate mid-position aligned to the start of an index item.
        let mid = left + (((right - left) / item_size as u64) / 2 * item_size as u64);

        // Seek to the middle index item.
        file.seek(std::io::SeekFrom::Start(mid))?;

        // Read FormID at mid in Little-Endian.
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id < target_form_id {
            // Update last valid position and item.
            last_valid_position = Some(mid);
            _last_valid_item = Some(read_index_item_at_position(file, mid)?);

            // Move left up to the next item.
            left = mid + item_size as u64;
        } else {
            // Move right down to this item's start.
            right = mid;
        }
    }

    // After binary search, check the last valid position.
    if right - left <= item_size as u64 {
        file.seek(std::io::SeekFrom::Start(left))?;

        // Read the FormID at the last position in Little-Endian.
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id < target_form_id {
            last_valid_position = Some(left);
            _last_valid_item = Some(read_index_item_at_position(file, left)?);
        }
    }

    // Move to the next item to get the first item that is greater than the target_form_id.
    if let Some(byte_position) = last_valid_position {
        let next_position = byte_position + item_size as u64;
        file.seek(std::io::SeekFrom::Start(next_position))?;
        
        // Read the next index item in Little-Endian.
        let next_item = read_index_item_at_position(file, next_position)?;

        // Calculate the index position relative to the starting position.
        let index_position = (next_position - starting_position) / item_size as u64;
        return Ok(Some((index_position, next_item)));
    }

    // If no valid item was found, it means the new form should be inserted at the beginning.
    Ok(None)
}

/// Reads an index item at a specific position using Little-Endian byte order.
fn read_index_item_at_position(file: &mut File, position: u64) -> io::Result<IOStructIndexItem> {
    file.seek(SeekFrom::Start(position))?;

    // Read FormID (2 bytes) in Little-Endian.
    let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
    file.read_exact(&mut form_id_buf)?;
    let form_id = FormID::from(form_id_buf);

    // Read FormType (1 byte).
    let mut form_type_buf = [0u8; 1];
    file.read_exact(&mut form_type_buf)?;
    let form_type = FormType::from(form_type_buf[0]);

    // Read Data Start Offset (4 bytes) in Little-Endian.
    let mut data_start_offset_buf = [0u8; 4];
    file.read_exact(&mut data_start_offset_buf)?;
    let data_start_offset = u32::from_le_bytes(data_start_offset_buf);

    Ok(IOStructIndexItem {
        form_id,
        form_type,
        data_start_offset,
    })
}
