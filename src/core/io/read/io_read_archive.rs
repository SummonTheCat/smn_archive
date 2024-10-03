use std::fs::File;
use std::io::{self, Seek, SeekFrom};

use crate::core::io::{read_block_bytestarts, read_block_header, binary_search_for_index_item};
use crate::core::structs::*;

use super::{read_block_index, read_form};

/// Reads the archive information from a file, including header and bytestart data.
pub fn read_archive_info(file_path: &str) -> io::Result<Archive> {
    let mut file = File::open(file_path)?;  
    let header = read_block_header(&mut file)?; 
    let bytestarts = read_block_bytestarts(&mut file)?; 

    // Populate the Archive structure with the read data
    let mut archive_out = Archive::new(header.archive_id, header.version, header.description);
    archive_out.form_count = header.form_count;
    archive_out.bytestart_index = bytestarts.bytestart_index;
    archive_out.bytestart_data = bytestarts.bytestart_data;

    Ok(archive_out)
}

/// Reads a lightweight version of the archive, including only the header and index data.
pub fn read_lite_archive(file_path: &str) -> io::Result<LiteArchive> {
    let mut file = File::open(file_path)?; 

    let header = read_block_header(&mut file)?; 
    let bytestarts = read_block_bytestarts(&mut file)?; 

    // Populate the LiteArchive structure with header info
    let mut archive_out = LiteArchive {
        archive_id: header.archive_id,
        version: header.version,
        description: header.description,
        form_count: header.form_count,
        archive_items: Vec::new(),
    };

    // Move to the index section in the file
    file.seek(SeekFrom::Start(bytestarts.bytestart_index as u64))?;
    let index = read_block_index(&mut file, header.form_count)?; 

    // Iterate through index items to populate LiteArchive
    for index_item in index.indexes {
        let form_id = index_item.form_id;
        let form_type = index_item.form_type;

        // Attempt to read the form by its ID
        let index_form = read_form(file_path, form_id);
        let form_name = match index_form {
            Ok(form) => form.form_name(),
            Err(_) => StrSml::from("Form not found"),
        };

        // Create and add a LiteArchiveItem for each form in the archive
        let new_lite_form = LiteArchiveItem {
            form_id,
            form_name,
            form_type,
        };
        archive_out.archive_items.push(new_lite_form);
    }

    Ok(archive_out)
}

/// Checks if a form with a specific `form_id` exists within the archive.
pub fn get_form_exists(file_path: &str, form_id: FormID) -> io::Result<bool> {
    let archive = read_archive_info(file_path)?;  
    if archive.form_count == 0 {
        return Ok(false); 
    }

    let mut file = File::open(file_path)?; 
    file.seek(SeekFrom::Start(archive.bytestart_index as u64))?;  

    // Perform a binary search to find the form by its ID
    match binary_search_for_index_item(&mut file, form_id, archive.form_count)? {
        Some(_) => Ok(true),  // Form found
        None => Ok(false),    // Form not found
    }
}
