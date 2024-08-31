use std::{fs::{remove_file, File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}};
use crate::archive_tools::{io::{binary_search_for_index_item_and_position, get_index_form_id_last, get_new_form_index_pos, IOStructIndex, IOStructIndexItem}, structs::{Archive, FormBase, FormTrait}, types::FormID};
use crate::archive_tools::io::{get_form_exists, get_index_form_id_first, read_block_bytestarts, read_block_header, write_block_bytestart, write_block_header, write_block_index};

/// Writes or updates a form in the specified archive file.
///
/// This function handles adding a form to an archive, updating an existing form, or inserting
/// the form in the appropriate position if the form ID does not already exist.
///
/// # Parameters
/// - `file_path`: The path to the archive file.
/// - `form`: A reference to a struct implementing the `FormTrait`, representing the form to write.
///
/// # Returns
/// - `io::Result<()>`: Ok on success, or an error on failure.
///
/// For more details, see the documentation in `io_write_archive.rs`.
pub fn write_form(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {
    let form_exists = get_form_exists(file_path, form.form_id())?;
    if form_exists {
        // Check if its the last form
        write_form_existing(file_path, form)?;
    } else {
        write_form_new(file_path, form)?;
    }

    Ok(())
}

fn write_form_existing(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {
    let mut file = File::options().read(true).write(true).open(file_path)?;

    // Read the header
    let header = read_block_header(&mut file)?;
    // Read the byte starts
    let byte_starts = read_block_bytestarts(&mut file)?;

    let mut archive_info = Archive::new(header.archive_id, header.version, header.description);
    archive_info.form_count = header.form_count;
    archive_info.bytestart_index = byte_starts.bytestart_index;
    archive_info.bytestart_data = byte_starts.bytestart_data;

    // Get the last form data
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

    let last_form_id = get_index_form_id_last(&mut file, archive_info.form_count)?;
    let mut last_form_type_buf = [0u8; 1];
    file.read_exact(&mut last_form_type_buf)?;

    let mut last_form_data_start_offset_buf = [0u8; 4];
    file.read_exact(&mut last_form_data_start_offset_buf)?;
    let last_form_data_start_offset = u32::from_be_bytes(last_form_data_start_offset_buf);

    if last_form_id == form.form_id() {
        // Overwrite the last form
        write_form_existing_last(&mut file, form, &mut archive_info, last_form_data_start_offset)?;
    } else {
        // Overwrite a form inside the archive
        write_form_existing_inner(&mut file, form, &mut archive_info)?;
    }

    Ok(())
}

fn write_form_existing_inner(file: &mut File, form: &dyn FormTrait, archive_info: &mut Archive) -> std::io::Result<()> {

    // Find the offset of the form to overwrite
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
    let old_form_index_item = binary_search_for_index_item_and_position(file, form.form_id(), archive_info.form_count)?;
    
    if old_form_index_item.is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Form not found in archive."));
    } else {
        let (index_position, old_form_index_item) = old_form_index_item.unwrap();


        // Read the old form data
        file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + old_form_index_item.data_start_offset as u64))?;
        let old_form = FormBase::read_from_bytes(file)?;
        let old_form_length = old_form.get_byte_count() as u32;


        // New form data
        let new_form_bytes = form.to_bytes();
        let new_form_length = new_form_bytes.len() as u32;

        // Calculate the difference in length
        let form_length_diff = new_form_length as i32 - old_form_length as i32;

        // Read the index to a temp file
        let temp_index_path = "temp_index.tmp";
        let mut temp_index_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(temp_index_path)?;

        // Seek to the start of the index
        file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

        // Iterate through the index items
        for i in 0..archive_info.form_count {
            let mut form_id_buf = [0u8; 2];
            file.read_exact(&mut form_id_buf)?;
            let _form_id = FormID::from(form_id_buf);

            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;

            let mut data_start_offset_buf = [0u8; 4];
            file.read_exact(&mut data_start_offset_buf)?;
            let mut data_start_offset = u32::from_be_bytes(data_start_offset_buf);


            if i as u64 > index_position {
                // Apply the offset change to the data start offset
                data_start_offset = (data_start_offset as i32 + form_length_diff) as u32;
            }

            // Write the index item to the temp file
            temp_index_file.write_all(&form_id_buf)?;
            temp_index_file.write_all(&form_type_buf)?;
            temp_index_file.write_all(&data_start_offset.to_be_bytes())?;
        }

        // Go to the start of the data changes
        file.seek(SeekFrom::Start(
            archive_info.bytestart_data as u64 
            + old_form_index_item.data_start_offset as u64 
            + old_form_length as u64
        ))?;
        
        // Open the temporary file to write the remaining data
        let temp_data_path = "temp_data.tmp";
        let mut temp_data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(temp_data_path)?;
        
        // Calculate the number of bytes to read (from the end of the old form to the start of the index)
        // Mark the current position (right after the old form)
        let current_position = file.stream_position()?;

        // Seek to the end of the file to calculate the total length
        file.seek(SeekFrom::End(0))?;
        let end_of_file = file.stream_position()?;

        // Calculate the length from the current position to the end of the file
        let length_to_end = end_of_file - current_position;

        // Calculate the length of the index block (each index item is 7 bytes)
        let index_block_length = archive_info.form_count as u64 * 7;

        // Calculate the number of bytes to read (data block only)
        let bytes_to_read = length_to_end - index_block_length;

        // Seek back to the position right after the old form
        file.seek(SeekFrom::Start(current_position))?;

        // Buffer to read in chunks
        let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks

        // Read the remaining data into the temp file, but only up to the start of the index
        let mut bytes_left = bytes_to_read;
        while bytes_left > 0 {
            let to_read = buffer.len().min(bytes_left as usize);
            let bytes_read = file.read(&mut buffer[..to_read])?;
            if bytes_read == 0 {
                break; // End of file or reached the index start
            }
            temp_data_file.write_all(&buffer[..bytes_read])?;
            bytes_left -= bytes_read as u64;
        }

        // Write the new form data to the archive
        file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + old_form_index_item.data_start_offset as u64))?;
        file.write_all(&new_form_bytes)?;

        // Write the remaining data back to the archive from the temp data file
        temp_data_file.seek(SeekFrom::Start(0))?; // Rewind temp file to start
        while let Ok(bytes_read) = temp_data_file.read(&mut buffer) {
            if bytes_read == 0 {
                break; // End of file reached
            }
            file.write_all(&buffer[..bytes_read])?;
        }


        // Write the updated index back to the archive
        temp_index_file.seek(SeekFrom::Start(0))?; // Rewind temp index file to start
        file.seek(SeekFrom::Start((archive_info.bytestart_index as i32 + form_length_diff)as u64 ))?;
        while let Ok(bytes_read) = temp_index_file.read(&mut buffer) {
            if bytes_read == 0 {
                break; // End of file reached
            }
            file.write_all(&buffer[..bytes_read])?;
        }

        // Trim the file to the new length
        archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;
        file.set_len(archive_info.bytestart_index as u64 + archive_info.form_count as u64 * 7)?;

        // Write the new header to the archive
        file.seek(SeekFrom::Start(0))?; // Seek to start of file
        write_block_header(file, archive_info)?; // Write the header
        write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?; // Write the byte start

        // Remove the temp files
        drop(temp_data_file);
        drop(temp_index_file);
        remove_file(temp_data_path)?;
        remove_file(temp_index_path)?;
    }

    Ok(())
}


fn write_form_existing_last(file: &mut File, form: &dyn FormTrait, archive_info: &mut Archive, old_data_start_offset: u32) -> std::io::Result<()> {

    // Read old form data
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + old_data_start_offset as u64))?;
    let old_form = FormBase::read_from_bytes(file)?;
    let old_form_length = old_form.get_byte_count() as u32;

    // New form data
    let new_form_bytes = form.to_bytes();
    let new_form_length = new_form_bytes.len() as u32;

    // Calculate the difference in length
    let form_length_diff = new_form_length as i32 - old_form_length as i32;

    // Read the index to a temp file
    let temp_index_path = "temp_index.tmp";
    let mut temp_index_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_index_path)?;

    // Seek to the start of the index
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

    // Read the index to the temp file till the end of the file in chunks
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_index_file.write_all(&buffer[..bytes_read])?;
    }

    // Write the new form data to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + old_data_start_offset as u64))?;
    file.write_all(&new_form_bytes)?;

    // Write the old index back to the archive
    temp_index_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_index_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Update the archive info
    archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

    // Trim the file to the new length
    file.set_len(archive_info.bytestart_index as u64 + archive_info.form_count as u64 * 7)?;

    // Write the new header to the archive
    file.seek(SeekFrom::Start(0))?;
    write_block_header(file, archive_info)?;

    // Write the new byte start to the archive
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?;

    // Remove the temp file
    drop(temp_index_file);
    remove_file(temp_index_path)?;

    Ok(())
}

fn write_form_new(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {
    let mut file = File::options().read(true).write(true).open(file_path)?;
    
    // Read the header
    let header = read_block_header(&mut file)?;
    // Read the byte starts
    let byte_starts = read_block_bytestarts(&mut file)?;

    let mut archive_info = Archive::new(header.archive_id, header.version, header.description);
    archive_info.form_count = header.form_count;
    archive_info.bytestart_index = byte_starts.bytestart_index;
    archive_info.bytestart_data = byte_starts.bytestart_data;

    // Convert the form to bytes
    let form_bytes = form.to_bytes();

    // if the form count is 0, send to write_form_new_first
    if header.form_count == 0 {
        write_form_new_first(&mut file, &form_bytes, form, &mut archive_info)?;
    } else {
        // Seek to the index
        file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
        
        // Read the first form id
        let first_form_id = get_index_form_id_first(&mut file)?;
        // If the form id is less than the first form id, write the form to the beginning
        if form.form_id() < first_form_id {
            write_form_new_start(&mut file, &form_bytes, form, &mut archive_info)?;
        } else {
            // Get the last form in the index's form id
            file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
            let last_form_id = get_index_form_id_last(&mut file, archive_info.form_count)?;
            // If the form id is greater than the last form id, write the form to the end
            if form.form_id() > last_form_id {
                write_form_new_end(&mut file, &form_bytes, &mut archive_info)?;
            } else {
                // Write the form in the middle of the archive

                // Find the position to insert the form
                file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
                let form_insert_position = get_new_form_index_pos(&mut file, archive_info.form_count, form.form_id())?;

                // Get the form insert position and item
                if let Some((position, index_item)) = form_insert_position {
                    // Write the form to the archive
                    write_form_new_mid(&mut file, &form_bytes, position, index_item, &mut archive_info)?;
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Form not found in archive."));
                }
            }
        }
    }
    
    Ok(())
}

fn write_form_new_mid(file: &mut File, form_bytes: &Vec<u8>, form_insert_position: u64, prev_form_item: IOStructIndexItem, archive_info: &mut Archive) -> std::io::Result<()> {

    // Seek to the start of file
    file.seek(SeekFrom::Start(0))?;

    // Read the header
    let _header = read_block_header(file)?;

    // Read the byte starts
    let _byte_starts = read_block_bytestarts(file)?;

    // Read the index to a temp file
    let temp_index_path = "temp_index.tmp";
    let mut temp_index_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_index_path)?;

    // Seek to the start of the index
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

    // Read the index to the temp file per index item
    let mut passed_insert_position = false; // Changed to mutable
    let mut new_form_data_start_offset = 0u32;


    for i in 0..archive_info.form_count {
        let mut form_id_buf = [0u8; 2];
        file.read_exact(&mut form_id_buf)?;
        let _form_id = FormID::from(form_id_buf);

        let mut form_type_buf = [0u8; 1];
        file.read_exact(&mut form_type_buf)?;

        let mut data_start_offset_buf = [0u8; 4];
        file.read_exact(&mut data_start_offset_buf)?;
        let mut data_start_offset = u32::from_be_bytes(data_start_offset_buf);

        if i as u64 == form_insert_position {
            passed_insert_position = true;

            // Write the new index item to the temp file
            let new_form_id = form_bytes[0..2].to_vec();
            let new_form_type = form_bytes[2];
            new_form_data_start_offset = prev_form_item.data_start_offset;

            temp_index_file.write_all(&new_form_id)?;
            temp_index_file.write_all(&[new_form_type])?;
            temp_index_file.write_all(&new_form_data_start_offset.to_be_bytes())?;

        }

        if passed_insert_position {
            // Update the data start offset
            data_start_offset += form_bytes.len() as u32;
        }

    
        // Write the index item to the temp file
        temp_index_file.write_all(&form_id_buf)?;
        temp_index_file.write_all(&form_type_buf)?;
        temp_index_file.write_all(&data_start_offset.to_be_bytes())?;
    }

    // Debug: Read all bytes from the temp index file and print as bytes
    temp_index_file.seek(SeekFrom::Start(0))?; // Rewind to the start of the temp index file
    let mut temp_index_buffer = Vec::new();
    temp_index_file.read_to_end(&mut temp_index_buffer)?;

    // Seek to the start of the data changes
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + new_form_data_start_offset as u64))?;
    // Open the temporary file to write the remaining data
    let temp_data_path = "temp_data.tmp";
    let mut temp_data_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(temp_data_path)?;

    // Buffer to read in chunks
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    let mut bytes_left = archive_info.bytestart_index - (new_form_data_start_offset + archive_info.bytestart_data);

    // Read from the current position till the start of the index
    while bytes_left > 0 {
        let to_read = buffer.len().min(bytes_left as usize);
        let bytes_read = file.read(&mut buffer[..to_read])?;
        if bytes_read == 0 {
            break; // End of file or reached the index start
        }
        temp_data_file.write_all(&buffer[..bytes_read])?;
        bytes_left -= bytes_read as u32;
    }

    // Debug: Read all bytes from the temp data file and print as bytes
    temp_data_file.seek(SeekFrom::Start(0))?; // Rewind to the start of the temp data file
    let mut temp_data_buffer = Vec::new();
    temp_data_file.read_to_end(&mut temp_data_buffer)?;

    // Write the new form data to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64 + new_form_data_start_offset as u64))?;
    file.write_all(form_bytes)?;

    // Write the remaining data back to the archive from the temp data file
    temp_data_file.seek(SeekFrom::Start(0))?; // Rewind temp file to start
    while let Ok(bytes_read) = temp_data_file.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file reached
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Write the updated index back to the archive
    temp_index_file.seek(SeekFrom::Start(0))?; // Rewind temp index file to start
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64 + form_bytes.len() as u64))?;
    while let Ok(bytes_read) = temp_index_file.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file reached
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Update the archive info
    archive_info.form_count += 1;
    archive_info.bytestart_index += form_bytes.len() as u32;

    // Write the new header to the archive
    file.seek(SeekFrom::Start(0))?; // Seek to start of file
    write_block_header(file, archive_info)?; // Write the header
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?; // Write the byte start

    // Remove the temp files
    drop(temp_data_file);
    drop(temp_index_file);
    remove_file(temp_data_path)?;
    remove_file(temp_index_path)?;


    Ok(())
}

fn write_form_new_end(file: &mut File, form_bytes: &Vec<u8>, archive_info: &mut Archive) -> std::io::Result<()> {
    // Step 1: Get the last form's offset and length
    // Read the ID of the last form in the archive
    let form_count_u64 = archive_info.form_count as u64;

    // Calculate the offset safely using checked arithmetic
    let index_offset = form_count_u64
        .checked_mul(7) // Multiply form_count by 7
        .and_then(|res| res.checked_sub(7)) // Subtract 7 from the result
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Overflow in index calculation"))?;

    // Calculate the seek position by adding the offset to bytestart_index
    let seek_position = (archive_info.bytestart_index as u64)
        .checked_add(index_offset)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Overflow in seek position calculation"))?;

    // Seek to the calculated position
    file.seek(SeekFrom::Start(seek_position))?;

    // Read the form ID
    let mut form_id_buf = [0u8; 2];
    file.read_exact(&mut form_id_buf)?;
    let _last_form_id = FormID::from(form_id_buf);

    // Read the type of the last form in the archive
    let mut form_type_buf = [0u8; 1];
    file.read_exact(&mut form_type_buf)?;

    // Read the offset of the last form in the archive
    let mut form_offset_buf = [0u8; 4];
    file.read_exact(&mut form_offset_buf)?;
    let last_form_offset = u32::from_be_bytes(form_offset_buf);

    // Read the last form from the archive
    file.seek(SeekFrom::Start(last_form_offset as u64 + archive_info.bytestart_data as u64 ))?;
    let last_form = FormBase::read_from_bytes(file)?;
    let last_form_length = last_form.get_byte_count() as u32;

    // Step 2: Write the old index to a temp file
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
    let temp_file_path = "temp_file.tmp"; // Temporary file path
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) // Ensure the file is truncated to 0 length if it already exists
        .open(temp_file_path)?;

    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }   

    // Step 3: Write the new form to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
    file.write_all(form_bytes)?;

    let checkpoint = file.seek(SeekFrom::Current(0))?;

    // Step 4: Write the old index back to the archive
    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Seek back to the checkpoint
    file.seek(SeekFrom::Start(checkpoint))?;
    // Seek to the end of the index (7 bytes per form x form_count)
    file.seek(SeekFrom::Current(archive_info.form_count as i64 * 7))?;

    // Step 5: Update the archive info
    let form_length_diff = form_bytes.len() as i32;
    archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

    // Write the new index item to the end of the index
    let new_form_offset = last_form_offset + last_form_length;
    
    // write the new form id to the end of the index
    let form_id_bytes = form_bytes[0..2].to_vec();
    file.write_all(&form_id_bytes)?;

    let form_type_byte = form_bytes[2];
    file.write_all(&[form_type_byte])?;

    let new_form_offset_bytes = new_form_offset.to_be_bytes();
    file.write_all(&new_form_offset_bytes)?;
    
    // Step 6: Write the new header to the archive
    archive_info.form_count += 1;
    file.seek(SeekFrom::Start(0))?;
    write_block_header(file, archive_info)?;
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?;

    // Step 7: Remove the temp file
    drop(temp_file);
    remove_file(temp_file_path)?;

    Ok(())
}


fn write_form_new_start(file: &mut File, form_bytes: &Vec<u8>, form: &dyn FormTrait, archive_info: &mut Archive) -> std::io::Result<()> {

    let form_bytes_len = form_bytes.len() as u32;
    let data_block_length = archive_info.bytestart_index - archive_info.bytestart_data;
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64))?;

    // Read the data block to a temporary file
    let temp_data_path = "temp_data.tmp";
    let mut temp_data_file = File::options().read(true).write(true).create(true).open(temp_data_path)?;

    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    let mut bytes_left = data_block_length;

    // Read only the length of the data block
    while bytes_left > 0 {
        let to_read = buffer.len().min(bytes_left as usize);
        let bytes_read = file.read(&mut buffer[..to_read])?;
        if bytes_read == 0 {
            break; // End of file reached before expected, break loop
        }
        temp_data_file.write_all(&buffer[..bytes_read])?;
        bytes_left -= bytes_read as u32;
    }

    // Read the index block to a temporary file
    let temp_index_path = "temp_index.tmp";
    let mut temp_index_file = File::options().read(true).write(true).create(true).open(temp_index_path)?;

    // Seek to the start of the index block
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

    // Read and modify each index item, then write it to the temp file
    for _ in 0..archive_info.form_count {
        // Read 2 bytes for form_id
        let mut form_id_buf = [0u8; 2];
        file.read_exact(&mut form_id_buf)?;

        // Read 1 byte for form_type
        let mut form_type_buf = [0u8; 1];
        file.read_exact(&mut form_type_buf)?;

        // Read 4 bytes for data_start_offset and add form_bytes_len to it
        let mut offset_buf = [0u8; 4];
        file.read_exact(&mut offset_buf)?;
        let data_start_offset = u32::from_be_bytes(offset_buf) + form_bytes_len;

        // Write the modified index item to the temp index file
        temp_index_file.write_all(&form_id_buf)?;
        temp_index_file.write_all(&form_type_buf)?;
        temp_index_file.write_all(&data_start_offset.to_be_bytes())?;
    }

    // Update the archive info
    archive_info.bytestart_index += form_bytes_len;
    archive_info.form_count += 1;

    // Seek back to the start of the file
    file.seek(SeekFrom::Start(0))?;

    // Write the header
    write_block_header(file, archive_info)?;
    
    // Write the byte start
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?;

    // Write the new form data to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64))?;
    file.write_all(form_bytes)?;

    // Write the old data block back to the archive
    temp_data_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_data_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Write the new index to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;

    let form_index_item = IOStructIndexItem {
        form_id: form.form_id(),
        form_type: form.form_type(),
        data_start_offset: 0,
    };
    let form_index = IOStructIndex { indexes: vec![form_index_item] };
    write_block_index(file, &form_index)?;

    // Write the old index back to the archive
    temp_index_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_index_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Remove the temp files
    drop(temp_data_file);
    drop(temp_index_file);
    std::fs::remove_file(temp_data_path)?;
    std::fs::remove_file(temp_index_path)?;

    Ok(())
}

fn write_form_new_first(file: &mut File, form_bytes: &Vec<u8>, form: &dyn FormTrait, archive_info: &mut Archive) -> std::io::Result<()> {

    archive_info.form_count += 1;
    let form_bytes_len = form_bytes.len() as u32;
    archive_info.bytestart_index = archive_info.bytestart_index + form_bytes_len;

    file.seek(SeekFrom::Start(0))?;    
    write_block_header(file, archive_info)?;
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?;

    file.seek(SeekFrom::Start(archive_info.bytestart_data as u64))?;
    file.write_all(form_bytes)?;

    let form_index_item = IOStructIndexItem {
        form_id: form.form_id(),
        form_type: form.form_type(),
        data_start_offset: 0,
    };
    let form_index = IOStructIndex { indexes: vec![form_index_item] };

    write_block_index(file, &form_index)?;

    Ok(())
}


 #[cfg(test)]
mod tests {
    use super::*;
    use crate::archive_tools::io::{read_form, write_archive_skeleton};
    use crate::archive_tools::structs::FormString;
    use crate::archive_tools::types::{ArchiveID, FormID, LangCode, StrLrg, StrSml, Version};
    use std::io::Result;

    #[test]
    fn test_write_form_new_first() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_add_form_to_empty_archive.bin";

        // Create an empty archive
        let archive = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Empty Archive")
        );
        write_archive_skeleton(path, &archive)?;

        // Create a form to add to the empty archive
        let form = FormString::new(
            FormID::from(1),
            StrSml::from("Test Form"),
            vec![LangCode::EN, LangCode::FR],
            vec![StrLrg::from("Test String"), StrLrg::from("Chaîne de test")]
        );

        // Call the function to add the form to the archive
        let result = write_form(path, &form);
        assert!(result.is_ok(), "Failed to write form to the empty archive.");

        // Read the form back to verify it was written correctly
        let form_exists = get_form_exists(path, form.form_id())?;
        assert!(form_exists, "The form should exist in the archive.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_write_form_new_start() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_twfns.bin";

        let archive_info = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Test Archive")
        );
        let write_result = write_archive_skeleton(path, &archive_info);
        match write_result {
            Ok(_) => {
                println!("Archive skeleton written successfully");
            },
            Err(e) => {
                println!("Error writing archive skeleton: {}", e);
            }
        }
    
        // Test creating a new form, adding it to the archive, and reading it
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form1 = FormString::new(form_id, form_name, form_langs, form_strings);
    
        let form_write_result = write_form(path, &form1);
        match form_write_result {
            Ok(_) => {
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }
    
        // Create a new form and add it to the start of the archive
        let form_id = FormID::from(1);
        let form_name = StrSml::from("Test Form at Start");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form2 = FormString::new(form_id, form_name, form_langs, form_strings);
    
        let form_write_result = write_form(path, &form2);
        match form_write_result {
            Ok(_) => {
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }
    
        // Read the first form in the archive
        let read = read_form(path, FormID::from(1));
        let form = read.unwrap();

        assert_eq!(form.form_id(), FormID::from(1), "The form ID should be 1.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_write_form_new_end() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_twfne.bin";

        let archive_info = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Test Archive")
        );
        let write_result = write_archive_skeleton(path, &archive_info);
        match write_result {
            Ok(_) => {
                println!("Archive skeleton written successfully");
            },
            Err(e) => {
                println!("Error writing archive skeleton: {}", e);
            }
        }
    
        // Test creating a new form, adding it to the archive, and reading it
        let form_id = FormID::from(1);
        let form_name = StrSml::from("Test Form");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form1 = FormString::new(form_id, form_name, form_langs, form_strings);
    
        let form_write_result = write_form(path, &form1);
        match form_write_result {
            Ok(_) => {
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }
    
        // Create a new form and add it to the end of the archive
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form at END");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form2 = FormString::new(form_id, form_name, form_langs, form_strings);
    
        let form_write_result = write_form(path, &form2);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Read the Last form in the archive
        let read = read_form(path, FormID::from(5));
        let form = read.unwrap();

        assert_eq!(form.form_id(), FormID::from(5), "The form ID should be 5.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_write_form_new_mid() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_twfnm.bin";

        let archive_info = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Test Archive")
        );

        let write_result = write_archive_skeleton(path, &archive_info);
        match write_result {
            Ok(_) => {
                println!("Archive skeleton written successfully");
            },
            Err(e) => {
                println!("Error writing archive skeleton: {}", e);
            }
        }

        // Test creating a new form, adding it to the archive, and reading it
        let form_id = FormID::from(1);
        let form_name = StrSml::from("Test Form");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form1 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form1);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Create a new form and add it to the end of the archive
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form at END");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form2 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form2);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Create a new form and add it to the middle of the archive
        let form_id = FormID::from(3);
        let form_name = StrSml::from("Test Form at MID");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form3 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form3);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Read the Middle form in the archive
        let read = read_form(path, FormID::from(3));
        let form = read.unwrap();

        assert_eq!(form.form_id(), FormID::from(3), "The form ID should be 3.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;

        Ok(())

    }

    #[test]
    fn test_write_form_exist_last() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_write_form_exist_last.bin";

        let archive_info = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Test Archive")
        );

        let write_result = write_archive_skeleton(path, &archive_info);
        match write_result {
            Ok(_) => {
                println!("Archive skeleton written successfully");
            },
            Err(e) => {
                println!("Error writing archive skeleton: {}", e);
            }
        }

        // Test creating a new form, adding it to the archive, and reading it
        let form_id = FormID::from(1);
        let form_name = StrSml::from("Test Form");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form1 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form1);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Create a new form and add it to the end of the archive
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form at END");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form2 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form2);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Write over the last form in the archive
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form at END Overwrite");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form3 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form3);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Read the Last form in the archive
        let read = read_form(path, FormID::from(5));
        let form = read.unwrap();

        assert_eq!(form.form_id(), FormID::from(5), "The form ID should be 5.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;

        Ok(())
    }

    #[test]
    fn test_write_form_exist_inner() -> Result<()> {
        // Setup: Create a temporary file path and an empty Archive instance
        let path = "test_write_form_exist_inner.bin";

        let archive_info = Archive::new(
            ArchiveID::from(1),
            Version::from(1.0),
            StrLrg::from("Test Archive")
        );

        let write_result = write_archive_skeleton(path, &archive_info);
        match write_result {
            Ok(_) => {
                println!("Archive skeleton written successfully");
            },
            Err(e) => {
                println!("Error writing archive skeleton: {}", e);
            }
        }

        // Test creating a new form, adding it to the archive, and reading it
        let form_id = FormID::from(1);
        let form_name = StrSml::from("Test Form");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form1 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form1);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Create a new form and add it to the end of the archive
        let form_id = FormID::from(5);
        let form_name = StrSml::from("Test Form at END");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form2 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form2);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Create a new form and add it to the middle of the archive
        let form_id = FormID::from(3);
        let form_name = StrSml::from("Test Form at MID");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form3 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form3);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Write over the middle form in the archive
        let form_id = FormID::from(3);
        let form_name = StrSml::from("Test Form at MID Overwrite");
        let form_langs = vec![LangCode::EN, LangCode::FR];
        let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
        let form4 = FormString::new(form_id, form_name, form_langs, form_strings);

        let form_write_result = write_form(path, &form4);
        match form_write_result {
            Ok(_) => {
                println!("Form written successfully");
            },
            Err(e) => {
                println!("Error writing form: {}", e);
            }
        }

        // Read the Middle form in the archive
        let read = read_form(path, FormID::from(3));
        let form = read.unwrap();
        
        assert_eq!(form.form_id(), FormID::from(3), "The form ID should be 3.");

        // Clean up: Delete the temporary file
        std::fs::remove_file(path)?;
        Ok(())
    }
}
