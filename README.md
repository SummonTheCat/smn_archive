# SMN Archive Library

This library provides tools to create, manage, and interact with custom archive files in Rust. The library is designed to be efficient and is structured to allow easy integration with other languages through a C-compatible Foreign Function Interface (FFI).

## Overview

The SMN Archive Library is intended for use cases where large collections of forms (data entries) need to be stored, retrieved, and modified efficiently. It is particularly well-suited for applications where binary data handling and performance are critical.

### Key Features:
- **Custom Archive Management**: Create, read, update, and delete forms within an archive.
- **FFI Compatibility**: Expose core functionality to other languages such as C or Python via a simple FFI interface.
- **Modular Design**: Easily extend or modify components by leveraging Rust’s modular architecture.

## Project Structure
- **`smn_archive/`**: The main Rust library.
  - **`src/io.rs`**: Handles reading and writing binary data.
  - **`src/structs.rs`**: Defines the core structures used throughout the library.
  - **`src/types.rs`**: Contains type definitions for various archive components.
  - **`src/archive.rs`**: Implements the main archive operations.
  - **`src/ffi.rs`**: Provides the FFI functions for external interaction.
  - **`src/lib.rs`**: Entry point for the Rust library, organizing modules.
  
- **`smn_archive_py/`**: Python wrapper and interaction scripts.
  - **`__init__.py`**: Initializes the Python package.
  - **`smn_archive_interaction.py`**: Provides high-level Python functions for interacting with the archive.
  - **`ctypes_wrapper.py`**: Handles the interaction between Python and the Rust FFI.

- **`smn_archive_py_test/`**: Contains tests and examples for using the library.
  - **`test_smn_archive.py`**: Python test script for validating library functionality.
  - **`data/`**: Contains sample data files for testing.

## Byte Structures

The SMN Archive Library operates on specific byte structures. Below is an overview of the primary structures:

- **Archive Header**:
  - **Bytes 0-3**: Archive ID (4 bytes)
  - **Bytes 4-7**: Version Number (4 bytes)
  - **Bytes 8-11**: Number of Entries (4 bytes)
  - **Bytes 12-15**: Index Offset (4 bytes)

- **Entry Index**:
  - **Bytes 0-3**: Entry ID (4 bytes)
  - **Bytes 4-7**: Data Offset (4 bytes)
  - **Bytes 8-11**: Data Length (4 bytes)

- **Form Data**:
  - **Variable**: Binary data for the form, length defined in the Entry Index.

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
