use std::fs::File;
use std::io::Seek;

use crate::archive_tools::structs::*;
use crate::archive_tools::io::{io_write_block::*, IOStructIndex};

pub fn io_write_archive_skeleton(path: &str, archive: &Archive) {
    println!("--- Writing Archive Skeleton ---");
    println!("Writing archive skeleton to: {}", path);
    println!("Archive Data: {:?}", archive);

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
    } else {
        println!("Successfully wrote the header block to the file.");
    }

    // Get the current position in the file
    let bytestart_pos = match file.seek(std::io::SeekFrom::Current(0)) {
        Ok(p) => p as u32,
        Err(e) => {
            eprintln!("Failed to get the current position in the file: {}", e);
            return;
        }
    };
    let bytestart_end = (bytestart_pos + 8) as u64;

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

    println!("Skeleton write complete.");
}

