use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn formtype_add(formtype_name: &str) -> io::Result<(PathBuf, PathBuf, PathBuf)> {
    // Get the current working directory
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return Err(e);
        }
    };

    // make sure the input is camel case
    let formtype_name = formtype_name.to_lowercase();
    let first_char = formtype_name.chars().next().unwrap().to_uppercase().to_string();
    let formtype_name = first_char + &formtype_name[1..];

    // Define the system-agnostic relative path for the form template
    let relative_template_path = Path::new("src/tooling/automation/templates/formclass.templ");

    // Combine the current directory with the relative template path
    let path_form_template = current_dir.join(relative_template_path);

    // Define the output directory (without the specific file)
    let path_form_out_dir = current_dir
        .join("src")
        .join("core")
        .join("structs")
        .join("forms");

    // Define the path for mod.rs
    let mod_file_path = path_form_out_dir.join("mod.rs");

    // Define the path for types_misc.rs
    let path_types_misc = current_dir
        .join("src")
        .join("core")
        .join("structs")
        .join("types")
        .join("types_misc.rs");

    // Print or use the full paths
    println!("Form template path: {}", path_form_template.display());
    println!("Form output directory: {}", path_form_out_dir.display());
    println!("Types misc path: {}", path_types_misc.display());

    // Generate the form class
    match generate_form_type_class(&path_form_template, &path_form_out_dir, &formtype_name) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Failed to generate form class: {}", e);
            return Err(e);
        }
    }

    // Append to the mod.rs file
    match append_to_mod_file(&mod_file_path, &formtype_name) {
        Ok(_) => println!("Form type successfully added to mod.rs."),
        Err(e) => {
            eprintln!("Failed to append form type to mod.rs: {}", e);
            return Err(e);
        }
    }

    // Return all paths for further operations
    Ok((path_form_template, path_form_out_dir, path_types_misc))
}

fn generate_form_type_class(path_form_template: &Path, path_form_out_dir: &Path, formtype_name: &str) -> io::Result<()> {
    // Read the form template file
    let form_template = match std::fs::read_to_string(path_form_template) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read form template file: {}", e);
            return Err(e);
        }
    };

    // Replace the formtype_name placeholder with the actual formtype_name
    let form_type_name_lower = formtype_name.to_lowercase();
    let form_class = form_template.replace("FormType::*FORMTYPE*", &format!("FormType::{}", formtype_name.to_uppercase()));
    let form_class = form_class.replace("*FORMTYPE*", formtype_name);
    // Define the path for the new form file
    let path_form_out = path_form_out_dir.join(format!("struc_form_{}.rs", form_type_name_lower));

    // Write the form class to the output file
    match std::fs::write(&path_form_out, form_class) {
        Ok(_) => {
            println!("Form class generated successfully at {}", path_form_out.display());
        },
        Err(e) => {
            eprintln!("Failed to write form class to file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn append_to_mod_file(mod_file_path: &Path, formtype_name: &str) -> io::Result<()> {
    // Read the existing mod.rs content
    let mut mod_file_content = match std::fs::read_to_string(mod_file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Creating a new mod.rs file at {}", mod_file_path.display());
            String::new() // If the mod.rs file doesn't exist, we create a new one
        }
    };

    let formtype_name_lower = formtype_name.to_lowercase();
    // Create the module and pub use statements for the new form type
    let mod_entry = format!("pub mod struc_form_{};\npub use struc_form_{}::*;\n", formtype_name_lower, formtype_name_lower);

    // Check if the formtype_name is already in the mod.rs file
    if !mod_file_content.contains(&mod_entry) {
        // Append the new form type module to mod.rs
        mod_file_content.push_str(&mod_entry);

        // Write the updated mod.rs content back to the file
        let mut mod_file = OpenOptions::new().write(true).truncate(true).open(mod_file_path)?;
        mod_file.write_all(mod_file_content.as_bytes())?;

        println!("Form type {} appended to mod.rs", formtype_name);
    } else {
        println!("Form type {} is already in mod.rs", formtype_name);
    }

    Ok(())
}

