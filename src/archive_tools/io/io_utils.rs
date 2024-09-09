use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

use crate::archive_tools::structs::types::*;

use super::IOStructIndexItem;

pub fn get_index_form_id_first(file: &mut File) -> Result<FormID, std::io::Error> {
    // Read the first form id
    let first_form_id_buffer = &mut [0u8; 2];
    file.read_exact(first_form_id_buffer)?;
    let first_form_id_raw = u16::from_be_bytes(*first_form_id_buffer);
    let first_form_id = FormID::from(first_form_id_raw);
    
    // Seek back the 2 bytes read to the start of the form
    file.seek(std::io::SeekFrom::Current(-2))?;
    Ok(first_form_id)
}

pub fn get_index_form_id_last(file: &mut File, form_count: u16) -> Result<FormID, std::io::Error> {
    // Go to the last form_id: form_count * 7 (form_id:2 + form_type:1 + form_offset:4) - 7
    file.seek(std::io::SeekFrom::Current((form_count as i64 * 7) - 7))?;

    // Read the last form id
    let last_form_id_buffer = &mut [0u8; 2];
    file.read_exact(last_form_id_buffer)?;
    let last_form_id_raw = u16::from_be_bytes(*last_form_id_buffer);
    let last_form_id = FormID::from(last_form_id_raw);

    Ok(last_form_id)
}

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

        // Calculate mid-position
        let mid = left + (((right - left) / item_size as u64) / 2 * item_size as u64);

        // Seek to the middle index item
        file.seek(std::io::SeekFrom::Start(mid))?;

        // Read FormID at mid
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id < target_form_id {
            // Update last valid position and item
            last_valid_position = Some(mid);
            _last_valid_item = Some(read_index_item_at_position(file, mid)?);

            // Move left up to the next item
            left = mid + item_size as u64;
        } else {
            // Move right down to this item's start
            right = mid;
        }
    }

    // After binary search, check the last valid position
    if right - left <= item_size as u64 {
        file.seek(SeekFrom::Start(left))?;

        // Read the FormID at the last position
        let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buf)?;
        let form_id = FormID::from(form_id_buf);

        if form_id < target_form_id {
            last_valid_position = Some(left);
            _last_valid_item = Some(read_index_item_at_position(file, left)?);
        }
    }

    // Move to the next item to get the first item that is greater than the target_form_id
    if let Some(byte_position) = last_valid_position {
        let next_position = byte_position + item_size as u64;
        file.seek(SeekFrom::Start(next_position))?;
        
        // Read the next index item
        let next_item = read_index_item_at_position(file, next_position)?;

        // Return the position as the next index in the form list and the next index item
        let index_position = (next_position - starting_position) / item_size as u64;
        return Ok(Some((index_position, next_item)));
    }

    // If no valid item was found, it means the new form should be inserted at the beginning
    Ok(None)
}

fn read_index_item_at_position(file: &mut File, position: u64) -> io::Result<IOStructIndexItem> {
    file.seek(SeekFrom::Start(position))?;

    // Read FormID
    let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
    file.read_exact(&mut form_id_buf)?;
    let form_id = FormID::from(form_id_buf);

    // Read FormType
    let mut form_type_buf = [0u8; 1];
    file.read_exact(&mut form_type_buf)?;
    let form_type = FormType::from(form_type_buf[0]);

    // Read Data Start Offset
    let mut data_start_offset_buf = [0u8; 4];
    file.read_exact(&mut data_start_offset_buf)?;
    let data_start_offset = u32::from_be_bytes(data_start_offset_buf);

    Ok(IOStructIndexItem {
        form_id,
        form_type,
        data_start_offset,
    })
}

