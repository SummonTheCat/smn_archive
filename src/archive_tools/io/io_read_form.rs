use std::{fs::File, io::{self, Seek}};

use crate::archive_tools::{io::{read_archive_info, read_block_index}, structs::{FormBase, FormTrait, FormWorld}, types::*};


pub fn read_form(file_path: &str, form_id: FormID) -> io::Result<Box<dyn FormTrait>> {
    println!("--- Reading Form ---");
    println!("Reading form with FormID: {:?}", form_id);

    // Step 1: Read the archive info to get metadata and indexes
    let archive = read_archive_info(file_path)?;
    println!("Archive Info: {:?}", archive);

    // Step 2: Open a file handle to the archive
    let mut file = File::open(file_path)?;
    
    // Step 3: Read the index for the FormID
    file.seek(std::io::SeekFrom::Start(archive.bytestart_index as u64))?;
    let bytepos_index_start = archive.bytestart_index;
    let bytepos_index_end = (archive.form_count * 8) as u32 + bytepos_index_start;
    println!("Searching Index Block Range: {} - {}...", bytepos_index_start, bytepos_index_end);

    // Will be binary search, for now linear search
    let search_target = form_id;

    let index_list = read_block_index(&mut file, archive.form_count)?;

    // Search list for the target
    for index_item in index_list.indexes.iter() {
        if index_item.form_id == search_target {
            println!("Found FormID: {:?}", index_item.form_id.to_string());
            let form_bytepos = index_item.data_start_offset + archive.bytestart_data;
            file.seek(std::io::SeekFrom::Start(form_bytepos as u64))?;

            let read_form = FormBase::read_from_bytes(&mut file)?;

            return Ok(read_form);
        }
    }

    println!("Index List: {:?}", index_list);

    // Temp form
    let form: Box<dyn FormTrait> = Box::new(FormWorld::new(
        FormID::from("00002"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    ));
    return Ok(form);
}