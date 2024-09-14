use std::{fs::File, io::{self, Seek}};

use crate::core::io::{binary_search_for_index_item, read_archive_info};
use crate::core::structs::{forms::*, types::*};

pub fn read_form(file_path: &str, form_id: FormID) -> io::Result<Box<dyn FormTrait>> {

    let archive = read_archive_info(file_path)?;
    if archive.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    let mut file = File::open(file_path)?;
    
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

pub fn read_forms(file_path: &str, form_ids: Vec<FormID>) -> io::Result<Vec<Box<dyn FormTrait>>> {
    let archive = read_archive_info(file_path)?;
    if archive.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    let mut file = File::open(file_path)?;

    let mut form_byte_starts = Vec::new();
    for form_id in form_ids {
        file.seek(std::io::SeekFrom::Start(archive.bytestart_index as u64))?;

        match binary_search_for_index_item(&mut file, form_id, archive.form_count)? {
            Some(index_item) => {
                let form_bytepos = index_item.data_start_offset + archive.bytestart_data;
                form_byte_starts.push(form_bytepos);
            }
            None => {
                return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
            }
        }
    }

    let mut forms = Vec::new();
    for form_bytepos in form_byte_starts {
        file.seek(std::io::SeekFrom::Start(form_bytepos as u64))?;
        let read_form = FormBase::read_from_bytes(&mut file)?;
        forms.push(read_form);
    }

    Ok(forms)
}

/*
#[allow(unused)]
pub fn read_forms(file_path: &str, form_ids: Vec<FormID>) -> io::Result<Vec<Box<dyn FormTrait>>> {
    let archive = read_archive_info(file_path)?;
    if archive.form_count == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Form not found in archive."));
    }

    let mut file = File::open(file_path)?;

    
    file.seek(std::io::SeekFrom::Start(archive.bytestart_index as u64))?;
    // Read the form index block to an IOStructIndex struct
    let index = read_block_index(&mut file, archive.form_count)?;

    let mut forms = Vec::new();

    match binary_search_for_index_item_inmem(form_ids, &index)? {
        Some(form_index_items) => {
            for form_index_item in form_index_items.indexes {
                let form_bytepos = form_index_item.data_start_offset + archive.bytestart_data;
                file.seek(std::io::SeekFrom::Start(form_bytepos as u64))?;
                let read_form = FormBase::read_from_bytes(&mut file)?;
                forms.push(read_form);
            }
        }
        None => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Forms not found in archive."));
        }
    }

    Ok(forms)
}
 */
