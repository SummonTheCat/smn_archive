use std::fs::{remove_file, File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};

use crate::archive_tools::io::{get_form_exists, read_archive_info, read_block_index, write_block_bytestart, write_block_header, write_block_index};
use crate::archive_tools::io::{IOStructIndex, IOStructIndexItem};
use crate::archive_tools::structs::{Archive, FormBase, FormTrait};
use crate::archive_tools::types::FormID;

pub fn write_form(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {
    let mut archive_info = read_archive_info(file_path)?;
    let new_form_bytes = form.to_bytes();
    let form_length_new = new_form_bytes.len() as u32;

    let mut file = File::options().read(true).write(true).open(file_path)?;

    if archive_info.form_count == 0 {
        // Handle the case where the archive is empty
        let form_index_item = IOStructIndexItem {
            form_id: form.form_id(),
            form_type: form.form_type(),
            data_start_offset: 0,
        };
        let mut form_index = IOStructIndex { indexes: vec![form_index_item] };

        archive_info.form_count = 1;
        archive_info.bytestart_index += form_length_new;

        write_form_to_empty(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
    } else {
        let found_form = get_form_exists(file_path, form.form_id())?;

        if !found_form {
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
            let last_form_id = FormID::from(form_id_buf);


            // Read the type of the last form in the archive
            let mut form_type_buf = [0u8; 1];
            file.read_exact(&mut form_type_buf)?;

            // Read the offset of the last form in the archive
            let mut form_offset_buf = [0u8; 4];
            file.read_exact(&mut form_offset_buf)?;
            let last_form_offset = u32::from_be_bytes(form_offset_buf);

            // Read the last form from the archive
            file.seek(SeekFrom::Start(last_form_offset as u64 + archive_info.bytestart_data as u64 ))?;
            let last_form = FormBase::read_from_bytes(&mut file)?;
            let last_form_length = last_form.get_byte_count() as u32;

            if form.form_id().to_u16() > last_form_id.to_u16() {
                let _write_result = write_form_to_end(&mut file, &mut archive_info, &new_form_bytes, last_form_length, last_form_offset)?;
                return Ok(());
            }
        }

        file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
        let mut form_index = read_block_index(&mut file, archive_info.form_count)?;

        // Search for the form in the index using binary search
        if let Some(found_index_pos) = binary_search_for_form_index(&form_index, form.form_id()) {
            // Form found, update existing form
            let form_bytepos = form_index.indexes[found_index_pos].data_start_offset + archive_info.bytestart_data;
            file.seek(SeekFrom::Start(form_bytepos as u64))?;
            let read_form = FormBase::read_from_bytes(&mut file)?;
            // Check that the form type matches
            if read_form.form_type() != form.form_type() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Form type mismatch"));
            }

            let form_length_diff = form_length_new as i32 - read_form.get_byte_count() as i32;

            archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

            if found_index_pos == form_index.indexes.len() - 1 {
                // If the form is the last one, use the optimized path
                write_form_over_last(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            } else {
                // Update the index items after the form
                for item in form_index.indexes.iter_mut().skip(found_index_pos + 1) {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }
                write_form_over_existing(&mut file, &archive_info, &new_form_bytes, &mut form_index, found_index_pos as u32, form_length_diff)?;
            }
        } else {
            // Form not found, insert new form in the appropriate position
            let form_length_diff = form_length_new as i32;

            let index_pos = match form_index.indexes.binary_search_by(|item| item.form_id.cmp(&form.form_id())) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };

            if index_pos == 0 {
                // Insert at the beginning
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: 0,
                };
                for item in form_index.indexes.iter_mut() {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }
                form_index.indexes.insert(0, form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                write_form_to_beginning(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
            } else {
                // Insert in the middle
                let next_index_item_offset = form_index.indexes[index_pos].data_start_offset;
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: next_index_item_offset,
                };

                for item in form_index.indexes.iter_mut().skip(index_pos) {
                    item.data_start_offset = (item.data_start_offset as i32 + form_length_diff) as u32;
                }

                form_index.indexes.insert(index_pos, form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                write_form_to_inside(&mut file, &archive_info, &new_form_bytes, &mut form_index, index_pos as u32)?;
            }
        }
    }

    Ok(())
}

pub fn write_form_to_end(file: &mut File, archive_info: &mut Archive, form_bytes: &Vec<u8>, last_form_length: u32, last_form_offset: u32) -> std::io::Result<()> {

    // Write the old index to a temp file
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


    // Write the new form to the archive
    file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
    file.write_all(form_bytes)?;

    let checkpoint = file.seek(SeekFrom::Current(0))?;

    // Write the old index back to the archive
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

    // Update the archive info
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
    
    // Write the new header to the archive
    archive_info.form_count += 1;
    file.seek(SeekFrom::Start(0))?;
    write_block_header(file, archive_info)?;
    write_block_bytestart(file, archive_info.bytestart_index, archive_info.bytestart_data)?;

    // Remove the temp file
    drop(temp_file);
    remove_file(temp_file_path)?;

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

    let temp_file_path = "temp_file.tmp"; // Temporary file path
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) // Ensure the file is truncated to 0 length if it already exists
        .open(temp_file_path)?;

    // Seek to the starting point in the original file
    file.seek(SeekFrom::Start(archive.bytestart_data as u64))?;

    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }

    file.seek(SeekFrom::Start(archive.bytestart_data as u64))?;
    file.write_all(form_bytes)?;

    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}

pub fn write_form_to_inside(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex, form_index_position: u32) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    let next_form_start_pos = (form_index.indexes[form_index_position as usize + 1].data_start_offset as i32 + archive.bytestart_data as i32 - form_bytes.len() as i32) as u32;

    let temp_file_path = "temp_file.tmp"; // Temporary file path
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) // Ensure the file is truncated to 0 length if it already exists
        .open(temp_file_path)?;

    file.seek(SeekFrom::Start(next_form_start_pos as u64))?;
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }

    let form_start_pos = form_index.indexes[form_index_position as usize].data_start_offset + archive.bytestart_data;
    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    drop(temp_file);
    std::fs::remove_file(temp_file_path)?;

    Ok(())
}

pub fn write_form_over_existing(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex, form_index_position: u32, form_length_diff: i32) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    let next_form_start_pos = (form_index.indexes[form_index_position as usize + 1].data_start_offset as i32 + archive.bytestart_data as i32 - form_length_diff) as u32;

    let temp_file_path = "temp_file.tmp"; // Temporary file path
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) // Ensure the file is truncated to 0 length if it already exists
        .open(temp_file_path)?;

    file.seek(SeekFrom::Start(next_form_start_pos as u64))?;
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }

    let form_start_pos = form_index.indexes[form_index_position as usize].data_start_offset + archive.bytestart_data;
    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    drop(temp_file);
    std::fs::remove_file(temp_file_path)?;

    Ok(())
}

pub fn write_form_over_last(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;

    write_block_header(file, archive)?;
    write_block_bytestart(file, archive.bytestart_index, archive.bytestart_data)?;

    let form_start_pos = form_index.indexes.last().unwrap().data_start_offset + archive.bytestart_data;

    file.seek(SeekFrom::Start(form_start_pos as u64))?;
    file.write_all(form_bytes)?;

    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;
    write_block_index(file, form_index)?;

    file.set_len(archive.bytestart_index as u64 + (form_index.indexes.len() as u64 * 8))?;

    Ok(())
}

fn binary_search_for_form_index(
    form_index: &IOStructIndex, 
    target_form_id: FormID
) -> Option<usize> {
    let mut left = 0;
    let mut right = form_index.indexes.len();

    while left < right {
        let mid = (left + right) / 2;
        let mid_form_id = form_index.indexes[mid].form_id;

        if mid_form_id == target_form_id {
            return Some(mid);
        } else if mid_form_id < target_form_id {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}
