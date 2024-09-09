use crate::archive_tools::io::{read_archive_info, read_lite_archive, write_archive_info, write_archive_skeleton, write_form};
use crate::archive_tools::structs::{Archive, FormString, FormWorld, ArchiveID, GlobalID, LangCode, StrLrg, StrSml, Version, FormID};

#[allow(unused)]
pub fn test_archive() {
    println!("--- Testing Archive ---");

    let mut archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );

    println!("Archive: \n{:?}", archive);
    println!("Archive Header byte count: {}", archive.get_header_byte_count());
    println!("Archive Header as bytes: {:?}", archive.header_to_bytes());

    println!();
}

#[allow(unused)]
pub fn test_archive_write(file_path: &str) {
    println!("--- Testing Archive Write ---");
    println!("Writing to file: {}", file_path);

    println!("-- Creating archive --");
    let mut archive = Archive::new(
        ArchiveID::from("002"),
        Version::from(1.2),
        StrLrg::from("Test Archive"),
    );

    let mut write_result = write_archive_skeleton(file_path, &archive);
    match write_result {
        Ok(_) => println!("Archive written successfully."),
        Err(e) => println!("Error writing archive: {}", e),
    }


    println!("-- Editing Archive Info --");
    archive.version = Version::from(1.3);
    archive.description = StrLrg::from("Edited Test Archive");
    
    write_result = write_archive_info(file_path, &archive);
    match write_result {
        Ok(_) => println!("Archive Info written successfully."),
        Err(e) => println!("Error writing archive info: {}", e),
    }
}

#[allow(unused)]
pub fn test_archive_read(file_path: &str) {
    
    println!("--- Testing Archive Read ---");
    println!("-- Reading Archive Info --");
    let archive_info = read_archive_info(file_path);
    println!("Archive Info: {:?}", archive_info);
    println!("Finished reading the archive data.");

    println!("-- Reading Lite Archive --");
    // Add some forms to the archive
    let form1 = FormString::new(
        FormID::from("00001"),
        StrSml::from("String1"),
        vec![LangCode::from("EN"), LangCode::from("FR")],
        vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")]
    );

    let form2 = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("Wrld1"),
        StrSml::from("Jungle"),
        vec![GlobalID::from("00100051")]
    );

    write_form(file_path, &form1);
    write_form(file_path, &form2);

    let lite_archive = read_lite_archive(file_path);
    println!("Lite Archive: {:?}", lite_archive);

}
