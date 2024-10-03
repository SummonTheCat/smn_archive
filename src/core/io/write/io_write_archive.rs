use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::core::structs::Archive;
use crate::core::io::{write_block_header, write_block_bytestart, write_block_index, read_block_bytestarts, read_block_header, read_block_index};
use crate::core::io::IOStructIndex;

/// Writes the archive skeleton to the specified file.
pub fn write_archive_skeleton(path: &str, archive: &Archive) -> io::Result<()> {
    // Attempt to create the file at the given path
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e), 
    };

    // Write the HEADER block to the file from the provided archive data
    if let Err(e) = write_block_header(&mut file, archive) {
        eprintln!("Failed to write header block: {}", e);
        return Err(e); 
    } 

    // Get the current position in the file for the BYTESTART block
    let bytestart_pos = match file.seek(SeekFrom::Current(0)) {
        Ok(p) => p as u32,
        Err(e) => {
            eprintln!("Failed to get the current position in the file: {}", e);
            return Err(e);
        }
    };

    let bytestart_end = bytestart_pos + 8; 

    // Write the BYTESTART block using the calculated start and end positions
    if let Err(e) = write_block_bytestart(&mut file, bytestart_end, bytestart_end) {
        eprintln!("Failed to write bytestart block: {}", e);
        return Err(e); 
    }

    // Prepare an empty INDEX block structure and write it to the file
    let index_block = IOStructIndex { indexes: Vec::new() };
    if let Err(e) = write_block_index(&mut file, &index_block) {
        eprintln!("Failed to write index block: {}", e);
        return Err(e); 
    }
    Ok(())
}


/// Updates the archive information in the specified file.
pub fn write_archive_info(file_path: &str, archive: &Archive) -> io::Result<()> {

    // Open the file for reading and writing
    let mut file = File::options().read(true).write(true).open(file_path)?;

    // Read the existing HEADER and BYTESTART blocks from the file
    let header = read_block_header(&mut file)?;
    let bytestart = read_block_bytestarts(&mut file)?;

    // Read the INDEX block based on the bytestart index
    file.seek(SeekFrom::Start(bytestart.bytestart_index as u64))?;
    let index = read_block_index(&mut file, header.form_count)?;

    // Calculate the difference in description length
    let description_length_diff = archive.description.get_byte_count() as i32 
        - header.description.get_byte_count() as i32;

    // Calculate the new byte start positions based on the description length difference
    let bytestart_index_new = (bytestart.bytestart_index as i32 + description_length_diff) as u32;
    let bytestart_data_new = (bytestart.bytestart_data as i32 + description_length_diff) as u32;

    // Create a new Archive with updated fields
    let mut new_archive = Archive::new(
        archive.archive_id,
        archive.version,
        archive.description.clone(),
    );
    new_archive.form_count = header.form_count;
    new_archive.bytestart_index = bytestart_index_new;
    new_archive.bytestart_data = bytestart_data_new;

    // Prepare a temporary file to store remaining file data
    let temp_file_path = "temp_file.tmp"; 
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) 
        .open(temp_file_path)?;

    // Copy the data following the BYTESTART block to the temporary file
    file.seek(SeekFrom::Start(bytestart.bytestart_data as u64))?;
    let mut buffer = [0u8; 8192]; 
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        temp_file.write_all(&buffer[..bytes_read])?;
    }

    // Start writing the updated archive data to the original file
    file.seek(SeekFrom::Start(0))?;

    // Write the archive info
    write_block_header(&mut file, &new_archive)?;
    write_block_bytestart(&mut file, bytestart_index_new, bytestart_data_new)?;

    // Write the data from the temporary file back to the original file
    temp_file.seek(SeekFrom::Start(0))?;
    loop {
        let bytes_read = temp_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }

    // Write the updated INDEX block
    file.seek(SeekFrom::Start(bytestart_index_new as u64))?;
    let new_index_block = IOStructIndex { indexes: index.indexes };
    write_block_index(&mut file, &new_index_block)?;

    // Truncate the file to remove any excess data beyond the new end
    let current_pos = file.seek(SeekFrom::Current(0))?;
    file.set_len(current_pos)?;

    // Remove the temporary file used during the process
    std::fs::remove_file(temp_file_path)?;

    Ok(())
}
