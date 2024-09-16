### Overview
This tool is designed for managing a custom archive format, optimized for game data streaming. The archives are stored in a custom, efficient binary format to ensure fast and smooth access to data during real-time applications.

- Each **archive** contains **archive info**, which includes metadata such as the archive ID, a description, and a version number.
- Archives also store **forms**, which encapsulate data specific to their form type. Each form represents a distinct unit of data, tailored to its role within the game.
- The tool supports reading from and writing to archives, with a focus on real-time data access to accommodate performance-critical environments like games.

---
### Usage
This project is designed to be flexible and can be integrated into both Rust and C++/C environments, providing robust control over custom archive formats.

- **Rust Library**: The core functionality of the project can be utilized directly as a Rust library, enabling developers to manage archives, archive data, and forms seamlessly within their Rust applications. This includes creating, reading, writing, and streaming custom archives with ease.
    
- **Dynamic Library for C++/C**: For non-Rust environments, the project is also available as a dynamic library. This allows integration with C++ or C projects, where you can manage and manipulate the custom archive format using the provided dynamic library API.
    
- **Extending the Project**: The core of the project is designed with customization in mind. Developers can extend and modify the archive format to suit specific needs. This includes:
    - Adding or customizing the **archive_data** to include additional metadata relevant to your use case.
    - Defining new **form types** or extending existing ones to handle different types of data.
    - Adapting the archive structure for specific performance optimizations or data handling requirements unique to your project.

---
## Command-Line Usage

The tool provides a command-line interface (CLI) that can be used for testing and development tasks via the `cargo run` command. The available commands are designed to help you run tests, build the project, and generate new form types.

### Running Commands

Commands are executed using the `cargo run` command, followed by the specific task and necessary arguments.

### Available Commands

#### 1. `test` – Run Tests

This command runs various test suites to validate the core functionality of the archive manager.

Usage:
```
cargo run -- test [test_type] [additional_arguments]
```

- **test_type**: Defines the type of test to run. If no test type is provided, core tests will be executed.
- **additional_arguments**: Optional, depending on the test type.

Available test types:

- `core`: Runs the core test suite to validate the basic functionality of archives, forms, and I/O operations.
- `sample`: Executes sample tests for specific use cases.
- `manyformsthreaded [r/w/rw/wr] [Form Count] [Thread Count]`: Executes threaded tests involving many forms, with options for read (`r`), write (`w`), or combined operations (`rw` or `wr`).

Examples:
```
# Run core tests
cargo run -- test core

# Run sample tests
cargo run -- test sample

# Run a multi-threaded read operation on 5000 forms with 8 threads
cargo run -- test manyformsthreaded r 5000 8

# Run a multi-threaded write operation on 2000 forms
cargo run -- test manyformsthreaded w 2000 4

# Run read-write operations on 1500 forms with 6 threads
cargo run -- test manyformsthreaded rw 1500 6
```

#### 2. `gen` – Generate Form Types

This command generates new form types for the archive system. You specify the name of the form type you want to generate, and it will be added to the project automatically.

Usage:
```
cargo run -- gen formtype [formtype_name]
```

- **formtype_name**: The name of the form type you want to create.
Example:
```
# Generate a new form type called "PlayerData"
cargo run -- gen formtype PlayerData
```

#### 3. `buildfull` – Full Build

This command triggers a complete build process for the project.

Usage:
```
cargo run -- buildfull
```

---
### Project Structure

- **src**
    
    - **bridge**: This module is responsible for FFI (Foreign Function Interface) to interact with C code. It provides the necessary functions to bridge Rust functionality into C-based applications.
        
        - `interchange_io.rs`
    - **core**
        
        - **io**: Handles all input/output operations, such as reading from and writing to archives.
            
            - **read**: Contains the modules for reading different parts of the archive.
                - `io_read_archive.rs`
                - `io_read_block.rs`
                - `io_read_form.rs`
            - **util**: Provides utility functions and structures that assist with various I/O operations.
                - `io_struct.rs`
                - `io_utils.rs`
            - **write**: Contains modules responsible for writing to archives, as well as deleting forms.
                - `io_delete_form.rs`
                - `io_write_archive.rs`
                - `io_write_block.rs`
                - `io_write_form.rs`
        - **structs**: Contains the main data structures used throughout the archive system.
            
            - **forms**: Represents different types of forms stored within the archive.
                - `struc_form_refgroup.rs`
                - `struc_form_string.rs`
                - `struc_form_world.rs`
                - `struc_form.rs`
            - **types**: Includes various type definitions used within the forms and archives.
                - `types_id.rs`
                - `types_misc.rs`
                - `types_str.rs`
            - `struc_archive.rs`: Defines the overall structure of the archive, including its metadata and contained forms.
    - **tooling**: Contains modules for automation, CLI interaction, and testing utilities to streamline project management.
        
        - **automation**: Provides scripts and tools for automating repetitive tasks related to building and managing form types within the archive.
            - `build.rs`
            - `formtype_management.rs`
        - **cli**: Contains the command-line interface logic, which allows the tool to be used via terminal commands.
            - `cmd.rs`
        - **testing**: Contains utilities and structures for testing the functionality of the project.



### Integration

#### **Rust Library Integration**

The project provides all functionality through **public Rust bindings**. This allows users to easily interact with archives, forms, and metadata using the core functions in the Rust crate.

- **Setup**: Add the crate to your `Cargo.toml` and import the necessary modules in your Rust application.
- **Functionality**: Use the provided public API to read, write, and modify archives, archive data, and forms.
- **Example Usage**:

```
use archive_tool::{Archive, Form};

let mut archive = Archive::new("path_to_archive");
archive.add_form("form_id", &form_data);
let data = archive.get_form("form_id");
```
#### **C++ Integration**

For non-Rust projects, you can integrate the tool into **C++ environments** by linking against the provided dynamic library (DLL) or by loading it at runtime.

1. **Linking to the DLL**:
    
    - You can either link directly to the library at compile time by including it in your build system (e.g., `CMake`, `Makefile`), or dynamically load the library during runtime using platform-specific mechanisms.
        
    - **Compile-time linking**: Link against the DLL or shared object file in your build system:
```
g++ -o my_program my_program.cpp -L/path_to_dll -lsmn_archive_extern

```

- **Runtime loading**: Use platform-specific functions to load the DLL dynamically:
    
    - On **Windows**: Use `LoadLibrary`.
    - On **Linux/macOS**: Use `dlopen`.

**Example for Linux/macOS**:
```
#include <dlfcn.h>  // For dynamic linking on Linux/macOS

void* DLLHandle = dlopen("path_to_your_dll.so", RTLD_LAZY); // Load the shared library
if (!DLLHandle) {
    std::cerr << "Failed to load the DLL" << std::endl;
}

```

**Define Function Pointers**: Once the DLL is linked or loaded, you can use function pointers to interact with its functionality, such as retrieving archive metadata, checking if forms exist, or reading specific forms.

Example function pointer definitions:
```
typedef const uint8_t* (*smn_get_form_exists_Fn)(const uint8_t* path, uint16_t form_id);
typedef const uint8_t* (*smn_read_archive_info_Fn)(const uint8_t* path);
typedef const uint8_t* (*smn_read_form_Fn)(const uint8_t* path, uint16_t form_id);

smn_get_form_exists_Fn smn_get_form_exists = (smn_get_form_exists_Fn)dlsym(DLLHandle, "smn_get_form_exists");
smn_read_archive_info_Fn smn_read_archive_info = (smn_read_archive_info_Fn)dlsym(DLLHandle, "smn_read_archive_info");
smn_read_form_Fn smn_read_form = (smn_read_form_Fn)dlsym(DLLHandle, "smn_read_form");

```

**Managing Archives**: Create a manager class to handle loading/unloading archives and interacting with the DLL functions:
```
class ArchiveManager {
public:
    ArchiveManager();
    ~ArchiveManager();
    void AddArchive(uint8_t archiveID, const std::string& path);
    std::vector<uint8_t> GetBytesFormExists(uint8_t archiveID, uint16_t form_id);
    std::vector<uint8_t> GetBytesArchiveInfo(uint8_t archiveID);
    std::vector<uint8_t> GetBytesForm(uint8_t archiveID, uint16_t form_id);
    void LoadDLL(const std::string& dll_path);
    void UnloadDLL();

private:
    void* DLLHandle;
};

```

**Loading and Unloading the DLL**: Implement methods to load the DLL and clean up by unloading it when done:
```
void ArchiveManager::LoadDLL(const std::string& dll_path) {
    DLLHandle = dlopen(dll_path.c_str(), RTLD_LAZY);  // Load the shared library
    if (!DLLHandle) {
        std::cerr << "Failed to load the DLL" << std::endl;
    }
}

void ArchiveManager::UnloadDLL() {
    if (DLLHandle) {
        dlclose(DLLHandle);  // Unload the shared library
        DLLHandle = nullptr;
    }
}

```

**Reading Data and Managing Pointers**: When reading raw bytes from the DLL, the first 4 bytes represent the length of the data, and the remaining bytes are the actual data. After reading, you can free the pointer or use smart pointers for better memory management.

**Example Implementation**:
```
std::vector<uint8_t> ArchiveManager::CopyBytesAndFree(const uint8_t* result) {
    std::vector<uint8_t> newData;

    if (result) {
        // First 4 bytes represent the length of the data
        uint32_t length = *((uint32_t*)result);
        const uint8_t* data = result + sizeof(uint32_t);

        // Resize the vector to hold the data
        newData.resize(length);
        std::copy(data, data + length, newData.begin());

        // Free the original pointer
        free_ptr((void*)result);
    }

    return newData;
}

```

**Example Method for Fetching Form Data**:

- Once the DLL is loaded, you can interact with the archives using the function pointers:
```
std::vector<uint8_t> ArchiveManager::GetBytesForm(uint8_t archiveID, uint16_t form_id) {
    // Retrieve the archive path and call the appropriate function via the function pointer
    const uint8_t* result = smn_read_form("archive_path", form_id);
    // Process and return the data as a byte array
    return CopyBytesAndFree(result);
}

```

This setup allows you to integrate and manage custom archives in C++ projects by either statically linking or dynamically loading the provided DLL. It offers flexibility for handling raw data, while managing memory safely by freeing the pointers or using smart pointers.

## Extending & Customization

Developers are encouraged to extend the project by:

- Adding or modifying the `archive_data` fields to suit specific metadata requirements.
- Creating new form types in the `forms` directory to handle different kinds of data.
- Customizing the binary structure to improve performance for unique workloads or scenarios.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or raise issues.