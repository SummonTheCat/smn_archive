use std::fs::File;
use std::io;

use crate::archive_tools::io::{read_block_bytestarts, read_block_header};
use crate::archive_tools::structs::*;

pub fn read_archive_info(file_path: &str) -> io::Result<Archive> {

    // Open the file for reading
    let mut file = File::open(file_path)?;

    // Extract the header using the read_block_header function
    let header = read_block_header(&mut file)?;

    // Extract he ByteStarts for the Index and Data blocks
    let bytestarts = read_block_bytestarts(&mut file)?;

    // Create the Archive struct with the extracted header data
    let mut archive_out = Archive::new(header.archive_id, header.version, header.description);
    archive_out.form_count = header.form_count;
    archive_out.bytestart_index = bytestarts.bytestart_index;
    archive_out.bytestart_data = bytestarts.bytestart_data;

    Ok(archive_out)
}

