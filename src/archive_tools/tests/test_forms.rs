use std::{io::Write, time::Instant};

use rand::Rng;

use crate::archive_tools::{io::{delete_form, get_form_exists, read_form, read_lite_archive, write_archive_skeleton, write_form}, structs::*, types::*};

#[allow(unused)]
pub fn test_forms() {
    println!("--- Testing Forms ---");

    // - FormBase - //
    println!("-- FormBase --");

    let form_base_1 = FormBase {
        form_id: FormID::from("00001"),
        form_type: FormType::STRING,
        form_name: StrSml::from("StrTest1"),
    };

    println!("FormBase 1: \n{:?}", form_base_1.to_string());
    println!("FormBase 1 byte count: {}", form_base_1.get_byte_count());

    println!("FormBase 1 as bytes: {:?}", form_base_1.to_bytes());

    println!();

    // - FormString - //
    println!("-- FormString --");

    let form_string_1 = FormString::new(
        FormID::from("00002"),
        StrSml::from("StrTest2"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")]
    );

    println!("FormString 1: \n{:?}", form_string_1.to_string());
    println!("FormString 1 byte count: {}", form_string_1.get_byte_count());

    println!("FormString 1 as bytes: {:?}", form_string_1.to_bytes());

    println!();

    // - FormWorld - //
    println!("-- FormWorld --");

    let form_world_1 = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );

    println!("FormWorld 1: \n{:?}", form_world_1.to_string());
    println!("FormWorld 1 byte count: {}", form_world_1.get_byte_count());
    println!("FormWorld 1 as bytes: {:?}", form_world_1.to_bytes());

    println!();
}


pub fn test_form_write(file_path: &str) {
    println!("--- Testing Form Write ---");

    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );
    let _ = write_archive_skeleton(file_path, &archive);

    // 1. Adding a form to an empty archive
    println!("-- Adding a forms to archive --");
    let form_1 = FormString::new(
        FormID::from("00005"),
        StrSml::from("StrTest1"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")]
    );
    let write_result = write_form(file_path, &form_1);
    match write_result {
        Ok(_) => println!("Form 1 written successfully {:?}.", form_1.form_id()),
        Err(e) => println!("Error writing form 1: {}", e),
    }

    // 2. Adding a form to the start of the archive
    let form_2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );
    let write_result = write_form(file_path, &form_2);
    match write_result {
        Ok(_) => println!("Form 2 written successfully {:?}.", form_2.form_id()),
        Err(e) => println!("Error writing form 2: {}", e),
    }

    // 3. Adding a form inside the archive
    let form_3 = FormWorld::new(
        FormID::from("00003"),
        StrSml::from("WrldJungle"),
        StrSml::from("Green Wastes"),
        vec![GlobalID::from("00100053"), GlobalID::from("00100055")]
    );
    let write_result = write_form(file_path, &form_3);
    match write_result {
        Ok(_) => println!("Form 3 written successfully {:?}.", form_3.form_id()),
        Err(e) => println!("Error writing form 3: {}", e),
    }

    // 4. Adding a form to the end of the archive
    let form_4 = FormWorld::new(
        FormID::from("00008"),
        StrSml::from("WrldMountain"),
        StrSml::from("High Peaks"),
        vec![GlobalID::from("00100056"), GlobalID::from("00100057")]
    );
    let write_result = write_form(file_path, &form_4);
    match write_result {
        Ok(_) => println!("Form 4 written successfully {:?}.", form_4.form_id()),
        Err(e) => println!("Error writing form 4: {}", e),
    }

    // 5. Overwriting a form at the start of the archive
    println!("-- Overwriting forms in archive --");
    let form_2b = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("WrldDesert"),
        StrSml::from("Derakiin Desert"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );
    let write_result = write_form(file_path, &form_2b);
    match write_result {
        Ok(_) => println!("Form 2 overwritten succsessfully {:?}.", form_2b.form_id()),
        Err(e) => println!("Error overwriting form 2: {}", e),
    }

    // 6. Overwriting a form at the end of the archive
    let form_4b = FormWorld::new(
        FormID::from("00008"),
        StrSml::from("WrldMountain"),
        StrSml::from("High Peaks of Derak"),
        vec![GlobalID::from("00100056"), GlobalID::from("00100057")]
    );
    let write_result = write_form(file_path, &form_4b);
    match write_result {
        Ok(_) => println!("Form 4 overwritten succsessfully {:?}.", form_4b.form_id()),
        Err(e) => println!("Error overwriting form 4: {}", e),
    }

    // 7. Overwriting a form inside the archive
    let form_3b = FormWorld::new(
        FormID::from("00003"),
        StrSml::from("WrldJungle"),
        StrSml::from("Green Wastes of Derak"),
        vec![GlobalID::from("00100053"), GlobalID::from("00100055")]
    );
    let write_result = write_form(file_path, &form_3b);
    match write_result {
        Ok(_) => println!("Form 3 overwritten succsessfully {:?}.", form_3b.form_id()),
        Err(e) => println!("Error overwriting form 3: {}", e),
    }

    // 8. Check if form exists
    let form_exists = get_form_exists(file_path, FormID::from("00001"));
    match form_exists {
        Ok(exists) => println!("Form 00001 exists: {}", exists),
        Err(e) => println!("Error checking if form 00001 exists: {}", e),
    }
    
    // 9. Deleting a form at the start of the archive  
    println!("-- Deleting forms in archive --");
    let delete_result = delete_form(file_path, form_2b.form_id());
    match delete_result {
        Ok(_) => println!("Form 2 deleted succsessfully {:?}.", form_2b.form_id()),
        Err(e) => println!("Error deleting form 2: {}", e),
    }

    // 10. Deleting a form inside the archive
    let delete_result = delete_form(file_path, form_3b.form_id());
    match delete_result {
        Ok(_) => println!("Form 3 deleted succsessfully {:?}.", form_3b.form_id()),
        Err(e) => println!("Error deleting form 3: {}", e),
    }

    // 11. Deleting a form at the end of the archive
    let delete_result = delete_form(file_path, form_4b.form_id());
    match delete_result {
        Ok(_) => println!("Form 4 deleted succsessfully {:?}.", form_4b.form_id()),
        Err(e) => println!("Error deleting form 4: {}", e),
    }

    // 12. Deleting the only form in the archive
    let delete_result = delete_form(file_path, form_1.form_id());
    match delete_result {
        Ok(_) => println!("Form 1 deleted succsessfully {:?}.", form_1.form_id()),
        Err(e) => println!("Error deleting form 1: {}", e),
    }
    let lite_archive = read_lite_archive(file_path);
    println!("Lite Archive: {:?}", lite_archive);

    
}

pub fn test_form_read(file_path: &str) {
    println!("--- Testing Form Read ---");

    // Write the archive and some test forms
    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );
    let _ = write_archive_skeleton(file_path, &archive);

    // 1. Adding a form to an empty archive
    println!("-- Adding a forms to archive --");
    let form_1 = FormString::new(
        FormID::from("00005"),
        StrSml::from("StrTest1"),
        vec![LangCode::EN, LangCode::FR],
        vec![StrLrg::from("Hello"), StrLrg::from("Bonjour")]
    );
    let write_result = write_form(file_path, &form_1);
    match write_result {
        Ok(_) => println!("Form 1 written successfully {:?}.", form_1.form_id()),
        Err(e) => println!("Error writing form 1: {}", e),
    }

    // 2. Adding a form to the start of the archive
    let form_2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("WrldDesert"),
        StrSml::from("Desert of Derak"),
        vec![GlobalID::from("00100050"), GlobalID::from("00100051")]
    );
    let write_result = write_form(file_path, &form_2);
    match write_result {
        Ok(_) => println!("Form 2 written successfully {:?}.", form_2.form_id()),
        Err(e) => println!("Error writing form 2: {}", e),
    }

    // 3. Adding a form inside the archive
    let form_3 = FormWorld::new(
        FormID::from("00003"),
        StrSml::from("WrldJungle"),
        StrSml::from("Green Wastes"),
        vec![GlobalID::from("00100053"), GlobalID::from("00100055")]
    );
    let write_result = write_form(file_path, &form_3);
    match write_result {
        Ok(_) => println!("Form 3 written successfully {:?}.", form_3.form_id()),
        Err(e) => println!("Error writing form 3: {}", e),
    }

    // Reading the forms back
    println!("-- Reading forms from archive --");
    let read_form_1 = read_form(file_path, form_1.form_id());
    match read_form_1 {
        Ok(form) => println!("Form 1 read successfully: {:?}", form),
        Err(e) => println!("Error reading form 1: {}", e),
    }

    let read_form_2 = read_form(file_path, form_2.form_id());
    match read_form_2 {
        Ok(form) => println!("Form 2 read successfully: {:?}", form),
        Err(e) => println!("Error reading form 2: {}", e),
    }

    let read_form_3 = read_form(file_path, form_3.form_id());
    match read_form_3 {
        Ok(form) => println!("Form 3 read successfully: {:?}", form),
        Err(e) => println!("Error reading form 3: {}", e),
    }
}

#[allow(unused)]
pub fn test_form_perf(file_path: &str, form_count: u16) {
    println!("--- Performance Test: Writing {} Forms ---", form_count);
    
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
        let form_id = FormID::from(i);

        let form_name = format!("Wrld{}", i);
        let form_description = format!("Description{}", i);

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

    // Delete the last form
    let last_form_id = FormID::from(1);
    let delete_result = delete_form(file_path, last_form_id);
    match delete_result {
        Ok(_) => println!("Deleted FormID: {:?}", last_form_id),
        Err(e) => println!("Error deleting FormID {:?}: {:?}", last_form_id, e),
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
