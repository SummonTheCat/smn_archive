# `write_archive_skeleton`

**Purpose**:
The `write_archive_skeleton` function is responsible for creating the initial structure of an archive file. It writes the necessary blocks (HEADER, BYTESTART, and INDEX) sequentially into a new file specified by the `path`.

**Parameters**:
- **`path`** (`&str`): A string slice specifying the file path where the archive will be created. The function will attempt to create a new file at this location.
- **`archive`** (`&Archive`): A reference to an `Archive` structure that contains the data needed to populate the HEADER block.

**Operation**:
1. **File Creation**: The function begins by attempting to create a new file at the specified `path`. If the file creation fails (e.g., due to permission issues), the function returns an `io::Error`.
2. **HEADER Block**: It writes the HEADER block to the file using data from the provided `archive`. This block serves as the initial metadata for the archive.
3. **BYTESTART Block**: The function calculates the current position in the file to determine where the BYTESTART block will be written. It then writes the BYTESTART block, which typically records the starting byte positions of subsequent data blocks.
4. **INDEX Block**: Finally, the function prepares an empty INDEX block (using `IOStructIndex`) and writes it to the file. The INDEX block is designed to hold references to the data blocks within the archive.

**Error Handling**:
- The function returns an `io::Result<()>`.
- If any step in the process (file creation, block writing) fails, the function immediately returns an error.
- The function also logs error messages to the console using `eprintln!`.

**Usage Example**:
```rust
let archive = Archive::new();
let result = write_archive_skeleton("path/to/file.bin", &archive);
assert!(result.is_ok());
```

**Dependencies**:
- **`write_block_header`**: Writes the HEADER block to the file.
- **`write_block_bytestart`**: Writes the BYTESTART block to the file.
- **`write_block_index`**: Writes the INDEX block to the file.

**Related Functions**:
- `read_archive_skeleton`: A counterpart function that reads the basic structure of an archive from a file.

# `write_archive_info`

**Purpose**:
The `write_archive_info` function updates an existing archive file with new metadata. It reads the current HEADER, BYTESTART, and INDEX blocks, adjusts them according to the new archive data, and rewrites the file to reflect these changes.

**Parameters**: 
- **`file_path`** (`&str`): A string slice specifying the path to the existing archive file. The function opens this file for reading and writing. 
- **`archive`** (`&Archive`): A reference to an `Archive` structure containing the updated metadata, such as the archive ID, version, and description.

**Operation**:
1. **File Access**: The function opens the specified file in read/write mode. It returns an error if the file cannot be opened.
2. **Reading Blocks**: The existing HEADER, BYTESTART, and INDEX blocks are read from the file.
3. **Updating Metadata**: The function calculates the new positions for the BYTESTART and INDEX blocks based on the changes in the archive description length.
4. **Temporary File Handling**: Data following the BYTESTART block is temporarily stored in a separate file to facilitate rewriting.
5. **Rewriting the Archive**: The HEADER, BYTESTART, and INDEX blocks are rewritten with the new data, followed by the restoration of the remaining data from the temporary file.
6. **Cleanup**: The temporary file is deleted, and the original file is truncated to its new size.

**Error Handling**:
- The function returns an `io::Result<()>`.
- Any I/O errors during file access, reading, writing, or truncating will cause the function to return an error.
- The function logs errors to the console using `eprintln!`.

**Usage Example**:
```rust
let archive = Archive::new_with_data( /* ... parameters ... */ );
let result = write_archive_info("path/to/archive.bin", &archive);
assert!(result.is_ok());
```

Dependencies:

- `read_block_header`: Reads the HEADER block from the file.
- `read_block_bytestarts`: Reads the BYTESTART block from the file.
- `read_block_index:` Reads the INDEX block from the file.
- `write_block_header`: Writes the HEADER block to the file.
- `write_block_bytestart`: Writes the BYTESTART block to the file.
- `write_block_index`: Writes the INDEX block to the file.

Related Functions:

- `read_archive_info`: A counterpart function that reads and returns the archive information from a file.