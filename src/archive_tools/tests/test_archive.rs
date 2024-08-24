use crate::archive_tools::types::*;
use crate::archive_tools::structs::Archive;

#[allow(unused)]
pub fn test_archive() {
    println!("--- Testing Archive ---");

    // - Archive - //
    println!("-- Archive --");

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

