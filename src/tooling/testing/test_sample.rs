use std::{env, path::PathBuf};

use crate::core::{io::{read_form, write_archive_skeleton, write_form}, structs::{Archive, ArchiveID, FormID, FormRefGroup, FormString, FormWorld, GlobalID, LangCode, StrLrg, StrSml, Vec3Int, Version}};

pub fn test_sample() {
    // Write the archive
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let archive_path = current_dir.join("archives").join("test_sample.smn");
    let path = archive_path.to_str().unwrap();

    let archive_id = ArchiveID::from(1);

    let archive = Archive::new(
        archive_id,
        Version::from((1,2)), 
        StrLrg::from("Test Archive")
    );

    println!("Raw Archive Info Bytes: {:?}", archive.header_to_bytes());

    let _ = write_archive_skeleton(&path, &archive);
    
    // Write a RefGroup
    let form_id = FormID::from(1);
    let form_name = StrSml::from("CollWrldList");

    let form = FormRefGroup::new(
        form_id,
        form_name,
        vec![GlobalID::from((archive_id, FormID::from(50))), GlobalID::from((archive_id, FormID::from(51)))]
    );

    println!("{:?}", form.to_bytes());

    let _ = write_form(&path, &form);

    // Write the worlds
    let form_id = FormID::from(50);
    let form_name = StrSml::from("WrldBeach");

    let form = FormWorld::new(
        form_id,
        form_name,
        GlobalID::from((archive_id, FormID::from(200))),
        StrSml::from("BeachOfAmonal"),
        vec![
            GlobalID::from((archive_id, FormID::from(105))),
            GlobalID::from((archive_id, FormID::from(106)))
        ],
        vec![
            Vec3Int::from((1, 2, 0)),
            Vec3Int::from((1000, -300, 50)),
        ]
    );

    let _ = write_form(&path, &form);

    let form_id = FormID::from(51);
    let form_name = StrSml::from("WrldForest");

    let form = FormWorld::new(
        form_id,
        form_name,
        GlobalID::from((archive_id, FormID::from(201))),
        StrSml::from("ForestOfAmonal"),
        vec![
            GlobalID::from((archive_id, FormID::from(106))),
            GlobalID::from((archive_id, FormID::from(107))),
            GlobalID::from((archive_id, FormID::from(108)))
        ],
        vec![
            Vec3Int::from((15, 200, 0)),
            Vec3Int::from((500, -3000, 100)),
            Vec3Int::from((100, 200, 300))
        ]
    );

    let _ = write_form(&path, &form);

    println!("------------------------");

    let form_id = FormID::from(10);
    let form_name = StrSml::from("StrTitle");
    let languages = vec![LangCode::EN, LangCode::FR];
    let strings = vec![
        StrLrg::from("Welcome to NullPoint"),
        StrLrg::from("Bienvenue à NullPoint")
    ];

    let form = FormString::new(form_id, form_name, languages, strings);
    let _ = write_form(&path, &form);

    // Read the forms
    let form = read_form(&path, FormID::from(1));
    println!("{:?}", form);
    println!("------------------------");
    let form = read_form(&path, FormID::from(50));
    println!("{:?}", form);

    let form = read_form(&path, FormID::from(51));
    println!("{:?}", form);
    
    // Attempt to read the form and handle errors
    let form = match read_form(&path, FormID::from(51)) {
        Ok(f) => f,
        Err(_) => {
            println!("Form not found");
            return;
        }
    };

    // Convert the form to bytes
    let form_bytes = form.to_bytes();
    let len = form_bytes.len() as u32;
    
    // Print the form bytes
    println!("Form Bytes: {:?}", form_bytes);
    println!("Form Bytes Length: {}", len);


    println!("------------------------");
}