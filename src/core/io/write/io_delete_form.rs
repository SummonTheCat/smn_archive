use std::{fs::File, io::{self, Read, Seek, Write}};

use crate::core::structs::*;
use crate::core::io::{read_archive_info, read_block_index, write_block_header, write_block_bytestart, write_block_index};

/// Deletes a form with the specified `form_id` from the archive located at `file_path`.
pub fn delete_form(file_path: &str, form_id: FormID) -> Result<(), io::Error> {
    let mut archive_info: Archive;

    let mut form_length_diff: i32 = 0;
    let mut form_bytestart: u32 = 0;
    let mut form_index_pos: usize = usize::MAX;
    
    // Read archive information
    archive_info = read_archive_info(file_path)?;
    if archive_info.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    // Open the archive file with read and write permissions
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(file_path)?;
    
    // Seek to the index block in the archive
    file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
    let read_index = read_block_index(&mut file, archive_info.form_count);
    if read_index.is_err() {
        return Err(read_index.err().unwrap());
    }
    let mut form_index = read_index.unwrap();

    // Iterate through the index items to find the form to delete
    for (i, index_item) in form_index.indexes.iter().enumerate() {
        if index_item.form_id == form_id {
            form_index_pos = i;
            
            // Seek to the form's byte position and read the form
            file.seek(std::io::SeekFrom::Start(index_item.data_start_offset as u64 + archive_info.bytestart_data as u64))?;
            let found_form = FormBase::read_from_bytes(&mut file)?;

            // Calculate the difference in byte length after deletion
            let form_length_old = found_form.get_byte_count() as u32;
            form_length_diff = 0 - form_length_old as i32;
            form_bytestart = index_item.data_start_offset + archive_info.bytestart_data;

            break;
        }
    }

    // If the form is found and it's the only form in the archive
    if (archive_info.form_count == 1) && (form_index_pos != usize::MAX) {
        archive_info.form_count = 0;
        archive_info.bytestart_index = archive_info.bytestart_data;

        // Write the updated block header
        file.seek(std::io::SeekFrom::Start(0))?;
        let block_header = write_block_header(&mut file, &archive_info);
        if block_header.is_err() {
            return Err(block_header.err().unwrap());
        }

        // Write the updated bytestart block
        let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
        if block_bytestart.is_err() {
            return Err(block_bytestart.err().unwrap());
        }

        // Truncate the file to the current position
        let current_pos = file.seek(std::io::SeekFrom::Current(0))?;
        file.set_len(current_pos)?;
    } else {
        if form_index_pos < form_index.indexes.len() - 1 {
            // ---- INNER DATA REMOVAL ----
            let mut temp_buffer: Vec<u8> = Vec::new();
            let temp_read_start = form_index.indexes[form_index_pos + 1].data_start_offset + archive_info.bytestart_data;

            // Read all data after the form to be deleted
            file.seek(std::io::SeekFrom::Start(temp_read_start as u64))?;
            file.read_to_end(&mut temp_buffer)?;

            // Remove the form's index entry
            form_index.indexes.remove(form_index_pos);
            
            // Adjust the data_start_offset for subsequent forms
            for i in form_index_pos..form_index.indexes.len() {
                form_index.indexes[i].data_start_offset = (form_index.indexes[i].data_start_offset as i32 + form_length_diff) as u32;
            }

            // Update archive information
            archive_info.form_count -= 1;
            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

            // Write the updated block header
            file.seek(std::io::SeekFrom::Start(0))?;
            let block_header = write_block_header(&mut file, &archive_info);
            if block_header.is_err() {
                return Err(block_header.err().unwrap());
            }

            // Write the updated bytestart block
            let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
            if block_bytestart.is_err() {
                return Err(block_bytestart.err().unwrap());
            }

            // Write the remaining data back to the form's byte position
            file.seek(std::io::SeekFrom::Start(form_bytestart as u64))?;
            file.write_all(&temp_buffer)?;

            // Write the updated index block
            file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
            let write_index = write_block_index(&mut file, &form_index);
            if write_index.is_err() {
                return Err(write_index.err().unwrap());
            }
            
            // Truncate the file to the current position
            let current_pos = file.seek(std::io::SeekFrom::Current(0))?;
            file.set_len(current_pos)?;
        } else {
            // ---- END DATA REMOVAL ----
            // Remove the form's index entry
            form_index.indexes.remove(form_index_pos);

            // Update archive information
            archive_info.form_count -= 1;
            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

            // Write the updated block header
            file.seek(std::io::SeekFrom::Start(0))?;
            let block_header = write_block_header(&mut file, &archive_info);
            if block_header.is_err() {
                return Err(block_header.err().unwrap());
            }

            // Write the updated bytestart block
            let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
            if block_bytestart.is_err() {
                return Err(block_bytestart.err().unwrap());
            }
            
            // Write the updated index block
            file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
            let write_index = write_block_index(&mut file, &form_index);
            if write_index.is_err() {
                return Err(write_index.err().unwrap());
            }
        }
    }

    Ok(())
}

