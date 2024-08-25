use std::fs::File;
use std::io::{self, Seek};

use crate::archive_tools::io::{read_block_bytestarts, read_block_header};
use crate::archive_tools::structs::*;
use crate::archive_tools::types::{FormID, StrSml};

use super::{read_block_index, read_form};

pub fn read_archive_info(file_path: &str) -> io::Result<Archive> {

    let mut file = File::open(file_path)?;

    let header = read_block_header(&mut file)?;
    let bytestarts = read_block_bytestarts(&mut file)?;

    let mut archive_out = Archive::new(header.archive_id, header.version, header.description);
    archive_out.form_count = header.form_count;
    archive_out.bytestart_index = bytestarts.bytestart_index;
    archive_out.bytestart_data = bytestarts.bytestart_data;

    Ok(archive_out)
}

pub fn read_lite_archive(file_path: &str) -> io::Result<LiteArchive> {

    // Open the file for reading
    let mut file = File::open(file_path)?;

    let header = read_block_header(&mut file)?;
    let bytestarts = read_block_bytestarts(&mut file)?;

    let mut archive_out = LiteArchive{
        archive_id: header.archive_id,
        version: header.version,
        description: header.description,
        form_count: header.form_count,
        archive_items: Vec::new(),
    };

    file.seek(std::io::SeekFrom::Start(bytestarts.bytestart_index as u64))?;
    let index = read_block_index(&mut file, header.form_count)?;
    
    for index_item in index.indexes {
        let form_id = index_item.form_id;
        let form_type = index_item.form_type;

        let index_form = read_form(file_path, form_id);
        let form_name = match index_form {
            Ok(form) => form.form_name(),
            Err(_) => StrSml::from("Form not found"),
        };

        let new_lite_form = LiteArchiveItem{
            form_id,
            form_name,
            form_type,
        };
        archive_out.archive_items.push(new_lite_form);
    }
    Ok(archive_out)
}

pub fn get_form_exists(file_path: &str, target_form_id: FormID) -> io::Result<bool> {
    match read_form(file_path, target_form_id) {
        Ok(_) => Ok(true),  
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)  
            } else {
                Err(e)  
            }
        }
    }
}
