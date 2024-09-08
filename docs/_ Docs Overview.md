[[Docs_API]]
## Breakdown of the File Structure

---
#### `lib.rs`
This is the main library file that pulls everything together. It imports and re-exports the `interchange` module and other functionality from `archive_tools`, making them accessible when the library is used externally.

---
#### `main.rs`
The entry point for the binary executable. It contains a `main()` function, which in this case, runs a set of tests via the `archive_tools::tests::run_tests()` function.

---
#### `archive_tools/`
This is the main folder for handling operations related to archives and forms.
##### **`io/` Handles Input/Output Operations**
- **`io_struct.rs`**: Likely contains structures related to I/O operations, such as data formats for reading and writing.
- **`io_utils.rs`**: Utility functions used by the I/O operations, e.g., helper functions for reading/writing byte arrays.
- **`io_write_archive.rs`**: Functions for writing archive files, likely covering metadata and structure serialization.
- **`io_write_block.rs`**: Specific functions for writing blocks of data into an archive, potentially optimizing large data writes.
- **`io_read_archive.rs`**: Functions for reading archive files, deserializing data into usable structures.
- **`io_read_block.rs`**: Functions for reading specific data blocks from an archive file.
- **`io_write_form.rs`**: Functions for writing form data (likely game-related forms) into archives.
- **`io_read_form.rs`**: Functions for reading form data from archives.
- **`io_delete_form.rs`**: Functions for deleting forms from the archive.
##### **`structs/` Data Structures for Archives and Forms**
- **`struc_archive.rs`**: Defines the structure for an archive, including metadata like archive ID, version, description, and form count. This is where core functionality for the archive is likely handled, such as serialization to bytes.
- **`struc_form.rs`**: Defines the base structure for forms, including ID, type, and name, as well as trait implementations for forms.
- **`struc_form_string.rs`**: Specialized form structure for handling "String" type forms, including language support and the actual string data.
- **`struc_form_world.rs`**: Specialized form structure for handling "World" type forms, likely including world names and parts (represented by Global IDs).
##### **`types/` Type Definitions for IDs, Strings, etc.**
- **`struc_types_id.rs`**: Contains the `FormID`, `ArchiveID`, and `GlobalID` types. These types define how IDs are serialized, deserialized, and manipulated within the archive system.
- **`struc_types_str.rs`**: Contains the definitions for `StrSml` (small strings) and `StrLrg` (large strings), which manage the serialization and storage of UTF-16 strings with specific length constraints.
- **`struc_types_misc.rs`**: Contains miscellaneous types like `Version` (for archive versioning) and `LangCode` (for specifying language codes like "EN", "FR").

---
#### `interchange/`
This folder handles the interaction between the Rust archive system and external languages or systems, particularly via C FFI (Foreign Function Interface).

- **`interchange_io.rs`**: Contains the actual `extern "C"` functions that expose Rust functionalities to other languages (like C++ or Python). These functions handle tasks like reading/writing archives, forms, checking form existence, etc.

---