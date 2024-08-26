pub mod test_archive;
pub mod test_types;
pub mod test_forms;

#[allow(unused)]
pub use test_archive::*;
#[allow(unused)]
pub use test_types::*;
#[allow(unused)]
pub use test_forms::*;

#[allow(unused)]
pub fn run_tests() {
    run_tests_structs();
    run_tests_io();
}

pub fn run_tests_structs() {
    test_types();
    test_forms();
    test_archive();
}

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