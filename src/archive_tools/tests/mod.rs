pub mod test_archive;
pub mod test_types;
pub mod test_forms;

#[allow(unused)]
pub use test_archive::*;
#[allow(unused)]
pub use test_types::*;
#[allow(unused)]
pub use test_forms::*;

use super::{io:: write_archive_skeleton, structs::Archive, types::{ArchiveID, StrLrg, Version}};

#[allow(unused)]
pub fn run_tests() {
    run_tests_structs();
    run_tests_io();
    run_tests_flow();
}

#[allow(unused)]
pub fn run_tests_structs() {
    test_types();
    test_forms();
    test_archive();
}

#[allow(unused)]
pub fn run_tests_io() {
    let path = "./archives/test_archive.smn";
    // -- Testing all io functions --
    // All archive writing functionality (Creating and overwriting archvives and their data)
    test_archive_write(path);
    // All archive reading functionality (Reading lite archive and archive info)
    test_archive_read(path);
    // All form writing functionality (Creating, overwriting, and deleting forms)
    test_form_write(path);
    // All form reading functionality (Reading)
    test_form_read(path);
    // Test performance of reading and writing
    test_form_perf(&path, 1000);
}

pub fn run_tests_flow() {
    // -- General flow tests --
    // Test creating a new archive, adding a form, and reading the form
    let path = "./archives/test_archive.smn";

    let archive_info = Archive::new(
    ArchiveID::from(1),
    Version::from(1.0),
    StrLrg::from("Test Archive")
    );
    let write_result = write_archive_skeleton(path, &archive_info);
    match write_result {
        Ok(_) => {
            println!("Archive skeleton written successfully");
        },
        Err(e) => {
            println!("Error writing archive skeleton: {}", e);
        }
    }


}