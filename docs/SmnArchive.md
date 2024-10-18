# SmnArchive Documentation

Welcome to the **SmnArchive** library documentation! This guide will help you get started with using the library by explaining how to import and utilize its core modules effectively.

## Table of Contents

1. [Overview](#overview)
2. [Crate Module Imports](#crate-module-imports)
    - [Core Modules](#core-modules)
        - [IO Module](#io-module)
        - [Structs Module](#structs-module)
3. [Usage Example](#usage-example)
4. [Further Resources](#further-resources)

---

## Overview

**SmnArchive** is a robust library designed for managing `.smn` binary archive structures. It provides efficient real-time data streaming with a focus on high-performance reading and writing operations. The library offers comprehensive CRUD (Create, Read, Update, Delete) functionalities for handling archives and their constituent forms.

## Crate Module Imports

To leverage the full capabilities of the **SmnArchive** library, you need to import the necessary modules and components from the crate. Below is a reference snippet commonly used in main documentation:

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

The `core` module is the heart of the **SmnArchive** library, containing essential submodules and components required for archive management. It is divided into two primary submodules: `io` and `structs`.

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

Feel free to explore the detailed documentation linked above to gain a deeper understanding of each component and how to utilize the **SmnArchive** library effectively in your projects. If you have any questions or need further assistance, don't hesitate to reach out to the community or the development team.
