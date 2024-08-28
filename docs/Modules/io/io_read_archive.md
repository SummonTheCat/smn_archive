# `read_archive_info`

**Purpose**:
The `read_archive_info` function reads the basic information of an archive file. It retrieves the HEADER and BYTESTART blocks from the file and constructs an `Archive` instance using the retrieved data.

**Parameters**:
- **`file_path`** (`&str`): A string slice specifying the path to the archive file that needs to be read. The function opens this file for reading.

**Operation**:
1. **File Access**: The function attempts to open the specified file in read-only mode. It returns an error if the file cannot be opened.
2. **Reading HEADER Block**: The HEADER block is read from the file, providing the initial metadata required for the `Archive` instance.
3. **Reading BYTESTART Block**: The BYTESTART block is read, containing the positions of various data blocks in the file.
4. **Constructing Archive**: The function constructs an `Archive` instance using the retrieved HEADER and BYTESTART data, filling in the necessary fields.

**Error Handling**:
- The function returns an `io::Result<Archive>`.
- If any step in the process (file opening, block reading) fails, the function immediately returns an error.
- The function logs error messages to the console using `eprintln!`.

**Usage Example**:
```rust
let archive_result = read_archive_info("path/to/archive.bin");
match archive_result {
    Ok(archive) => println!("Archive ID: {}", archive.archive_id),
    Err(e) => eprintln!("Failed to read archive info: {}", e),
}
```

**Dependencies**:

- **`read_block_header`**: Reads the HEADER block from the file.
- **`read_block_bytestarts`**: Reads the BYTESTART block from the file.

**Related Functions**:

- `write_archive_info`: A counterpart function that writes or updates the archive information in a file.


# `read_lite_archive`

**Purpose**:
The `read_lite_archive` function reads a simplified version of the archive file. It retrieves basic metadata such as archive ID, version, description, and form count, along with minimal form metadata, constructing a `LiteArchive` instance.

**Parameters**:
- **`file_path`** (`&str`): A string slice specifying the path to the archive file that needs to be read. The function opens this file for reading.

**Operation**:
1. **File Access**: The function attempts to open the specified file in read-only mode. It returns an error if the file cannot be opened.
2. **Reading HEADER and BYTESTART Blocks**: The HEADER block is read to retrieve general metadata, while the BYTESTART block provides information on where the INDEX block is located.
3. **Reading INDEX Block**: The INDEX block is read, containing metadata for each form within the archive.
4. **Form Metadata Retrieval**: The function iterates over each item in the INDEX block, attempting to read each form's name. If a form is not found, a placeholder name is used.
5. **Constructing LiteArchive**: A `LiteArchive` instance is created with the gathered metadata, including a list of `LiteArchiveItem` instances for each form.

**Error Handling**:
- The function returns an `io::Result<LiteArchive>`.
- If any step in the process (file opening, block reading, form reading) fails, the function immediately returns an error.
- The function logs error messages to the console using `eprintln!`.

**Usage Example**:
```rust
let lite_archive_result = read_lite_archive("path/to/archive.bin");
match lite_archive_result {
    Ok(lite_archive) => println!("Archive ID: {}", lite_archive.archive_id),
    Err(e) => eprintln!("Failed to read lite archive: {}", e),
}
```

**Dependencies**:

- **`read_block_header`**: Reads the HEADER block from the file.
- **`read_block_bytestarts`**: Reads the BYTESTART block from the file.
- **`read_block_index`**: Reads the INDEX block from the file.
- **`read_form`**: Reads a form by its ID from the file.

**Related Functions**:

- `read_archive_info`: A function that reads a more detailed version of the archive from the file.

# `get_form_exists`

**Purpose**:
The `get_form_exists` function checks whether a form with a specified ID exists within an archive file. It tries to read the form using its ID, returning a boolean value indicating the form's presence.

**Parameters**:
- **`file_path`** (`&str`): A string slice specifying the path to the archive file that should be searched.
- **`target_form_id`** (`FormID`): The ID of the form to check for within the archive file.

**Operation**:
1. **Form Check**: The function attempts to read the form with the specified ID from the archive file.
2. **Return Value**:
   - If the form is successfully read, the function returns `Ok(true)`, indicating that the form exists.
   - If the form is not found, the function returns `Ok(false)`.
   - If another I/O error occurs (e.g., permission issues), the function returns an error.

**Error Handling**:
- The function returns an `io::Result<bool>`.
- It differentiates between a "form not found" error and other I/O errors, handling them accordingly.

**Usage Example**:
```rust
let form_exists_result = get_form_exists("path/to/archive.bin", target_form_id);
match form_exists_result {
    Ok(true) => println!("Form exists in the archive."),
    Ok(false) => println!("Form does not exist in the archive."),
    Err(e) => eprintln!("Failed to check form existence: {}", e),
}
```

**Dependencies**:

- **`read_form`**: Attempts to read the specified form from the archive file.

**Related Functions**:

- `read_lite_archive`: A function that reads minimal metadata from the archive, which can also be used to check for form presence.