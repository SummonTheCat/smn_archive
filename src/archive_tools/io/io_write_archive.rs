use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::archive_tools::structs::*;
use crate::archive_tools::io::{io_write_block::*, read_block_index, IOStructIndex};

use super::{read_block_bytestarts, read_block_header};

pub fn io_write_archive_skeleton(path: &str, archive: &Archive) {

    // Create the file
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("File creation error: {}", e);
            return;
        }
    };

    // Write the HEADER block
    if let Err(e) = write_block_header(&mut file, archive) {
        eprintln!("Failed to write header block: {}", e);
        return;
    } 

    // Get the current position in the file
    let bytestart_pos = match file.seek(std::io::SeekFrom::Current(0)) {
        Ok(p) => p as u32,
        Err(e) => {
            eprintln!("Failed to get the current position in the file: {}", e);
            return;
        }
    };
    let bytestart_end = bytestart_pos + 8;

    // Call the helper function to write the bytestart block
    if let Err(e) = write_block_bytestart(&mut file, bytestart_end, bytestart_end) {
        eprintln!("Failed to write bytestart block: {}", e);
        return;
    }

    // Write the INDEX block
    let index_block = IOStructIndex { indexes: Vec::new() };
    let result_index = write_block_index(&mut file, &index_block);
    if let Err(e) = result_index {
        eprintln!("Failed to write index block: {}", e);
    }
}

pub fn write_archive_info(file_path: &str, archive: &Archive) -> io::Result<()> {
    
    let mut file = File::options().read(true).write(true).open(file_path)?;

    // Read the header block
    let header = read_block_header(&mut file)?;
    // read the bytestart block
    let bytestart = read_block_bytestarts(&mut file)?;

    // Go to index block
    file.seek(SeekFrom::Start(bytestart.bytestart_index as u64))?;
    let index = read_block_index(&mut file, header.form_count)?;

    println!("Header: {:?}", header);
    println!("Bytestart: {:?}", bytestart);
    println!("Index: {:?}", index);

    // Calculate new description length difference
    let description_length_diff = archive.description.get_byte_count() as i32 - header.description.get_byte_count() as i32;

    // Calculate new byte start positions
    let bytestart_index_new = (bytestart.bytestart_index as i32 + description_length_diff) as u32;
    let bytestart_data_new = (bytestart.bytestart_data as i32 + description_length_diff) as u32;

    println!("New Bytestart Index: {}", bytestart_index_new);
    println!("New Bytestart Data: {}", bytestart_data_new);

    let new_archive_id = header.archive_id;
    let new_archive_version = archive.version;
    let new_arhive_description = archive.description.clone();

    let mut new_archive = Archive::new(
        new_archive_id,
        new_archive_version,
        new_arhive_description
    );
    new_archive.form_count = header.form_count;
    new_archive.bytestart_index = bytestart_index_new;
    new_archive.bytestart_data = bytestart_data_new;

    // Prepare to read the remaining file data into a temporary file
    let temp_file_path = "temp_file.tmp"; // Temporary file path
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) // Ensure the file is empty
        .open(temp_file_path)?;

    // Seek to the start of the data block
    file.seek(SeekFrom::Start(bytestart.bytestart_data as u64))?;

    // Copy the rest of the data to the temporary file in chunks
    let mut buffer = [0u8; 8192]; // 8KB buffer for reading and writing in chunks
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }

    println!("Remaining data has been copied to the temporary file.");

    // Start writing the new archive data to the original file
    file.seek(SeekFrom::Start(0))?;

    // Write the header block
    write_block_header(&mut file, &new_archive)?;

    // Write the bytestart block
    write_block_bytestart(&mut file, bytestart_index_new, bytestart_data_new)?;

    // Write the temp file data back to the original file in chunks
    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Seek to the new index block position
    file.seek(SeekFrom::Start(bytestart_index_new as u64))?;

    // Write the new index block
    let new_index_block = IOStructIndex { indexes: index.indexes };
    write_block_index(&mut file, &new_index_block)?;

    // Truncate the file to the new size
    let current_pos = file.seek(SeekFrom::Current(0))?;
    file.set_len(current_pos)?;

    // Remove the temporary file
    std::fs::remove_file(temp_file_path)?;

    Ok(())
}

