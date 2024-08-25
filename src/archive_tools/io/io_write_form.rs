use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::archive_tools::io::*;
use crate::archive_tools::structs::*;
use crate::archive_tools::types::FormID;

#[allow(unused)]
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
        // Search for the form in the index using binary search
        file.seek(SeekFrom::Start(archive_info.bytestart_index as u64))?;
        let mut form_index = read_block_index(&mut file, archive_info.form_count)?;

        if let Some(found_index_pos) = binary_search_for_form_index(&form_index, form.form_id()) {
            // Form found, update existing form
            let form_bytepos = form_index.indexes[found_index_pos].data_start_offset + archive_info.bytestart_data;
            file.seek(SeekFrom::Start(form_bytepos as u64))?;
            let read_form = FormBase::read_from_bytes(&mut file)?;
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
            } else if index_pos == form_index.indexes.len() {
                // Insert at the end
                let last_index_item_offset = form_index.indexes.last().unwrap().data_start_offset;
                let last_form = read_form(file_path, form_index.indexes.last().unwrap().form_id)?;
                let last_form_length = last_form.get_byte_count() as u32;
                let form_index_item = IOStructIndexItem {
                    form_id: form.form_id(),
                    form_type: form.form_type(),
                    data_start_offset: last_index_item_offset + last_form_length,
                };
                form_index.indexes.push(form_index_item);

                archive_info.form_count += 1;
                archive_info.bytestart_index = (archive_info.bytestart_index as i32 + form_length_diff) as u32;

                write_form_to_end(&mut file, &archive_info, &new_form_bytes, &mut form_index)?;
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

pub fn write_form_to_end(file: &mut File, archive: &Archive, form_bytes: &Vec<u8>, form_index: &mut IOStructIndex) -> std::io::Result<()> {
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
