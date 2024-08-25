use std::{fs::File, io::{self, Seek}};

use crate::archive_tools::{io::{binary_search_for_index_item, read_archive_info}, structs::{FormBase, FormTrait}, types::*};


pub fn read_form(file_path: &str, form_id: FormID) -> io::Result<Box<dyn FormTrait>> {

    // Step 1: Read the archive info to get metadata and indexes
    let archive = read_archive_info(file_path)?;
    if archive.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    // Step 2: Open a file handle to the archive
    let mut file = File::open(file_path)?;
    
    // Step 3: Use binary search to find the index item for the FormID
    file.seek(std::io::SeekFrom::Start(archive.bytestart_index as u64))?;

    match binary_search_for_index_item(&mut file, form_id, archive.form_count)? {
        Some(index_item) => {
            let form_bytepos = index_item.data_start_offset + archive.bytestart_data;
            file.seek(std::io::SeekFrom::Start(form_bytepos as u64))?;

            let read_form = FormBase::read_from_bytes(&mut file)?;

            Ok(read_form)
        }
        None => {
            Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."))
        }
    }
}