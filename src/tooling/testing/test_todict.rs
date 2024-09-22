use std::{env, path::PathBuf};

use crate::core::{io::read_form, structs::FormID};

pub fn test_todict() {
     // Get the archive path
     let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
     let archive_path = current_dir.join("archives").join("test_sample.smn");
     let path = archive_path.to_str().unwrap();

     let form1_id = FormID::from(1);
     let form1 = read_form(path, form1_id);
    if let Ok(form) = form1 {
        println!("{:?}", form.to_dict());
    } else {
        println!("Failed to read form");
    }
}