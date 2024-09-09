use std::fs::File;
use std::io::{self, Seek, SeekFrom};

use crate::archive_tools::io::{read_block_bytestarts, read_block_header};
use crate::archive_tools::structs::{Archive, LiteArchive, LiteArchiveItem};
use crate::archive_tools::structs::{FormID, StrSml};

use super::{read_block_index, read_form};

/// Reads the archive information from the specified file.
///
/// Opens the file at `file_path`, reads the HEADER and BYTESTART blocks,
/// and constructs an `Archive` instance with the retrieved data.
///
/// # Parameters
/// - `file_path`: The path to the archive file to be read.
///
/// # Returns
/// - `io::Result<Archive>`: The `Archive` instance on success, or an error on failure.
///
/// For more details, see the documentation in `io_read_archive.rs`.
pub fn read_archive_info(file_path: &str) -> io::Result<Archive> {
    // Open the file for reading
    let mut file = File::open(file_path)?;

    // Read the HEADER block from the file
    let header = read_block_header(&mut file)?;

    // Read the BYTESTART block from the file
    let bytestarts = read_block_bytestarts(&mut file)?;

    // Construct a new Archive instance with the retrieved data
    let mut archive_out = Archive::new(header.archive_id, header.version, header.description);
    archive_out.form_count = header.form_count;
    archive_out.bytestart_index = bytestarts.bytestart_index;
    archive_out.bytestart_data = bytestarts.bytestart_data;

    // Return the populated Archive instance
    Ok(archive_out)
}


/// Reads a lite version of the archive from the specified file.
///
/// Opens the file at `file_path`, reads the HEADER and BYTESTART blocks,
/// and constructs a `LiteArchive` instance with minimal information, including form metadata.
///
/// # Parameters
/// - `file_path`: The path to the archive file to be read.
///
/// # Returns
/// - `io::Result<LiteArchive>`: The `LiteArchive` instance on success, or an error on failure.
///
/// For more details, see the documentation in `io_read_archive.rs`.
pub fn read_lite_archive(file_path: &str) -> io::Result<LiteArchive> {
    
    // Open the file for reading
    let mut file = File::open(file_path)?;

    // Read the HEADER block from the file
    let header = read_block_header(&mut file)?;

    // Read the BYTESTART block from the file
    let bytestarts = read_block_bytestarts(&mut file)?;

    // Initialize the LiteArchive with basic header information
    let mut archive_out = LiteArchive {
        archive_id: header.archive_id,
        version: header.version,
        description: header.description,
        form_count: header.form_count,
        archive_items: Vec::new(),
    };

    // Seek to the INDEX block in the file
    file.seek(SeekFrom::Start(bytestarts.bytestart_index as u64))?;
    let index = read_block_index(&mut file, header.form_count)?;
    
    // Iterate over each index item and retrieve form metadata
    for index_item in index.indexes {
        let form_id = index_item.form_id;
        let form_type = index_item.form_type;

        // Read the form to retrieve its name, handle error if form is not found
        let index_form = read_form(file_path, form_id);
        let form_name = match index_form {
            Ok(form) => form.form_name(),
            Err(_) => StrSml::from("Form not found"),
        };

        // Create a LiteArchiveItem and add it to the archive
        let new_lite_form = LiteArchiveItem {
            form_id,
            form_name,
            form_type,
        };
        archive_out.archive_items.push(new_lite_form);
    }

    // Return the populated LiteArchive instance
    Ok(archive_out)
}



/// Checks if a form with the specified ID exists in the archive file.
///
/// Attempts to read the form specified by `target_form_id` from the file at `file_path`.
/// Returns `true` if the form exists, `false` if the form is not found, or an error if another I/O error occurs.
///
/// # Parameters
/// - `file_path`: The path to the archive file to be searched.
/// - `target_form_id`: The ID of the form to check for existence.
///
/// # Returns
/// - `io::Result<bool>`: `Ok(true)` if the form exists, `Ok(false)` if the form does not exist,
///   or an `Err` if another error occurs.
///
/// For more details, see the documentation in `io_read_archive.rs`.
pub fn get_form_exists(file_path: &str, target_form_id: FormID) -> io::Result<bool> {
    match read_form(file_path, target_form_id) {
        Ok(_) => Ok(true),  // Form exists
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)  // Form does not exist
            } else {
                Err(e)  // Other I/O error occurred
            }
        }
    }
}
