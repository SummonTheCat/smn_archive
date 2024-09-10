use std::env;
use std::path::PathBuf;

use crate::core::io::*;
use crate::core::structs::*;

pub fn test_io() {
    println!("------ TESTING IO ------");
    // Get the current directory where the application is run from
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let archive_path = current_dir.join("archives").join("test_io.smn");
    println!("Archive Path: {:?}", archive_path);

    test_io_archive(archive_path.to_str().unwrap());
    test_io_form(archive_path.to_str().unwrap());
}

fn test_io_archive(path: &str) {
    println!("------ Testing Archive IO ------");

    // Write an archive
    let skeleton_archive = Archive::new_empty();

    let result = write_archive_skeleton(path, &skeleton_archive);
    match result {
        Ok(_) => {
            println!("Archive written successfully");
        },
        Err(e) => {
            println!("Error writing archive: {:?}", e);
        }
    }

    // Read the archive
    let result = read_archive_info(path);
    match result {
        Ok(archive) => {
            println!("Archive read successfully: {:?}", archive);

            // Edit the archive
            let mut edited_archive = archive;
            edited_archive.description = StrLrg::from("An edited archive");
            edited_archive.version = Version::from(0.2);

            let result = write_archive_info(path, &edited_archive);
            match result {
                Ok(_) => {
                    println!("Archive edited successfully");
                },
                Err(e) => {
                    println!("Error editing archive: {:?}", e);
                }
            }

        },
        Err(e) => {
            println!("Error reading archive: {:?}", e);
        }
    }

    // Read the edited archive
    let result = read_archive_info(path);
    match result {
        Ok(archive) => {
            println!("Edited Archive read successfully: {:?}", archive);
        },
        Err(e) => {
            println!("Error reading edited archive: {:?}", e);
        }
    }
    
}

fn test_io_form(path: &str) {
    println!("------ Testing Form IO ------");
    // Write a form
    let form1 = FormString::new(
        FormID::from(5),
        StrSml::from("StrWelcome"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("Welcome"), StrLrg::from("Bienvenue")]
    );

    let result = write_form(path, &form1);
    match result {
        Ok(_) => {
            println!("Form written successfully");
        },
        Err(e) => {
            println!("Error writing form: {:?}", e);
        }
    }

    // Read the form
    let result = read_form(path, FormID::from(5));
    match result {
        Ok(form) => {
            println!("Form read successfully: {:?}", form);
        },
        Err(e) => {
            println!("Error reading form: {:?}", e);
        }
    }

    // Edit the form
    let edited_form = FormString::new(
        FormID::from(5),
        StrSml::from("StrWelcomeEdited"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("WELCOME"), StrLrg::from("BIENVENUE")]
    );

    let result = write_form(path, &edited_form);
    match result {
        Ok(_) => {
            println!("Form edited successfully");
        },
        Err(e) => {
            println!("Error editing form: {:?}", e);
        }
    }

    // Read the Lite Archive
    let result = read_lite_archive(path);
    match result {
        Ok(archive) => {
            println!("Lite Archive read successfully: {:?}", archive);
        },
        Err(e) => {
            println!("Error reading lite archive: {:?}", e);
        }
    }

    // Delete the form
    let result = delete_form(path, FormID::from(5));
    match result {
        Ok(_) => {
            println!("Form deleted successfully");
        },
        Err(e) => {
            println!("Error deleting form: {:?}", e);
        }
    }

}
