use std::{fs::File, io::{self, Read, Seek, Write}};

use crate::archive_tools::{io::*, structs::*, types::*};

#[allow(unused)]
pub fn delete_form(file_path: &str, form_id: FormID) -> Result<(), io::Error> {
    let mut archive_info: Archive;

    let mut form_length_old: u32  = u32::MAX;
    let mut form_length_new: u32  = u32::MAX;
    let mut form_length_diff: i32 = 0;
    let mut form_bytestart: u32 = 0;

    let mut form_index: IOStructIndex = IOStructIndex{ indexes: Vec::new() };
    let mut form_index_pos: usize = usize::MAX;
    // Step 1: Read the archive info to get metadata and indexes
    archive_info = read_archive_info(file_path)?;
    if archive_info.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    // Step 2: Open a file in read and write to handle to the archive
    let mut file = File::options()
        .read(true)
        .write(true)
        .open(file_path)?;
    
    // Step 3: Read the index list
    file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
    let read_index = read_block_index(&mut file, archive_info.form_count);
    if read_index.is_err() {
        return Err(read_index.err().unwrap());
    }
    form_index = read_index.unwrap();

    // Step 4: Search through the index list to find the FormID
    println!("> Getting form length for form_id: {:?}...", form_id);
    for (i, index_item) in form_index.indexes.iter().enumerate() {
        if index_item.form_id == form_id {
            form_index_pos = i;
            println!("Form found at index: {}", form_index_pos);
            
            file.seek(std::io::SeekFrom::Start(index_item.data_start_offset as u64 + archive_info.bytestart_data as u64))?;
            let found_form = FormBase::read_from_bytes(&mut file)?;

            form_length_old = found_form.get_byte_count() as u32;
            form_length_new = 0;
            form_length_diff = form_length_new as i32 - form_length_old as i32;
            form_bytestart = index_item.data_start_offset + archive_info.bytestart_data;

            

            println!("Working variables:");
            println!("form_length_old: {}", form_length_old);
            println!("form_length_new: {}", form_length_new);
            println!("form_length_diff: {}", form_length_diff);

            break;
        }
    }

    // if found and is the only form in the archive
    if (archive_info.form_count == 1) && (form_index_pos != usize::MAX) {
        println!("> Deleting the only form in the archive...");

        // Step 4.5: Update the archive info
        archive_info.form_count = 0;
        archive_info.bytestart_index = archive_info.bytestart_data;

        // Write the Header
        file.seek(std::io::SeekFrom::Start(0))?;
        let block_header = write_block_header(&mut file, &archive_info);
        if block_header.is_err() {
            return Err(block_header.err().unwrap());
        }

        let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
        if block_bytestart.is_err() {
            return Err(block_bytestart.err().unwrap());
        }

        // Clip the file to end at this point
        let current_pos = file.seek(std::io::SeekFrom::Current(0))?;
        file.set_len(current_pos)?;

    } else {
        // Step 5: Delete the form from the index list and update the following indexes
        println!("> Deleting form from index list...");
        
        // Check if the form is not the last form in the data
        println!("Comparing form_index_pos: {} with form_index.len(): {}", form_index_pos, form_index.indexes.len());
        if form_index_pos < form_index.indexes.len()-1 {
            // ---- INNER DATA REMOVAL ----
            println!("> Removing form inside data...");
            // Step 4.5: Write the data from the form after the deleted form to a temporary buffer
            let mut temp_buffer: Vec<u8> = Vec::new();
            let temp_read_start = form_index.indexes[form_index_pos+1].data_start_offset + archive_info.bytestart_data;

            // Read the rest of the file into the buffer
            file.seek(std::io::SeekFrom::Start(temp_read_start as u64))?;
            file.read_to_end(&mut temp_buffer)?;

            // DEBUG: READ 2 BYTES from the buffer for FormID
            let mut temp_form_id_buf = [0u8; FormID::BYTE_COUNT];
            temp_form_id_buf.copy_from_slice(&temp_buffer[0..FormID::BYTE_COUNT]);
            let temp_form_id = FormID::from(temp_form_id_buf);
            println!("DEBUG: FormID from buffer: {:?}", temp_form_id);


            // Update the data_start_offset for the following indexes
            form_index.indexes.remove(form_index_pos);
            for i in form_index_pos..form_index.indexes.len() {
                form_index.indexes[i].data_start_offset = (form_index.indexes[i].data_start_offset as i32 + form_length_diff) as u32;
            }

            archive_info.form_count -= 1;
            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;


            // Step 5 Update the file
            file.seek(std::io::SeekFrom::Start(0))?;

            println!("> Writing updated archive...");
            let block_header = write_block_header(&mut file, &archive_info);
            if block_header.is_err() {
                return Err(block_header.err().unwrap());
            }

            let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
            if block_bytestart.is_err() {
                return Err(block_bytestart.err().unwrap());
            }

            // Seek to the form start position
            file.seek(std::io::SeekFrom::Start(form_bytestart as u64))?;

            // DEBUG for FormID
            let mut form_id_buf = [0u8; FormID::BYTE_COUNT];
            file.read_exact(&mut form_id_buf)?;
            let form_id = FormID::from(form_id_buf);
            println!("DEBUG: FormID from file: {:?}", form_id);

            file.seek(std::io::SeekFrom::Start(form_bytestart as u64))?;

            // Write the temp buffer to the file
            file.write_all(&temp_buffer)?;

            // Write the index list
            file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
            let write_index = write_block_index(&mut file, &form_index);
            if write_index.is_err() {
                return Err(write_index.err().unwrap());
            }
            
            // Mark current position as the end of the file
            let current_pos = file.seek(std::io::SeekFrom::Current(0))?;
            file.set_len(current_pos)?;

        } else {
            // ---- END DATA REMOVAL ----
            println!("> Removing form at the end of the data...");
            form_index.indexes.remove(form_index_pos);

            archive_info.form_count -= 1;
            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

            // Write the Header
            file.seek(std::io::SeekFrom::Start(0))?;
            let block_header = write_block_header(&mut file, &archive_info);
            if block_header.is_err() {
                return Err(block_header.err().unwrap());
            }

            let block_bytestart = write_block_bytestart(&mut file, archive_info.bytestart_index, archive_info.bytestart_data);
            if block_bytestart.is_err() {
                return Err(block_bytestart.err().unwrap());
            }
            
            // Write the new index list
            file.seek(std::io::SeekFrom::Start(archive_info.bytestart_index as u64))?;
            let write_index = write_block_index(&mut file, &form_index);
            if write_index.is_err() {
                return Err(write_index.err().unwrap());
            }
        }
    }

    Ok(())
}