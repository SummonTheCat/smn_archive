use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use rand::Rng;

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

pub fn test_many_forms(form_count: u16) {
    println!("--- Performance Test: Writing {} Forms ---", form_count);
    
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let archive_path = current_dir.join("archives").join("test_io.smn");
    println!("Archive Path: {:?}", archive_path);

    let file_path = archive_path.to_str().unwrap();

    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );

    write_archive_skeleton(file_path, &archive);
    
    let mut rng = rand::thread_rng();

    let mut tenth_percent_times = Vec::new();
    let mut last_checkpoint = Instant::now();

    println!("-- Started Writing Forms --");
    let write_start = Instant::now();
    
    for i in 1..=form_count {
        let form_id = FormID::from(i as u16);

        let form_name = format!("Wrld{}", i);
        let form_description = format!("Description{}", i);
        let form_map = format!("Map{}", i);

        let world_parts_count = rng.gen_range(1..5);
        let mut world_parts = Vec::new();
        for _ in 0..world_parts_count {
            let random_archive_id = ArchiveID::from(rng.gen_range(1..=200));
            let random_form_id = FormID::from(rng.gen_range(1..=10000));
            let random_global_id = GlobalID::from((random_archive_id, random_form_id));
            world_parts.push(GlobalID::from(random_global_id));
        }

        let form = FormWorld::new(
            form_id,
            StrSml::from(form_name.as_str()),
            StrSml::from(form_description.as_str()),
            StrSml::from(form_map.as_str()),
            world_parts,
        );

        let write_result = write_form(file_path, &form);
        if write_result.is_err() {
            println!("Error writing form {}: {:?}", i, write_result.err());
        } else {
            // Update the progress bar
            let progress = (i as f32 / form_count as f32) * 100.0;
            print!("\rSuccessfully wrote FormID: {} Progress: [{:<50}] {:.2}%", i, "=".repeat((progress / 2.0) as usize), progress);
            std::io::stdout().flush().unwrap();
            println!();
        }

        // Record the time every 10%
        if i as f32 % (form_count as f32 / 10.0) == 0.0 {
            let now = Instant::now();
            let segment_time = now.duration_since(last_checkpoint);
            tenth_percent_times.push(segment_time);
            last_checkpoint = now;
        }
    }

    let write_duration = write_start.elapsed();
    println!("-- Finished Writing Forms --");
    println!("Time taken to write {} forms: {:?}", form_count, write_duration);

    println!("-- Started Reading Forms --");
    let read_start = Instant::now();

    for i in 1..=form_count {
        let form_id_str = format!("{:05}", i);
        let form_id = FormID::from(form_id_str.as_str());

        let read_form = read_form(file_path, form_id);
        match read_form {
            Ok(_) => println!("Successfully read FormID: {}", form_id_str),
            Err(e) => println!("Error reading FormID {}: {:?}", form_id_str, e),
        }
    }

    let read_duration = read_start.elapsed();
    println!("-- Finished Reading Forms --");
    println!("Time taken to read {} forms: {:?}", form_count, read_duration);

    println!("-- Performance Test Completed --");
    println!("Breakdown:");
    println!("Write Duration: {:?}", write_duration);
    println!("Read Duration: {:?}", read_duration);
    println!("Total Duration: {:?}", write_duration + read_duration);
    
    // Print the time taken for each 10%
    println!("-- Write Time per 10% --");
    for (i, time) in tenth_percent_times.iter().enumerate() {
        println!("Time to write {}%: {:?}", (i + 1) * 10, time);
    }

}