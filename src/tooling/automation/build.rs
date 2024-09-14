use std::process::Command;
use std::fs;
use std::path::Path;

const EXTERNAL_OUTPUT_PATHS: [&str; 2] = [
    "D:\\Work\\Projects\\GameDev\\SmnEngine\\DevTools\\ArchiveTools\\libs\\smnarchpy\\src\\smnarchpy\\lib",
    "D:\\Work\\Projects\\GameDev\\SmnEngine\\SmnEngine\\Binaries\\Win64"
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
    let lib_files = vec!["smn_archive_extern.dll", "smn_archive_extern.dll.exp", "smn_archive_extern.dll.lib", "smn_archive_extern.pdb"];

    // Step 3: Copy the compiled library files to each external project
    for output_path in EXTERNAL_OUTPUT_PATHS.iter() {
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

    println!("Files copied to external projects.");
}
