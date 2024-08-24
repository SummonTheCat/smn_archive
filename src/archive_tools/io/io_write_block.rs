use std::fs::File;
use std::io;
use std::io::Write;

use crate::archive_tools::structs::*;
use  crate::archive_tools::io::*;

pub fn write_block_header(file: &mut File, archive: &Archive) -> std::io::Result<()> {
    let header = archive.header_to_bytes();
    file.write_all(&header)?;
    Ok(())
}

pub fn write_block_bytestart(file: &mut File, bytestart_index: u64, bytestart_data: u64) -> std::io::Result<()> {
    // Convert the bytestart values to bytes
    let bytestart_index_bytes = bytestart_index.to_be_bytes();
    let bytestart_data_bytes = bytestart_data.to_be_bytes();

    // Write the 4-byte bytestart_index value twice (using the last 4 bytes)
    file.write_all(&bytestart_index_bytes[4..])?;
    file.write_all(&bytestart_data_bytes[4..])?;
    
    Ok(())
}

// Function to write the index block
pub fn write_block_index(file: &mut File, index_block: &IOStructIndex) -> io::Result<()> {
    // Iterate over each index item in the IOStructIndex
    for index in &index_block.indexes {
        // Write the FormID to the file
        file.write_all(&index.form_id.to_bytes())?;

        // Write the FormType to the file
        file.write_all(&[index.form_type.to_byte()])?;

        // Convert the data start offset (u16) to a 4-byte u32
        let data_start_offset_u32 = index.data_start_offset as u32;

        // Write the 4-byte data start offset to the file
        file.write_all(&data_start_offset_u32.to_be_bytes())?;
    }

    Ok(())
}

