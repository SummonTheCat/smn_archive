pub mod test_archive;
pub mod test_types;
pub mod test_forms;

#[allow(unused)]
pub use test_archive::*;
#[allow(unused)]
pub use test_types::*;
#[allow(unused)]
pub use test_forms::*;

use super::{io::{ read_form, write_archive_skeleton, write_form}, structs::{Archive, FormString}, types::{ArchiveID, FormID, LangCode, StrLrg, StrSml, Version}};

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

    // Test creating a new form, adding it to the archive, and reading it
    let form_id = FormID::from(5);
    let form_name = StrSml::from("Test Form");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form1 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form1);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Create a new form and add it to the start of the archive
    let form_id = FormID::from(1);
    let form_name = StrSml::from("Test Form at Start");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form2 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form2);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Write a new form at the end of the archive
    let form_id = FormID::from(10);
    let form_name = StrSml::from("Test Form at End");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form3 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form3);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Read the new form
    let form_read_result = read_form(path, form_id);
    match form_read_result {
        Ok(form) => {
            println!("Form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form: {}", e);
        }
    }

    // Write new form with id 7
    let form_id = FormID::from(7);
    let form_name = StrSml::from("Test Form at 7");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form4 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form4);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Read the new form
    let form_read_result = read_form(path, FormID::from(7));
    match form_read_result {
        Ok(_form) => {
            println!("Form read successfully: {:?}", FormID::from(7));
        },
        Err(e) => {
            println!("Error reading form: {}", e);
        }
    }

    // Overwrite the last form
    let form_id = FormID::from(10);
    let form_name = StrSml::from("Test Form at End Overwritten");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form5 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form5);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Read the new form
    let form_read_result = read_form(path, FormID::from(10));
    match form_read_result {
        Ok(form) => {
            println!("Form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form: {}", e);
        }
    }

    // Overwrite the middle form
    let form_id = FormID::from(5);
    let form_name = StrSml::from("Test Form at Middle Overwritten");
    let form_langs = vec![LangCode::EN, LangCode::FR];
    let form_strings = vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")];
    let form6 = FormString::new(form_id, form_name, form_langs, form_strings);

    let form_write_result = write_form(path, &form6);
    match form_write_result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {}", e);
        }
    }

    // Read the new form
    let form_read_result = read_form(path, FormID::from(5));
    match form_read_result {
        Ok(form) => {
            println!("Form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form: {}", e);
        }
    }

    test_form_perf(path, 500);


}