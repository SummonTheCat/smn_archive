# SmnArchive

SmnArchive is a high-performance library designed to manage `.smn` binary archives, optimized for real-time data streaming. It enables efficient reading and writing of complex forms of data in a structured, block-based archive format.

**Notable Info:**
- [SmnArchive - Type Structures - Binary File](./smnarchive/SmnArchive_Type_Structures_Binary_File.md) - Binary file layout.
- [SmnArchive - Type Structures - Values](./smnarchive/SmnArchive_Type_Structures_Values.md) - Value type sizes and formats.
- [SmnArchive - Type Structures - Forms](./smnarchive/SmnArchive_Type_Structures_Forms.md) - Form structure details.
- [SmnArchive - Glossary](./smnarchive/SmnArchive_Glossary.md) - Definitions of key terms.
- [SmnArchive - Statistics](./smnarchive/SmnArchive_Statistics.md) - Performance benchmarks.
- [SmnArchive - FFI](./smnarchive/SmnArchive_FFI.md) - C/C++ interoperability details.

## Overview

**SmnArchive** provides a structured way to manage data archives with fast read and write access, making it ideal for systems requiring real-time data processing. The library organizes data into forms, which can represent strings, world configurations, and various other data types, all of which are managed within a binary archive format.

### Core Modules

1. **IO Module**:  
   Handles input/output operations for `.smn` files. Functions include reading and writing data to the archive, ensuring performance and integrity.
   
   Key Functions:
   - `read_form`: Fetches data for a specific form.
   - `write_form`: Inserts or updates a form in the archive.
   - `write_archive_skeleton`: Initializes a new archive structure.
   - `read_lite_archive`: Provides a lightweight view of the archive’s forms.

2. **Structs Module**:  
   Defines the core data structures used in SmnArchive, including forms and various data types such as `FormID`, `GlobalID`, vectors (`Vec3Int`, `Vec3Float`), and strings (`StrSml`, `StrLrg`).
   
   Key Structures:
   - **Archive**: Represents the entire archive structure.
   - **FormID**: Unique identifier for forms.
   - **FormWorld**: Structure to handle world-related data within the archive.
   - **FormRefGroup**: Used for managing references between forms.

3. **FFI Module**:  
   Provides Foreign Function Interface (FFI) bindings, enabling integration with C and C++ code. This allows developers to interact with SmnArchive functionality from non-Rust projects, facilitating cross-language compatibility.

   Key Functions:
   - `smn_write_archive_skeleton`: Initializes a new archive, callable from C/C++.
   - `smn_read_form`: Reads a form from the archive, returning a pointer to the serialized form data.
   - `smn_get_form_exists`: Checks whether a form exists in an archive.

## Key Operations

### Archive Management
- **write_archive_skeleton**: Initializes a new archive structure.
- **read_archive_info**: Retrieves metadata about the archive.
- **write_archive_info**: Updates archive metadata.

### Form Handling
- **read_form**: Retrieves data for a specific form.
- **read_forms**: Retrieves multiple forms simultaneously.
- **write_form**: Adds or updates a form in the archive.
- **delete_form**: Removes a form from the archive.

## Example Usage

Create an archive, add a form, and read it back:

```rust
use crate::core::{
    io::{read_form, write_archive_skeleton, write_form},
    structs::{Archive, FormID, FormWorld},
};

fn main() {
    let archive = write_archive_skeleton("path/to/archive.smn").unwrap();
    let form_id = FormID::new(1001);
    let world_form = FormWorld { name: "New World".to_string() };
    write_form(&archive, form_id, &world_form).unwrap();
    let retrieved_form = read_form(&archive, &form_id).unwrap();
}
```

## Further Resources

- Explore more in-depth documentation through the **Notable Info** links, covering everything from type structures to FFI integration.
