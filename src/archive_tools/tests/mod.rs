pub mod test_archive;
pub mod test_types;
pub mod test_forms;

#[allow(unused)]
pub use test_archive::*;
#[allow(unused)]
pub use test_types::*;
#[allow(unused)]
pub use test_forms::*;

use super::{io::{ delete_form, get_form_exists, read_form, write_archive_skeleton, write_form}, structs::{Archive, FormString}, types::{ArchiveID, FormID, LangCode, StrLrg, StrSml, Version}};

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

    // Overwrite the form
    let form = FormString::new(
        FormID::from(3),
        StrSml::from("OWForm"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("STRING"), StrLrg::from("NEW")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form overwritten successfully");
        },
        Err(e) => {
            println!("Error overwriting form: {}", e);
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

    // overwrite form 00001
    let form = FormString::new(
        FormID::from(1),
        StrSml::from("Form00001"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("ST"), StrLrg::from("NE")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00001 overwritten successfully");
        },
        Err(e) => {
            println!("Error overwriting form 00001: {}", e);
        }
    }

    // Read form 00001
    let read = read_form(path, FormID::from(1));
    match read {
        Ok(form) => {
            println!("Form 00001 read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form 00001: {}", e);
        }
    }

    // Add a form 00008
    let form = FormString::new(
        FormID::from(8),
        StrSml::from("Form00008"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("ST"), StrLrg::from("NE")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00008 written successfully");
        },
        Err(e) => {
            println!("Error writing form 00008: {}", e);
        }
    }

    // Overwrite form 00008
    let form = FormString::new(
        FormID::from(8),
        StrSml::from("Form00008"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("STaDadwadwd"), StrLrg::from("NEAwdawdwd")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00008 overwritten successfully");
        },
        Err(e) => {
            println!("Error overwriting form 00008: {}", e);
        }
    }

    // Read form 00008
    let read = read_form(path, FormID::from(8));
    match read {
        Ok(form) => {
            println!("Form 00008 read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form 00008: {}", e);
        }
    }

    // Read form 00001
    let read = read_form(path, FormID::from(1));
    match read {
        Ok(form) => {
            println!("Form 00001 read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form 00001: {}", e);
        }
    }

    // Read form 00003
    let read = read_form(path, FormID::from(3));
    match read {
        Ok(form) => {
            println!("Form 00003 read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form 00003: {}", e);
        }
    }

    //Write form 00005
    let form = FormString::new(
        FormID::from(5),
        StrSml::from("Form00005"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("STall"), StrLrg::from("NEed")],
    );
    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00005 written successfully");
        },
        Err(e) => {
            println!("Error writing form 00005: {}", e);
        }
    }

    // Read form 00005
    let read = read_form(path, FormID::from(5));
    match read {
        Ok(form) => {
            println!("Form 00005 read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form 00005: {}", e);
        }
    }

    // Overwrite form 00005
    let form = FormString::new(
        FormID::from(5),
        StrSml::from("Fo05"),
        vec![LangCode::EN, LangCode::ES],
        vec![StrLrg::from("STawdawdall"), StrLrg::from("NEeAdawdd")],
    );

    let write_result = write_form(path, &form);
    match write_result {
        Ok(_) => {
            println!("Form 00005 overwritten successfully");
        },
        Err(e) => {
            println!("Error overwriting form 00005: {}", e);
        }
    }

    // Delete form 00005
    let delete_result = delete_form(&path, FormID::from(5));
    match delete_result {
        Ok(_) => {
            println!("Form 00005 deleted successfully");
        },
        Err(e) => {
            println!("Error deleting form 00005: {}", e);
        }
    }


    // Check if form 00005 was deleted
    let check_exists = get_form_exists(&path, FormID::from(5));
    match check_exists {
        Ok(exists) => {
            println!("Form 00005 exists: {}", exists);
        },
        Err(e) => {
            println!("Error checking if form 00005 exists: {}", e);
        }
    }

    // Delete form 00001
    let delete_result = delete_form(&path, FormID::from(1));
    match delete_result {
        Ok(_) => {
            println!("Form 00001 deleted successfully");
        },
        Err(e) => {
            println!("Error deleting form 00001: {}", e);
        }
    }

    // Check if form 00001 was deleted
    let check_exists = get_form_exists(&path, FormID::from(1));
    match check_exists {
        Ok(exists) => {
            println!("Form 00001 exists: {}", exists);
        },
        Err(e) => {
            println!("Error checking if form 00001 exists: {}", e);
        }
    }

    // Delete form 00008
    let delete_result = delete_form(&path, FormID::from(8));
    match delete_result {
        Ok(_) => {
            println!("Form 00008 deleted successfully");
        },
        Err(e) => {
            println!("Error deleting form 00008: {}", e);
        }
    }

    // Check if form 00008 was deleted
    let check_exists = get_form_exists(&path, FormID::from(8));
    match check_exists {
        Ok(exists) => {
            println!("Form 00008 exists: {}", exists);
        },
        Err(e) => {
            println!("Error checking if form 00008 exists: {}", e);
        }
    }

}