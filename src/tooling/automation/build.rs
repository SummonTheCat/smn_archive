use std::process::Command;
use std::fs;
use std::path::Path;

const EXTERNAL_OUTPUT_PATHS: [&str; 2] = [
    "D:\\Work\\Projects\\GameDev\\SmnEngine\\DevTools\\ArchiveTools\\libs\\smnarchpy\\src\\smnarchpy\\lib",
    "D:\\Work\\Projects\\GameDev\\SmnEngine\\SmnUnreal\\Binaries\\Win64"
];

pub fn build_full() {
    // Step 1: Build the project in release mode
    println!("Building the project in release mode...");

    let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to build the project");

    if !build_status.success() {
        eprintln!("Failed to build the project in release mode.");
        return;
    }

    println!("Project successfully built.");

    // Step 2: Get the target library path
    let target_dir = "target\\release";
    let lib_files = vec!["smn_archive.dll", "smn_archive.dll.exp", "smn_archive.dll.lib", "smn_archive.pdb"];
    let lib_files_outname = vec!["smn_archive.dll", "smn_archive.dll.exp", "smn_archive.dll.lib", "smn_archive.pdb"];

    // Step 3: Copy the compiled library files to each external project
    println!("Copying the compiled library files to each external project...");

    for output_path in EXTERNAL_OUTPUT_PATHS.iter() {
        // Check if the output path exists
        if !Path::new(output_path).exists() {
            eprintln!("{:?} does not exist, skipping...", output_path);
            continue;
        } else {
            for lib_file in lib_files.iter() {
                let src = Path::new(target_dir).join(lib_file);
                let dest = Path::new(output_path).join(lib_file);
    
                if src.exists() {
                    match fs::copy(&src, &dest) {
                        Ok(_) => println!("Successfully copied {:?} to {:?}", src, dest),
                        Err(e) => eprintln!("Failed to copy {:?} to {:?}: {:?}", src, dest, e),
                    }
                } else {
                    println!("{:?} does not exist, skipping...", src);
                }
            }
        }
        
    }

    // Step 4: Rename the copied library files
    println!("Renaming the copied library files...");

    for output_path in EXTERNAL_OUTPUT_PATHS.iter() {
        for (i, lib_file) in lib_files.iter().enumerate() {
            let src = Path::new(output_path).join(lib_file);
            let dest = Path::new(output_path).join(lib_files_outname[i]);

            if src.exists() {
                match fs::rename(&src, &dest) {
                    Ok(_) => println!("Successfully renamed {:?} to {:?}", src, dest),
                    Err(e) => eprintln!("Failed to rename {:?} to {:?}: {:?}", src, dest, e),
                }
            } else {
                println!("{:?} does not exist, skipping...", src);
            }
        }
    }
}
