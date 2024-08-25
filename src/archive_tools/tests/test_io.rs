use std::time::Instant;

use rand::Rng;

use crate::archive_tools::types::*;
use crate::archive_tools::structs::{Archive, FormWorld};
use crate::archive_tools::io::*;

#[allow(unused)]
pub fn test_archive_write(file_path: &str) {
    println!("--- Testing Archive Write ---");
    println!("Writing to file: {}", file_path);

    // Create an archive
    let archive = Archive::new(
        ArchiveID::from("002"),
        Version::from(1.2),
        StrLrg::from("Test Archive"),
    );

    io_write_archive_skeleton(file_path, &archive);
}

#[allow(unused)]
pub fn test_archive_read(file_path: &str) {
    let archive_info = read_archive_info(file_path);
    println!("Archive Info: {:?}", archive_info);

    println!("Finished reading the archive data.");
}

#[allow(unused)]
pub fn test_write_form(file_path: &str) {
    println!("--- Testing Form Write ---");
    println!("Writing to file: {}", file_path);

    println!("---------------------------");
    // Create a form
    let form = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("Wrld2"),
        StrSml::from("Desert"),
        vec![GlobalID::from("00100050")]
    );

    let write_result = write_form(file_path, &form);
    println!("Write Result: {:?}", write_result);

    println!("---------------------------");
    let read_form1 = read_form(file_path, FormID::from("00002"));
    if read_form1.is_err() {
        println!("Error reading form: {:?}", read_form1.err());
    }

    println!("---------------------------");

    let form2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("Jungle"),
        vec![GlobalID::from("00100051")]
    );

    let write_result2 = write_form(file_path, &form2);
    println!("Write Result: {:?}", write_result2);

    println!("---------------------------");
    
    let form3 = FormWorld::new(
        FormID::from("00004"),
        StrSml::from("Wrld4"),
        StrSml::from("Tundra"),
        vec![GlobalID::from("00100052")]
    );

    let write_result3 = write_form(file_path, &form3);
    println!("Write Result: {:?}", write_result3);

    println!("---------------------------");
    let read_form2 = read_form(file_path, FormID::from("00004"));
    if read_form2.is_err() {
        println!("Error reading form: {:?}", read_form2.err());
    }

    println!("---------------------------");
    let form4 = FormWorld::new(
        FormID::from("00003"),
        StrSml::from("Wrld3"),
        StrSml::from("Forest"),
        vec![GlobalID::from("00100053")]
    );

    let write_result4 = write_form(file_path, &form4);
    println!("Write Result: {:?}", write_result4);

    println!("---------------------------");
    let read_form3 = read_form(file_path, FormID::from("00002"));
    if read_form3.is_err() {
        println!("Error reading form: {:?}", read_form3.err());
    }

    println!("---------------------------");
    let form3b = FormWorld::new(
        FormID::from("00004"),
        StrSml::from("Wrld4"),
        StrSml::from("undra"),
        vec![GlobalID::from("00100054")]
    );

    let write_result3b = write_form(file_path, &form3b);

    println!("Write Result: {:?}", write_result3b);

    println!("---------------------------");

    let read_form4 = read_form(file_path, FormID::from("00004"));
    if read_form4.is_err() {
        println!("Error reading form: {:?}", read_form4.err());
    }

    println!("---------------------------");

    let form1b = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("Wrld2"),
        StrSml::from("DesertOfRed"),
        vec![GlobalID::from("00100051")]
    );
    
    let write_result1b = write_form(file_path, &form1b);
    println!("Write Result: {:?}", write_result1b);

    println!("---------------------------");
    let form5 = FormWorld::new(
        FormID::from("00005"),
        StrSml::from("Wrld5"),
        StrSml::from("Plains"),
        vec![GlobalID::from("00100055")]
    );

    let write_result5 = write_form(file_path, &form5);
    println!("Write Result: {:?}", write_result5);

    println!("---------------------------");

    let read_form5 = read_form(file_path, FormID::from("00001"));
    if read_form5.is_err() {
        println!("Error reading form: {:?}", read_form5.err());
    }

    println!("---------------------------");   

    let archive_info = read_archive_info(file_path);
    let archive_data = archive_info.unwrap();
    let new_desc = StrLrg::from("New Description");
    
    let mut new_archive = Archive::new(
        archive_data.archive_id,
        archive_data.version,
        new_desc
    );

    new_archive.bytestart_data = archive_data.bytestart_data;
    new_archive.bytestart_index = archive_data.bytestart_index;
    new_archive.form_count = archive_data.form_count;

    println!("New Archive: {:?}", new_archive);

    write_archive_info(&file_path, &new_archive);

    println!("---------------------------");

    let archive_info2 = read_archive_info(file_path);
    println!("New Archive Info: {:?}", archive_info2);

    println!("---------------------------");

    let read_form6 = read_form(file_path, FormID::from("00002"));
    if read_form6.is_err() {
        println!("Error reading form: {:?}", read_form6.err());
    } else {
        println!("Read Form: {:?}", read_form6.unwrap());
    }
    println!("---------------------------");

    let lite_archive = read_lite_archive(file_path);
    println!("Lite Archive: {:?}", lite_archive);

    println!("---------------------------");

    let form_exists1 = get_form_exists(file_path, FormID::from("00007"));
    println!("Form Exists: {:?}", form_exists1);
}



#[allow(unused)]
pub fn test_perf_write_x_forms(file_path: &str, form_count: u16) {
    println!("--- Performance Test: Writing {} Forms ---", form_count);
    
    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );

    io_write_archive_skeleton(file_path, &archive);
    
    let mut rng = rand::thread_rng();

    println!("--- Started Writing Forms ---");
    // Start timing the write process
    let write_start = Instant::now();
    
    // Writing forms
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
            println!("Successfully wrote FormID: {}", i);
        }
    }

    // End timing the write process
    let write_duration = write_start.elapsed();
    println!("--- Finished Writing Forms ---");
    println!("Time taken to write {} forms: {:?}", form_count, write_duration);

    println!("--- Started Reading Forms ---");
    // Start timing the read process
    let read_start = Instant::now();

    // Reading forms back
    for i in 1..=form_count {
        let form_id_str = format!("{:05}", i);
        let form_id = FormID::from(form_id_str.as_str());

        let read_form = read_form(file_path, form_id);
        match read_form {
            Ok(_) => println!("Successfully read FormID: {}", form_id_str),
            Err(e) => println!("Error reading FormID {}: {:?}", form_id_str, e),
        }
    }

    // End timing the read process
    let read_duration = read_start.elapsed();
    println!("--- Finished Reading Forms ---");
    println!("Time taken to read {} forms: {:?}", form_count, read_duration);

    println!("--- Performance Test Completed ---");
    println!("Breakdown:");
    println!("Write Duration: {:?}", write_duration);
    println!("Read Duration: {:?}", read_duration);
    println!("Total Duration: {:?}", write_duration + read_duration);
}

#[allow(unused)]
pub fn test_form_delete(file_path: &str) {
    println!("---------------------------");
    println!("--- Testing Form Delete ---");
    println!("Writing to file: {}", file_path);

    // create the archive
    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );

    io_write_archive_skeleton(file_path, &archive);
    
    println!("---------------------------");
    // Create a form
    let form = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("Wrld2"),
        StrSml::from("Desert"),
        vec![GlobalID::from("00100050")]
    );

    let write_result = write_form(file_path, &form);
    println!("Write Result: {:?}", write_result);

    println!("---------------------------");

    let form2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("Jungle"),
        vec![GlobalID::from("00100051")]
    );

    let write_result2 = write_form(file_path, &form2);
    println!("Write Result: {:?}", write_result2);

    println!("---------------------------");

    // Delete a form
    let delete_result = delete_form(file_path, FormID::from("00002"));
    println!("Delete Result: {:?}", delete_result);

    println!("---------------------------");

    let read_form1 = read_form(file_path, FormID::from("00001"));
    if read_form1.is_err() {
        println!("Error reading form: {:?}", read_form1.err());
    } else {
        println!("Read Form: {:?}", read_form1.unwrap());
    }

    println!("---------------------------");

    let delete_result2 = delete_form(file_path, FormID::from("00001"));
    println!("Delete Result: {:?}", delete_result2);
}

#[allow(unused)]
pub fn test_form_override(file_path: &str) {
    println!("Writing to file: {}", file_path);

    println!("---------------------------");

    // create the archive
    let archive = Archive::new(
        ArchiveID::from("001"),
        Version::from(1.0),
        StrLrg::from("Test Archive"),
    );
    io_write_archive_skeleton(file_path, &archive);

    println!("---------------------------");

    // Create forms
    let form = FormWorld::new(
        FormID::from("00002"),
        StrSml::from("Wrld2"),
        StrSml::from("Desert"),
        vec![GlobalID::from("00100050")]
    );

    let write_result = write_form(file_path, &form);
    println!("Write Result: {:?}", write_result);

    let form2 = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("Jungle"),
        vec![GlobalID::from("00100051")]
    );

    let write_result2 = write_form(file_path, &form2);
    println!("Write Result: {:?}", write_result2);
    
    println!("---------------------------");

    // Override a form
    let form2b = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("JungleOfGreen"),
        vec![GlobalID::from("00100051")]
    );

    let override_result = write_form(file_path, &form2b);
    println!("Override Result: {:?}", override_result);

    println!("---------------------------");

    let form2c = FormWorld::new(
        FormID::from("00001"),
        StrSml::from("Wrld1"),
        StrSml::from("Jngl"),
        vec![GlobalID::from("00100051")]
    );

    let override_result2 = write_form(file_path, &form2c);
    println!("Override Result: {:?}", override_result2);

}