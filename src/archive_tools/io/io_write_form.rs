use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::archive_tools::io::*;
use crate::archive_tools::structs::*;

#[allow(unused)]
pub fn write_form(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {

    let mut archive_info: Archive;
    let mut new_form_bytes: Vec<u8> = Vec::new();

    let mut form_length_old: u32  = u32::MAX;
    let mut form_length_new: u32  = u32::MAX;
    let mut form_length_diff: i32 = 0;

    let mut found_form: bool = false;
    let mut found_index_pos: u32 = u32::MAX;
    let mut form_index: IOStructIndex = IOStructIndex{ indexes: Vec::new() };

    // Set up initial working variables
    archive_info = read_archive_info(file_path)?;
    new_form_bytes = form.to_bytes();
    form_length_new = new_form_bytes.len() as u32;

    let mut file = File::options()
        .read(true)
        .write(true)
        .open(file_path)?;

    if archive_info.form_count == 0 {
        form_length_old = 0;
        form_length_diff = form_length_new as i32;
        let form_index_item = IOStructIndexItem {
            form_id: form.form_id(),
            form_type: form.form_type(),
            data_start_offset: 0,
        };
        form_index.indexes.push(form_index_item);

        archive_info.form_count = 1;
        archive_info.bytestart_index += form_length_diff as u32;

        write_form_to_empty(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            
    } else {
        // Search in index
        file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
        form_index = read_block_index(&mut file, archive_info.form_count)?;

        for (i, item) in form_index.indexes.iter().enumerate() {
            if item.form_id == form.form_id() {
                found_form = true;
                found_index_pos = i as u32;  // Set found_index_pos to the current index
                break;
            }
        }

        if found_form {
            // Get the form's length
            let form_bytepos = form_index.indexes[found_index_pos as usize].data_start_offset + archive_info.bytestart_data;
            file.seek(SeekFrom::Start(form_bytepos as u64))?;
            let read_form = FormBase::read_from_bytes(&mut file)?;
            form_length_old = read_form.get_byte_count() as u32;
            form_length_diff = form_length_new as i32 - form_length_old as i32;

            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

            // check if we are not the last form
            if found_index_pos == form_index.indexes.len() as u32 - 1 {
                write_form_over_last(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            } else {
                // Update the index items after the form with the form diff
                for item in form_index.indexes.iter_mut().skip(found_index_pos as usize + 1) {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }
                write_form_over_existing(&mut file, &archive_info, &new_form_bytes, &mut form_index, found_index_pos, form_length_diff)?;
            }
        } else {
            form_length_diff = form_length_new as i32;

            // Find new placement in the index (sorted numerically by FormID)
            let mut index_pos: usize = 0;
            for (i, item) in form_index.indexes.iter().enumerate() {
                if item.form_id < form.form_id() {
                    index_pos = i + 1;
                }
            }

            if index_pos == 0 {
                // Append the new index item to the beginning of the index
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: 0,
                };
                // Move all index's offsets by the new form length except the first one
                for item in form_index.indexes.iter_mut() {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }
                form_index.indexes.insert(0, form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                // Write the form to the beginning of the file
                write_form_to_beginning(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            } else if index_pos == form_index.indexes.len() {
                // Read the last index item's offset
                let last_index_item_offset = form_index.indexes.last().unwrap().data_start_offset;
                // new File handle to read the last form's length
                let last_form = read_form(file_path, form_index.indexes.last().unwrap().form_id)?;
                let last_form_length = last_form.get_byte_count() as u32;
                // Create new index item
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: last_index_item_offset + last_form_length,
                };
                form_index.indexes.push(form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                // Write the form to the end of the file
                write_form_to_end(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            } else {
                // Take the offset from the next index item
                let next_index_item_offset = form_index.indexes[index_pos].data_start_offset;
                // insert the new form at the index position
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: next_index_item_offset,
                };

                // Move all index's offsets by the new form length starting from the next index item
                for item in form_index.indexes.iter_mut().skip(index_pos) {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }

                form_index.indexes.insert(index_pos, form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                // Write the form inside the file
                write_form_to_inside(&mut file, &archive_info, &new_form_bytes, &mut form_index, index_pos as u32)?;
            }
        }
    }

    Ok(())
}

pub fn write_form_to_empty(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    file.seek(SeekFrom::Start(archive.bytestart_data as u64))?;
    file.write_all(form_bytes)?;

    write_block_index(file, form_index)?;

    Ok(())
}

pub fn write_form_to_beginning(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    let mut temp_buffer: Vec<u8> = Vec::new();
    file.seek(SeekFrom::Start(archive.bytestart_data as u64))?;
    file.read_to_end(&mut temp_buffer)?;

    file.seek(SeekFrom::Start(archive.bytestart_data as u64))?;
    file.write_all(form_bytes)?;

    file.write_all(&temp_buffer)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}

pub fn write_form_to_end(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    // Get the start position of the final form in the index list (our new form's start position)
    let form_start_pos = form_index.indexes.last().unwrap().data_start_offset + archive.bytestart_data;

    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}
    
pub fn write_form_to_inside(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex, form_index_position: u32) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    // Get the start position of next form in the index list (one after our new form)
    let next_form_start_pos = (form_index.indexes[form_index_position as usize + 1].data_start_offset as i32 + archive.bytestart_data as i32 - form_bytes.len() as i32) as u32;

    let mut temp_buffer: Vec<u8> = Vec::new();
    file.seek(SeekFrom::Start(next_form_start_pos as u64))?;
    file.read_to_end(&mut temp_buffer)?;

    // Get the start position of the new form in the index list
    let form_start_pos = form_index.indexes[form_index_position as usize].data_start_offset + archive.bytestart_data;
    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    
    file.write_all(form_bytes)?;

    file.write_all(&temp_buffer)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}

pub fn write_form_over_existing(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex, form_index_position: u32, form_length_diff: i32) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;
    
    write_block_header(file, archive)?; 
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;
    
    // Get the old position of the form after the one we are replacing
    let next_form_start_pos = (form_index.indexes[form_index_position as usize + 1].data_start_offset as i32 + archive.bytestart_data as i32 - form_length_diff) as u32;

    // Save the rest of the forms after the one we are replacing to a temporary buffer
    let mut temp_buffer: Vec<u8> = Vec::new();
    file.seek(SeekFrom::Start(next_form_start_pos as u64))?;
    file.read_to_end(&mut temp_buffer)?;

    // Get the start position of the form we are replacing
    let form_start_pos = form_index.indexes[form_index_position as usize].data_start_offset + archive.bytestart_data;
    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    file.write_all(&temp_buffer)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}

pub fn write_form_over_last(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;
    
    write_block_header(file, archive)?; 
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;
    
    // Get the start position of the final form in the index list (our new form's start position)
    let form_start_pos = form_index.indexes.last().unwrap().data_start_offset + archive.bytestart_data;

    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}
