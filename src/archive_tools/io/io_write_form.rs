use std::fs::File;

use crate::archive_tools::io::*;
use crate::archive_tools::structs::*;

pub fn write_form(file_path: &str, form: &dyn FormTrait) -> std::io::Result<()> {
    println!("--- Writing Form ---");

    // Set up working variables
    #[allow(unused)]
    let mut new_form_bytes: Vec<u8> = Vec::new();
    #[allow(unused)]
    let mut archive_info: Archive;

    // -- READ ARCHIVE INFO --
    archive_info = read_archive_info(file_path)?;

    new_form_bytes = form.to_bytes();

    let mut _file = File::options()
        .read(true)
        .write(true)
        .open(file_path)?;

    println!("--- Debug ---");
    // Write working variable states
    println!("Archive Info: {:?}", archive_info);
    println!("NEW FORM BYTES: {:?}", new_form_bytes);

    Ok(())
}

