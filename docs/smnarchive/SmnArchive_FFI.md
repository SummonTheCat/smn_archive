---
# FFI (Foreign Function Interface)

The **SmnArchive** library provides a robust Foreign Function Interface (FFI) that allows seamless integration with C and C++ applications. This enables developers to leverage the high-performance capabilities of **SmnArchive** within systems and applications written in these languages.

## Overview

The FFI layer exposes key functionalities of the **SmnArchive** library through C-compatible functions. These functions handle tasks such as creating and managing archives, reading and writing forms, and querying archive information. Memory management is carefully handled to ensure safe interoperability between Rust and C/C++.

## Included Functions

The FFI exposes the following functions:

- `smn_write_archive_skeleton`
- `smn_write_archive_info`
- `smn_read_archive_info`
- `smn_read_lite_archive`
- `smn_write_form`
- `smn_delete_form`
- `smn_get_form_exists`
- `smn_read_form`
- `smn_read_forms`
- `free_ptr`

Each function is documented in detail below, including descriptions, parameters, return values, and usage examples for both Rust and C/C++ ends.

---

## Function Documentation

### `smn_write_archive_skeleton`

**Description:**

Initializes a new archive by writing its skeleton structure. This function sets up the basic structure of the archive, including its ID, version, and description.

**Signature:**

```c
const uint8_t* smn_write_archive_skeleton(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path where the archive will be created.
- `archive_id` (`uint8_t`): 1-byte identifier for the archive.
- `version_major` (`uint8_t`): Major version number of the archive.
- `version_minor` (`uint8_t`): Minor version number of the archive.
- `description` (`const char*`): C-style string containing a description of the archive.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the success flag (always `1` byte).
  - The next byte (`uint8_t`): Success flag (`1` for success, `0` for failure).

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>

// Declare the FFI functions
const uint8_t* smn_write_archive_skeleton(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/new_archive.smn";
    uint8_t archive_id = 1;
    uint8_t version_major = 1;
    uint8_t version_minor = 0;
    const char* description = "Initial archive setup";

    // Create the archive skeleton
    const uint8_t* result = smn_write_archive_skeleton(archive_path, archive_id, version_major, version_minor, description);

    if (result == NULL) {
        printf("Failed to allocate memory for result.\n");
        return 1;
    }

    // Read the success flag
    uint8_t success = *(result + 4);

    if (success) {
        printf("Archive skeleton created successfully.\n");
    } else {
        printf("Failed to create archive skeleton.\n");
    }

    // Free the allocated memory
    free_ptr((void*)result);

    return 0;
}
```

---

### `smn_write_archive_info`

**Description:**

Writes or updates the archive's information, including its header details such as archive ID, version, and description.

**Signature:**

```c
const uint8_t* smn_write_archive_info(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `archive_id` (`uint8_t`): 1-byte identifier for the archive.
- `version_major` (`uint8_t`): Major version number of the archive.
- `version_minor` (`uint8_t`): Minor version number of the archive.
- `description` (`const char*`): C-style string containing a description of the archive.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the success flag (always `1` byte).
  - The next byte (`uint8_t`): Success flag (`1` for success, `0` for failure).

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>

// Declare the FFI functions
const uint8_t* smn_write_archive_info(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/existing_archive.smn";
    uint8_t archive_id = 1;
    uint8_t version_major = 1;
    uint8_t version_minor = 1;
    const char* description = "Updated archive description";

    // Update the archive info
    const uint8_t* result = smn_write_archive_info(archive_path, archive_id, version_major, version_minor, description);

    if (result == NULL) {
        printf("Failed to allocate memory for result.\n");
        return 1;
    }

    // Read the success flag
    uint8_t success = *(result + 4);

    if (success) {
        printf("Archive info updated successfully.\n");
    } else {
        printf("Failed to update archive info.\n");
    }

    // Free the allocated memory
    free_ptr((void*)result);

    return 0;
}
```

---

### `smn_read_archive_info`

**Description:**

Reads the archive's header information from the specified file and returns the serialized data.

**Signature:**

```c
const uint8_t* smn_read_archive_info(const char* path);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the archive data.
  - The subsequent bytes: Serialized archive information.

- Returns `NULL` if an error occurs.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_read_archive_info(const char* path);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";

    // Read the archive info
    const uint8_t* data = smn_read_archive_info(archive_path);

    if (data == NULL) {
        printf("Failed to read archive info.\n");
        return 1;
    }

    // Extract the length and archive info bytes
    uint32_t length = *(uint32_t*)data;
    const uint8_t* archive_info = data + 4;

    // Process the archive_info bytes as needed
    // For demonstration, we'll just print the length
    printf("Archive Info Length: %u bytes\n", length);

    // Free the allocated memory
    free_ptr((void*)data);

    return 0;
}
```
    
---

### `smn_read_lite_archive`

**Description:**

Retrieves a lightweight version of the archive, containing a list of simple form data without the full details. This is useful for quick access to form identifiers and basic information.

**Signature:**

```c
const uint8_t* smn_read_lite_archive(const char* path);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the lite archive data.
  - The subsequent bytes: Serialized lite archive information.

- Returns `NULL` if an error occurs.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_read_lite_archive(const char* path);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";

    // Read the lite archive
    const uint8_t* data = smn_read_lite_archive(archive_path);

    if (data == NULL) {
        printf("Failed to read lite archive.\n");
        return 1;
    }

    // Extract the length and lite archive bytes
    uint32_t length = *(uint32_t*)data;
    const uint8_t* lite_archive = data + 4;

    // Process the lite_archive bytes as needed
    // For demonstration, we'll just print the length
    printf("Lite Archive Length: %u bytes\n", length);

    // Free the allocated memory
    free_ptr((void*)data);

    return 0;
}
```

---

### `smn_write_form`

**Description:**

Writes a form's data to the specified archive file. The form data should be serialized as a byte array before being passed to this function.

**Signature:**

```c
const uint8_t* smn_write_form(const char* path, const uint8_t* form_data, size_t form_size);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `form_data` (`const uint8_t*`): Pointer to the serialized form data.
- `form_size` (`size_t`): Size of the `form_data` byte array.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the success flag (always `1` byte).
  - The next byte (`uint8_t`): Success flag (`1` for success, `0` for failure).

- Returns `NULL` if an error occurs during form serialization.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_write_form(const char* path, const uint8_t* form_data, size_t form_size);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";
    uint8_t form_data[] = { /* Serialized form data bytes */ };
    size_t form_size = sizeof(form_data);

    // Write the form to the archive
    const uint8_t* result = smn_write_form(archive_path, form_data, form_size);

    if (result == NULL) {
        printf("Failed to write form.\n");
        return 1;
    }

    // Read the success flag
    uint8_t success = *(result + 4);

    if (success) {
        printf("Form written successfully.\n");
    } else {
        printf("Failed to write form.\n");
    }

    // Free the allocated memory
    free_ptr((void*)result);

    return 0;
}
```

---

### `smn_delete_form`

**Description:**

Removes a specific form from the archive based on its `FormID`.

**Signature:**

```c
const uint8_t* smn_delete_form(const char* path, uint16_t form_id);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `form_id` (`uint16_t`): 2-byte identifier of the form to be deleted.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the success flag (always `1` byte).
  - The next byte (`uint8_t`): Success flag (`1` for success, `0` for failure).

- Returns `NULL` if an error occurs during the deletion process.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_delete_form(const char* path, uint16_t form_id);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";
    uint16_t form_id = 12345;

    // Delete the form from the archive
    const uint8_t* result = smn_delete_form(archive_path, form_id);

    if (result == NULL) {
        printf("Failed to delete form.\n");
        return 1;
    }

    // Read the success flag
    uint8_t success = *(result + 4);

    if (success) {
        printf("Form deleted successfully.\n");
    } else {
        printf("Failed to delete form.\n");
    }

    // Free the allocated memory
    free_ptr((void*)result);

    return 0;
}
```

---

### `smn_get_form_exists`

**Description:**

Checks whether a specific form exists within the archive based on its `FormID`.

**Signature:**

```c
const uint8_t* smn_get_form_exists(const char* path, uint16_t form_id);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `form_id` (`uint16_t`): 2-byte identifier of the form to check.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the result (always `1` byte).
  - The next byte (`uint8_t`): Existence flag (`1` if the form exists, `0` otherwise).

- Returns `NULL` if an error occurs during the check.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_get_form_exists(const char* path, uint16_t form_id);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";
    uint16_t form_id = 12345;

    // Check if the form exists in the archive
    const uint8_t* result = smn_get_form_exists(archive_path, form_id);

    if (result == NULL) {
        printf("Failed to check form existence.\n");
        return 1;
    }

    // Read the existence flag
    uint8_t exists = *(result + 4);

    if (exists) {
        printf("Form exists in the archive.\n");
    } else {
        printf("Form does not exist in the archive.\n");
    }

    // Free the allocated memory
    free_ptr((void*)result);

    return 0;
}
```

---

### `smn_read_form`

**Description:**

Retrieves a specific form's data from the archive based on its `FormID`. The form data is returned as a serialized byte array.

**Signature:**

```c
const uint8_t* smn_read_form(const char* path, uint16_t form_id);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `form_id` (`uint16_t`): 2-byte identifier of the form to be read.

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Length of the form data.
  - The subsequent bytes: Serialized form data.

- Returns `NULL` if the form does not exist or an error occurs during reading.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_read_form(const char* path, uint16_t form_id);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";
    uint16_t form_id = 12345;

    // Read the form from the archive
    const uint8_t* data = smn_read_form(archive_path, form_id);

    if (data == NULL) {
        printf("Failed to read form or form does not exist.\n");
        return 1;
    }

    // Extract the length and form data bytes
    uint32_t length = *(uint32_t*)data;
    const uint8_t* form_data = data + 4;

    // Process the form_data bytes as needed
    // For demonstration, we'll just print the length
    printf("Form Data Length: %u bytes\n", length);

    // Example: Print the first few bytes of form data
    for (uint32_t i = 0; i < (length < 10 ? length : 10); i++) {
        printf("%02X ", form_data[i]);
    }
    printf("\n");

    // Free the allocated memory
    free_ptr((void*)data);

    return 0;
}
```

---

### `smn_read_forms`

**Description:**

Retrieves multiple forms' data from the archive based on an array of `FormID`s. The forms' data are returned as a concatenated serialized byte array.

**Signature:**

```c
const uint8_t* smn_read_forms(const char* path, const uint8_t* form_ids);
```

**Parameters:**

- `path` (`const char*`): C-style string specifying the file path of the archive.
- `form_ids` (`const uint8_t*`): Pointer to a byte array containing the `FormID`s to be read. The first 2 bytes (`uint16_t`) represent the number of `FormID`s, followed by the `FormID`s themselves (each `FormID` is 2 bytes in Big Endian format).

**Return Value:**

- Returns a pointer to a byte array containing:
  - The first 4 bytes (`uint32_t`): Total length of the serialized forms' data.
  - The subsequent bytes: Concatenated serialized form data.

- Returns `NULL` if an error occurs during reading.

**Usage Example (C):**

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

// Declare the FFI functions
const uint8_t* smn_read_forms(const char* path, const uint8_t* form_ids);
void free_ptr(void* ptr);

int main() {
    const char* archive_path = "path/to/archive.smn";
    
    // Define the FormIDs to read
    uint16_t form_ids_count = 3;
    uint8_t form_ids_buffer[2 + (3 * 2)] = {0}; // 2 bytes for count + 3 FormIDs
    form_ids_buffer[0] = (form_ids_count >> 8) & 0xFF; // High byte of count
    form_ids_buffer[1] = form_ids_count & 0xFF;        // Low byte of count

    // Add FormIDs (Big Endian)
    form_ids_buffer[2] = 0x30; // FormID 12345: 0x3039
    form_ids_buffer[3] = 0x39;
    form_ids_buffer[4] = 0x5B; // FormID 23456: 0x5BA0
    form_ids_buffer[5] = 0xA0;
    form_ids_buffer[6] = 0x86; // FormID 34567: 0x86A7
    form_ids_buffer[7] = 0xA7;

    // Read the forms from the archive
    const uint8_t* data = smn_read_forms(archive_path, form_ids_buffer);

    if (data == NULL) {
        printf("Failed to read forms.\n");
        return 1;
    }

    // Extract the total length and forms' data bytes
    uint32_t total_length = *(uint32_t*)data;
    const uint8_t* forms_data = data + 4;

    // Process the forms_data bytes as needed
    // For demonstration, we'll just print the total length
    printf("Total Forms Data Length: %u bytes\n", total_length);

    // Example: Print the first few bytes of forms data
    for (uint32_t i = 0; i < (total_length < 10 ? total_length : 10); i++) {
        printf("%02X ", forms_data[i]);
    }
    printf("\n");

    // Free the allocated memory
    free_ptr((void*)data);

    return 0;
}
```

---

### `free_ptr`

**Description:**

Frees memory that was allocated by the FFI functions. It's essential to prevent memory leaks when using FFI.

**Signature:**

```c
void free_ptr(void* ptr);
```

**Parameters:**

- `ptr` (`void*`): Pointer to the memory to be freed. This should be a pointer returned by one of the FFI functions.

**Return Value:**

- None.

**Usage Example (C):**

```c
#include <stdlib.h>

// Declare the FFI functions
void free_ptr(void* ptr);

int main() {
    // Assume ptr was previously allocated by an FFI function
    void* ptr = malloc(100); // Example allocation

    // Use ptr for some operations...

    // When done, free the memory using the FFI function
    free_ptr(ptr);

    return 0;
}
```

---

## How It Works

The **SmnArchive** FFI functions are designed to bridge Rust's high-performance capabilities with C/C++ applications. Here's a brief overview of how they operate:

1. **Serialization and Deserialization:**
   - Forms and archive information are serialized into byte arrays in Rust. These byte arrays are then passed to C/C++ as pointers.
   - Similarly, C/C++ can pass serialized byte arrays to Rust functions for processing.

2. **Memory Management:**
   - Memory allocated by Rust (for return values) is managed carefully. After the C/C++ side finishes using the data, it must call `free_ptr` to release the allocated memory, preventing memory leaks.
   - The `free_ptr` function uses `libc::free` to deallocate memory, ensuring compatibility with C's memory management.

3. **Error Handling:**
   - Functions typically return `NULL` pointers in case of errors, allowing the C/C++ side to detect and handle failures gracefully.
   - Success flags (as `uint8_t`) are returned within the byte arrays to indicate the outcome of operations.

4. **Data Encoding:**
   - All multi-byte values are handled in Big Endian format to maintain consistency across different platforms.
   - C/C++ applications must ensure that data passed to and received from the FFI adheres to the expected byte orders and structures.

## How to Use FFI in C/C++ Projects

Integrating **SmnArchive** FFI into C/C++ projects involves the following steps:

1. **Include the FFI Header:**
   - Define the function signatures in your C/C++ project, typically using a header file. Ensure that the functions are declared with `extern "C"` linkage in C++ to prevent name mangling.

   ```c
   // smn_archive_ffi.h
   #ifndef SMN_ARCHIVE_FFI_H
   #define SMN_ARCHIVE_FFI_H

   #include <stdint.h>
   #include <stddef.h>

   #ifdef __cplusplus
   extern "C" {
   #endif

   const uint8_t* smn_write_archive_skeleton(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
   const uint8_t* smn_write_archive_info(const char* path, uint8_t archive_id, uint8_t version_major, uint8_t version_minor, const char* description);
   const uint8_t* smn_read_archive_info(const char* path);
   const uint8_t* smn_read_lite_archive(const char* path);
   const uint8_t* smn_write_form(const char* path, const uint8_t* form_data, size_t form_size);
   const uint8_t* smn_delete_form(const char* path, uint16_t form_id);
   const uint8_t* smn_get_form_exists(const char* path, uint16_t form_id);
   const uint8_t* smn_read_form(const char* path, uint16_t form_id);
   const uint8_t* smn_read_forms(const char* path, const uint8_t* form_ids);
   void free_ptr(void* ptr);

   #ifdef __cplusplus
   }
   #endif

   #endif // SMN_ARCHIVE_FFI_H
   ```
    
2. **Link Against the Rust Library:**
   - Compile the **SmnArchive** Rust library as a C-compatible shared library (`.dll`, `.so`, or `.dylib` depending on the platform).
   - Link your C/C++ project against this shared library.

   **Example (Linux):**

   In Rust, ensure you build the library as a C-compatible shared library:

   ```c
   cargo build --release
   ```
    
   In C/C++, compile and link against the shared library:

   ```c
   gcc -o my_app my_app.c -L/path/to/rust/lib -lsmnarchive
   ```
    
3. **Handle Data Serialization:**
   - Serialize and deserialize data according to the expected formats. Use appropriate byte order conversions if necessary.

   **Example: Serializing `FormID` in C:**

   ```c
   uint16_t form_id = 12345;
   uint8_t serialized_form_id[2];
   serialized_form_id[0] = (form_id >> 8) & 0xFF; // High byte
   serialized_form_id[1] = form_id & 0xFF;        // Low byte
   ```
    
4. **Manage Memory Appropriately:**
   - After receiving data from FFI functions, ensure to call `free_ptr` to release allocated memory.

   **Example:**

   ```c
   const uint8_t* data = smn_read_form(archive_path, form_id);
   if (data != NULL) {
       // Process data...

       // Free memory
       free_ptr((void*)data);
   }
   ```
    
5. **Error Checking:**
   - Always check for `NULL` pointers and success flags to handle errors gracefully.

   **Example:**

   ```c
   const uint8_t* result = smn_write_form(archive_path, form_data, form_size);
   if (result == NULL) {
       // Handle allocation or serialization error
   } else {
       uint8_t success = *(result + 4);
       if (success) {
           // Success
       } else {
           // Failure
       }
       free_ptr((void*)result);
   }
   ```
    
## Best Practices

- **Thread Safety:** Ensure that FFI functions are called in a thread-safe manner if your application is multi-threaded.
- **Data Alignment:** Maintain proper data alignment when dealing with serialized byte arrays to prevent undefined behavior.
- **Error Handling:** Implement comprehensive error handling on the C/C++ side to manage potential failures returned by FFI functions.
- **Memory Management:** Rigorously manage memory allocations and deallocations to avoid leaks and dangling pointers.

## Conclusion

The **SmnArchive** FFI empowers C and C++ developers to harness the performance and capabilities of the Rust-based **SmnArchive** library. By following the documentation and adhering to best practices, developers can seamlessly integrate archive management functionalities into their applications, ensuring efficient and reliable data handling.

For more detailed information on each function and advanced usage scenarios, refer to the specific function documentation sections above.

---
