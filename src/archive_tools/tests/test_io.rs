use crate::archive_tools::types::*;
use crate::archive_tools::structs::{Archive, FormWorld};
use crate::archive_tools::io::*;

pub fn test_archive_write(file_path: &str) {
    println!("--- Testing Archive Write ---");
    println!("Writing to file: {}", file_path);

    // Create an archive
    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );

    io_write_archive_skeleton(file_path, &archive);
}

pub fn test_archive_read(file_path: &str) {
    let archive_info = read_archive_info(file_path);
    println!("Archive Info: {:?}", archive_info);

    println!("Finished reading the archive data.");
}

pub fn test_write_form(file_path: &str) {
    println!("--- Testing Form Write ---");
    println!("Writing to file: {}", file_path);

    println!("---------------------------");
    // Create a form
    let form = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );

    let write_result = write_form(file_path, &form);
    println!("Write Result: {:?}", write_result);

    println!("---------------------------");
    let read_form1 = read_form(file_path, FormID::from("00002"));
    if read_form1.is_err() {
        println!("Error reading form: {:?}", read_form1.err());
    }
}