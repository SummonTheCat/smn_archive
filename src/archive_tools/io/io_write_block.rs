use std::fs::File;
use std::io;
use std::io::Write;

use crate::archive_tools::structs::Archive;
use crate::archive_tools::io::IOStructIndex;

pub fn write_block_header(file: &mut File, archive: &Archive) -> std::io::Result<()> {
    let header = archive.header_to_bytes();
    file.write_all(&header)?;
    Ok(())
}

pub fn write_block_bytestart(file: &mut File, bytestart_index: u32, bytestart_data: u32) -> std::io::Result<()> {
    let bytestart_index_bytes = bytestart_index.to_be_bytes();  
    let bytestart_data_bytes = bytestart_data.to_be_bytes();   

    file.write_all(&bytestart_index_bytes)?;  
    file.write_all(&bytestart_data_bytes)?;  
    
    Ok(())
}

// Function to write the index block
pub fn write_block_index(file: &mut File, index_block: &IOStructIndex) -> io::Result<()> {
    for index in &index_block.indexes {
        file.write_all(&index.form_id.to_bytes())?;
        file.write_all(&[index.form_type.to_byte()])?;
        let data_start_offset_u32 = index.data_start_offset as u32;
        file.write_all(&data_start_offset_u32.to_be_bytes())?;
    }

    Ok(())
}

