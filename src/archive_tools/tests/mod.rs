pub mod test_archive;
pub mod test_types;
pub mod test_forms;

#[allow(unused)]
pub use test_archive::*;
#[allow(unused)]
pub use test_types::*;
#[allow(unused)]
pub use test_forms::*;

use super::{io::{ read_archive_info, read_form, write_archive_info, write_archive_skeleton, write_form}, structs::{Archive, FormRefGroup, FormString, ArchiveID, FormID, GlobalID, LangCode, StrLrg, StrSml, Version}};

#[allow(unused)]
pub fn run_tests() {
    //run_tests_structs();
    //run_tests_io();
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
    test_form_perf(&path, 5000);
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

    // Update the archive info
    let archive_info = Archive::new(
        ArchiveID::from(1),
        Version::from(1.1),
        StrLrg::from("Test Archive Updated")
    );

    let write_result = write_archive_info(&path, &archive_info);
    match write_result {
        Ok(_) => {
            println!("Archive info written successfully");
        },
        Err(e) => {
            println!("Error writing archive info: {}", e);
        }
    }
    
    // Read archive info
    let read = read_archive_info(&path);
    match read {
        Ok(info) => {
            println!("Archive info read successfully: {:?}", info);
        },
        Err(e) => {
            println!("Error reading archive info: {}", e);
        }
    }



    // Test creating a new form, adding it to the archive, and reading it
    let form = FormString::new(
        FormID::from(3),
        StrSml::from("TestForm"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("TestField1"), StrLrg::from("TestField2")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    

    // Read the form
    let read = read_form(path, FormID::from(3));
    match read {
        Ok(form) => {
            println!("Form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form: {}", e);
        }
    }

    // Write a new form 00001
    let form = FormString::new(
        FormID::from(1),
        StrSml::from("Test Form 00001"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("Test Field 1"), StrLrg::from("Test Field 2")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00001 written successfully");
        },
        Err(e) => {
            println!("Error writing form 00001: {}", e);
        }
    }

    //Write a new RefGroup form
    let form = FormRefGroup::new(
        FormID::from(2),
        StrSml::from("RefWorlds"),
        vec![GlobalID::from("00100005"), GlobalID::from("00100006")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("RefGroup form written successfully");
        },
        Err(e) => {
            println!("Error writing RefGroup form: {}", e);
        }
    }

    // Read the form
    let read = read_form(path, FormID::from(2));
    match read {
        Ok(form) => {
            println!("RefGroup form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading RefGroup form: {}", e);
        }
    }
}