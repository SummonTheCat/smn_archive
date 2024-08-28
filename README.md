# SMN Archive Library

lorum ipsum dolor sit amet.

## Overview

The SMN Archive Library is intended for use cases where large collections of forms (data entries) need to be stored, retrieved, and modified efficiently. It is particularly well-suited for applications where binary data handling and performance are critical.

### Key Features:
- **Custom Archive Management**: Create, read, update, and delete forms within an archive.
- **FFI Compatibility**: Expose core functionality to other languages such as C or Python via a simple FFI interface.
- **Modular Design**: Easily extend or modify components by leveraging Rust’s modular architecture.

- **`archive_tools/`**: Contains the tools related to archive operations.
  - **`io/`**: Handles reading and writing binary data.
    - **`io_delete_form.rs`**: Manages deletion of forms from the archive.
    - **`io_read_archive.rs`**: Handles reading of archive data.
    - **`io_read_block.rs`**: Manages reading blocks of data.
    - **`io_read_form.rs`**: Handles reading individual forms.
    - **`io_struct.rs`**: Defines structures related to I/O operations.
    - **`io_write_archive.rs`**: Handles writing data to the archive.
    - **`io_write_block.rs`**: Manages writing blocks of data.
    - **`io_write_form.rs`**: Handles writing individual forms.
  - **`mod.rs`**: Organizes modules within `archive_tools`.

- **`structs/`**: Defines the core structures used throughout the library.
  - **`struc_archive.rs`**: Structure definitions for archive management.
  - **`struc_form_string.rs`**: Handles string-based form structures.
  - **`struc_form_world.rs`**: Manages world-based form structures.
  - **`struc_form.rs`**: Defines general form structures.

- **`tests/`**: Contains unit tests and validation scripts.
  - **`test_archive.rs`**: Tests archive-related operations.
  - **`test_forms.rs`**: Tests form handling and manipulation.
  - **`test_types.rs`**: Tests different type definitions and their functionalities.

- **`types/`**: Contains type definitions for various archive components.
  - **`struc_types_id.rs`**: Defines ID types for forms and entries.
  - **`struc_types_misc.rs`**: Miscellaneous type definitions.
  - **`struc_types_str.rs`**: Defines string types used in the library.

- **`interchange/`**: Handles data interchange and external interaction.
  - **`interchange_io.rs`**: Manages data interchange I/O operations.

- **`lib.rs`**: Entry point for the Rust library, organizing modules.

- **`main.rs`**: Main entry point for running the application (if needed).

## Byte Structures

The SMN Archive Library operates on specific byte structures. Below is an overview of the primary structures:

- **Element**: `FormID`
    - **Byte Size**: 2 bytes
    - **Description**: A unique 2-byte identifier used for identifying forms within an archive. This identifier is represented as an unsigned 16-bit integer (`u16`) in big-endian format.

- **Element**: `ArchiveID`
    - **Byte Size**: 1 byte
    - **Description**: A unique 1-byte identifier used to distinguish between different archives. This identifier is represented as an unsigned 8-bit integer (`u8`).

- **Element**: `GlobalID`
    - **Sub-elements**:
        - `ArchiveID` (1 byte)
        - `FormID` (2 bytes)
    - **Byte Size**: 3 bytes (1 byte for `ArchiveID` + 2 bytes for `FormID`)
    - **Description**: A composite identifier that combines both the `ArchiveID` and `FormID` to uniquely identify forms across different archives. This structure is represented by a total of 3 bytes.

- **Element**: `StrSml`
    - **Byte Size**: 
        - 1 byte for character count
        - 2 bytes per character (UTF-16 encoding)
    - **Description**: A small UTF-16 encoded string structure with a maximum length of 255 characters. The string length is stored in the first byte, followed by the UTF-16 encoded characters. The characters are stored as their decimal representation in UTF-16 encoding.

- **Element**: `StrLrg`
    - **Byte Size**: 
        - 2 bytes for character count
        - 2 bytes per character (UTF-16 encoding)
    - **Description**: A large UTF-16 encoded string structure with a maximum length of 65,535 characters. The string length is stored in the first two bytes, followed by the UTF-16 encoded characters. The characters are stored as their decimal representation in UTF-16 encoding.

- **Element**: `Version`
  - **Byte Size**:
    - **Major Version**: 1 byte
    - **Minor Version**: 1 byte
  - **Description**: Represents a version number with major and minor components. Stored as two separate bytes, where the first byte represents the major version and the second byte represents the minor version. The version is displayed in the format `major.minor`.

- **Element**: `LangCode`
  - **Byte Size**: 1 byte
  - **Description**: Enum representing a language code. Stored as a single byte corresponding to a specific language (e.g., EN, FR, ES, DE). The byte value represents the decimal representation of the language code.

- **Element**: `FormType`
  - **Byte Size**: 1 byte
  - **Description**: Enum representing the type of form. Stored as a single byte, where different values represent different form types (e.g., STRING, WORLD). The byte value represents the decimal representation of the form type.

## External Functions

The Rust library exposes several functions for external use via FFI:

- **`create_archive(path: *const c_char) -> c_int`**: Creates a new archive at the specified path.
- **`open_archive(path: *const c_char) -> *mut ArchiveHandle`**: Opens an existing archive and returns a handle.
- **`close_archive(handle: *mut ArchiveHandle) -> c_int`**: Closes the archive and frees resources.
- **`add_form(handle: *mut ArchiveHandle, data: *const u8, length: usize) -> c_int`**: Adds a new form to the archive.
- **`get_form(handle: *mut ArchiveHandle, id: u32, buffer: *mut u8, buffer_length: usize) -> c_int`**: Retrieves a form by ID and copies it into the provided buffer.
- **`remove_form(handle: *mut ArchiveHandle, id: u32) -> c_int`**: Removes a form by ID from the archive.

## Interaction with Base Code

To interact with the base code in Rust:

1. **Build the Rust library**: Use Cargo to build the library and generate the shared object or static library files.

   ------
   cargo build --release
   ------

2. **Integrate with Python/C**: Use the FFI functions in the `ffi.rs` module to interact with the archive from Python or C. You can use `ctypes` in Python or link directly in C.

3. **Testing**: Use the scripts in `smn_archive_py_test/` to validate the integration and ensure that the library behaves as expected in your use case.
