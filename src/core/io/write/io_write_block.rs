use std::fs::File;
use std::io;
use std::io::Write;

use crate::core::structs::Archive;
use crate::core::io::IOStructIndex;

/// Writes the HEADER block to the specified file using Little-Endian byte order.
pub fn write_block_header(file: &mut File, archive: &Archive) -> io::Result<()> {
    let header = archive.header_to_bytes();
    file.write_all(&header)?;
    Ok(())
}

/// Writes the BYTESTART block to the specified file using Little-Endian byte order.
pub fn write_block_bytestart(file: &mut File, bytestart_index: u32, bytestart_data: u32) -> io::Result<()> {
    // Convert `bytestart_index` and `bytestart_data` to Little-Endian byte arrays.
    let bytestart_index_bytes = bytestart_index.to_le_bytes();  
    let bytestart_data_bytes = bytestart_data.to_le_bytes();   

    // Write the Little-Endian bytes to the file.
    file.write_all(&bytestart_index_bytes)?;  
    file.write_all(&bytestart_data_bytes)?;  
    
    Ok(())
}

/// Writes the INDEX block to the specified file using Little-Endian byte order.
pub fn write_block_index(file: &mut File, index_block: &IOStructIndex) -> io::Result<()> {
    for index in &index_block.indexes {
        // Write FormID as bytes (assumed to be in Little-Endian).
        file.write_all(&index.form_id.to_bytes())?;
        
        // Write FormType as a single byte (endianess irrelevant for single bytes).
        file.write_all(&[index.form_type.to_byte()])?;
        
        // Convert `data_start_offset` to Little-Endian bytes before writing.
        let data_start_offset_u32 = index.data_start_offset as u32;
        file.write_all(&data_start_offset_u32.to_le_bytes())?;
    }

    Ok(())
}
