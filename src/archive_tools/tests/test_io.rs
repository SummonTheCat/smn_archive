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
        StrSml::from("Wrld2"),
        StrSml::from("Desert"),
        vec![GlobalID::from("00100050")]
    );

    let write_result = write_form(file_path, &form);
    println!("Write Result: {:?}", write_result);

    println!("---------------------------");
    let read_form1 = read_form(file_path, FormID::from("00002"));
    if read_form1.is_err() {
        println!("Error reading form: {:?}", read_form1.err());
    }

    println!("---------------------------");

    let form2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("Jungle"),
        vec![GlobalID::from("00100051")]
    );

    let write_result2 = write_form(file_path, &form2);
    println!("Write Result: {:?}", write_result2);

    println!("---------------------------");
    
    let form3 = FormWorld::new(
        FormID::from("00004"),
        StrSml::from("Wrld4"),
        StrSml::from("Tundra"),
        vec![GlobalID::from("00100052")]
    );

    let write_result3 = write_form(file_path, &form3);
    println!("Write Result: {:?}", write_result3);

    println!("---------------------------");
    let read_form2 = read_form(file_path, FormID::from("00004"));
    if read_form2.is_err() {
        println!("Error reading form: {:?}", read_form2.err());
    }

    println!("---------------------------");
    let form4 = FormWorld::new(
        FormID::from("00003"),
        StrSml::from("Wrld3"),
        StrSml::from("Forest"),
        vec![GlobalID::from("00100053")]
    );

    let write_result4 = write_form(file_path, &form4);
    println!("Write Result: {:?}", write_result4);

    println!("---------------------------");
    let read_form3 = read_form(file_path, FormID::from("00002"));
    if read_form3.is_err() {
        println!("Error reading form: {:?}", read_form3.err());
    }

    
}