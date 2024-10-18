# SmnArchive

A library for managing the SmnArchive (`.smn`) binary archive structure. The binary structure is set up for real-time streaming of data, with a focus on performant reading.

**Notable Info:**
- [SmnArchive - Type Structures - Binary File](./smnarchive/SmnArchive_Type_Structures_Binary_File.md) - Describes the archive file's block-based layout.
- [SmnArchive - Type Structures - Values](./smnarchive/SmnArchive_Type_Structures_Values.md) - Defines sizes and formats for a form's stored value types.
- [SmnArchive - Type Structures - Forms](./smnarchive/SmnArchive_Type_Structures_Forms.md) - Describes structures for different form types.
- [SmnArchive - Glossary](./smnarchive/SmnArchive_Glossary.md) - Glossary of terms used in the SmnArchive library.
- [SmnArchive - Statistics](./smnarchive/SmnArchive_Statistics.md) - Performance statistics for reading and writing forms.

## Overview

**SmnArchive** is a robust library designed for managing `.smn` binary archive structures. It provides efficient real-time data streaming with a focus on high-performance reading and writing operations. The library offers comprehensive CRUD (Create, Read, Update, Delete) functionalities for handling archives and their constituent forms.

## Crate Module Imports

To effectively utilize the **SmnArchive** library, it's essential to understand how to import its core modules and components. Below is a reference snippet commonly used in the main documentation:

```rust
use crate::core::{
    io::{read_form, write_archive_skeleton, write_form},
    structs::{
        Archive, ArchiveID, EntID, EntInstance, FormID, FormRefGroup, FormString, FormWeather,
        FormWorld, FormWorldPart, GlobalID, LangCode, SmlColor, StrLrg, StrSml, Vec3Float, Vec3Int,
        Version,
    },
};
```

### Core Modules

The `core` module is the foundation of the **SmnArchive** library, containing essential submodules and components required for archive management. It is divided into two primary submodules: `io` and `structs`.

#### IO Module

The `io` submodule handles all input/output operations related to the archive. It provides functions to read from and write to `.smn` files, ensuring data integrity and performance.

**Imported Functions:**

- `read_form`: 
  - **Description:** Reads and retrieves data for a specific form from the archive.
  - **Usage:** Useful when you need to access detailed information of a single form by its identifier.

- `write_archive_skeleton`: 
  - **Description:** Initializes and creates a new archive structure.
  - **Usage:** Use this function when setting up a new `.smn` archive file before adding any data.

- `write_form`: 
  - **Description:** Writes or updates data for a specific form within the archive.
  - **Usage:** Ideal for inserting new form data or modifying existing information in the archive.

**Example Usage:**

```rust
use crate::core::io::{read_form, write_archive_skeleton, write_form};
use crate::core::structs::{Archive, FormID};

// Initialize a new archive
let archive = write_archive_skeleton("path/to/archive.smn")?;

// Write a new form to the archive
let form_id = FormID::new(1);
write_form(&archive, form_id, &form_data)?;
```
    
#### Structs Module

The `structs` submodule defines the various data structures used throughout the **SmnArchive** library. These structures represent the core entities and types that make up the archive and its forms.

**Imported Structures:**

- **Archive-Related:**
  - `Archive`: Represents the entire archive.
  - `ArchiveID`: Unique identifier for an archive.
  - `Version`: Specifies the version of the archive format.

- **Entity-Related:**
  - `EntID`: Identifier for an entity.
  - `EntInstance`: Represents an instance of an entity within the archive.

- **Form-Related:**
  - `FormID`: Unique identifier for a form.
  - `FormRefGroup`: Manages groups of references within forms.
  - `FormString`: Handles string-based form data.
  - `FormWeather`: Manages weather-related form data.
  - `FormWorld`: Represents world configuration forms.
  - `FormWorldPart`: Handles subdivisions or components of a world.

- **Global Identifiers:**
  - `GlobalID`: Unique identifier for global entities across archives.

- **Miscellaneous Types:**
  - `LangCode`: Specifies language codes for localization.
  - `SmlColor`: Represents small color data.
  - `StrLrg`: Handles large string data efficiently.
  - `StrSml`: Manages small string data.
  - `Vec3Float`: Represents 3D vectors with floating-point precision.
  - `Vec3Int`: Represents 3D vectors with integer precision.

**Example Usage:**

```rust
use crate::core::structs::{Archive, FormID, FormWorld};

// Load an existing archive
let archive = Archive::load("path/to/archive.smn")?;

// Access a specific form within the archive
let form_id = FormID::new(42);
if archive.get_form_exists(&form_id) {
    let form = archive.read_form(&form_id)?;
    
    // If the form is a World form, handle accordingly
    if let Some(world_form) = form.downcast_ref::<FormWorld>() {
        println!("World Name: {}", world_form.name);
    }
}
```
    
## Library Functionalities

The library supports various CRUD operations on the archive, categorized into **Archive** and **Forms** operations.

### Archive

These functions allow you to perform CRUD operations on the entire archive.

- **`write_archive_skeleton`**
  - **Purpose:** Creates a new archive structure.
  - **Usage Scenario:** Initializing a new `.smn` archive file before adding any data.

- **`read_archive_info`**
  - **Purpose:** Retrieves information about the archive.
  - **Usage Scenario:** Fetching metadata or summary details of the archive for display or processing.

- **`write_archive_info`**
  - **Purpose:** Writes or updates the archive's information.
  - **Usage Scenario:** Modifying metadata or updating summary details of the archive.

- **`read_lite_archive`**
  - **Purpose:** Retrieves a lightweight version of the archive, containing a list of simple form data.
  - **Usage Scenario:** Quick access to form identifiers and basic information without loading full form details.

### Forms

These functions handle CRUD operations for individual forms within the archive.

- **`read_form`**
  - **Purpose:** Retrieves data for a specific form.
  - **Usage Scenario:** Accessing detailed information of a single form based on its identifier.

- **`read_forms`**
  - **Purpose:** Retrieves data for multiple forms simultaneously.
  - **Usage Scenario:** Batch processing or loading multiple forms at once for efficiency.

- **`write_form`**
  - **Purpose:** Adds or updates data for a specific form.
  - **Usage Scenario:** Inserting new form data or modifying existing form information within the archive.

- **`delete_form`**
  - **Purpose:** Removes a specific form from the archive.
  - **Usage Scenario:** Deleting obsolete or unnecessary form data to maintain archive integrity.

- **`get_form_exists`**
  - **Purpose:** Checks if a specific form exists within the archive.
  - **Usage Scenario:** Validating the presence of a form before attempting read or write operations.

## Data Structures

The library includes various structures to safely and efficiently manage different types of forms and their content.

### Form Structures

These structures define the layout and components of different form types.

- **`struc_form`**
  - **Description:** The base form structure that serves as the foundation for all form types.
  - **Usage:** Inherited by other specific form structures to ensure consistency.

- **`struc_form_world`**
  - **Description:** Structure tailored for *WORLD* forms.
  - **Usage:** Managing and organizing data related to world configurations or environments.

- **`struc_form_string`**
  - **Description:** Structure for *STRING* forms.
  - **Usage:** Handling textual data or string-based information within the archive.

- **`struc_form_refgroup`**
  - **Description:** Structure for *REFGROUP* forms.
  - **Usage:** Managing groups of references, possibly for linking related data.

- **`struc_form_worldpart`**
  - **Description:** Structure for *WORLDPART* forms.
  - **Usage:** Handling subdivisions or components of a world within the archive.

### Type Structures

These structures define various data types used within forms, ensuring type safety and consistency.

- **`types_id`**
  - **Description:** Defines different ID types such as *ArchiveID*, *FormID*, and *GlobalID*.
  - **Usage:** Unique identifiers for archives, forms, and global entities.

- **`types_misc`**
  - **Description:** Defines miscellaneous types like *FormType* and *LangCode*.
  - **Usage:** Categorizing forms and specifying language codes for localization.

- **`types_str`**
  - **Description:** Defines different string types such as *StrSml* and *StrLrg*.
  - **Usage:** Handling small and large strings efficiently based on their size.

- **`types_vector`**
  - **Description:** Defines different vector types like *Vec2Int*, *Vec3Int*, *Vec2Float*, and *Vec3Float*.
  - **Usage:** Managing numerical data in 2D and 3D space with integer and floating-point precision.

## Usage Example

Below is a comprehensive example demonstrating how to initialize a new archive, add a form, and read it back.

```rust
use crate::core::{
    io::{read_form, write_archive_skeleton, write_form},
    structs::{
        Archive, ArchiveID, FormID, FormWorld, Vec3Float, Version,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a new archive
    let archive_path = "path/to/new_archive.smn";
    let archive = write_archive_skeleton(archive_path)?;

    // Step 2: Define a new World form
    let form_id = FormID::new(1001);
    let world_form = FormWorld {
        name: "New World".to_string(),
        position: Vec3Float::new(0.0, 0.0, 0.0),
        // Additional fields...
    };

    // Step 3: Write the World form to the archive
    write_form(&archive, form_id, &world_form)?;

    // Step 4: Read the form back from the archive
    if archive.get_form_exists(&form_id) {
        let retrieved_form = read_form(&archive, &form_id)?;
        if let Some(world) = retrieved_form.downcast_ref::<FormWorld>() {
            println!("Retrieved World Name: {}", world.name);
        }
    }

    Ok(())
}
```

## Further Resources

- **[SmnArchive - Type Structures - Binary File](./smnarchive/SmnArchive_Type_Structures_Binary_File.md)**
- **[SmnArchive - Type Structures - Values](./smnarchive/SmnArchive_Type_Structures_Values.md)**
- **[SmnArchive - Type Structures - Forms](./smnarchive/SmnArchive_Type_Structures_Forms.md)**
- **[SmnArchive - Glossary](./smnarchive/SmnArchive_Glossary.md)**
- **[SmnArchive - Statistics](./smnarchive/SmnArchive_Statistics.md)**

---
