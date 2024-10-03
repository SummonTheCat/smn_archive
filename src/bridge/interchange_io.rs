use std::{ffi::{c_void, CStr}, ptr, slice};

use crate::core::io::{delete_form, get_form_exists, read_archive_info, read_form, read_forms, read_lite_archive, write_archive_info, write_archive_skeleton, write_form};
use crate::core::structs::*;

/// Writes the skeleton of an archive, initializing the archive structure.
/// Parameters are passed as C-style strings and integers, and memory is allocated for the result.
#[no_mangle]
pub extern "C" fn smn_write_archive_skeleton(path: *const i8, archive_id: u8, version_major: u8, version_minor: u8, description: *const i8) -> *const u8 {
    // Convert C strings to Rust strings
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    let desc_c_str = unsafe { CStr::from_ptr(description) };
    let description_str = desc_c_str.to_str().unwrap_or("Invalid UTF-8");

    // Create an Archive struct with the provided parameters
    let archive = Archive::new(
        ArchiveID::from(archive_id),
        Version::from((version_major, version_minor)),
        StrLrg::from(description_str),
    );

    // Write the archive skeleton and capture success or failure
    let result = write_archive_skeleton(path_str, &archive);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // Allocate memory for the success flag and its length
    let result_len = std::mem::size_of::<u8>();
    let total_len = std::mem::size_of::<u32>() + result_len;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and success flag into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}

/// Writes archive info to a file, including the archive header.
#[no_mangle]
pub extern "C" fn smn_write_archive_info(path: *const i8, archive_id: u8, version_major: u8, version_minor: u8, description: *const i8) -> *const u8 {
    // Convert C strings to Rust strings
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    let desc_c_str = unsafe { CStr::from_ptr(description) };
    let description_str = desc_c_str.to_str().unwrap_or("Invalid UTF-8");

    // Create an Archive struct with the provided parameters
    let archive = Archive::new(
        ArchiveID::from(archive_id),
        Version::from((version_major, version_minor)),
        StrLrg::from(description_str),
    );

    // Write the archive info and capture success or failure
    let result = write_archive_info(path_str, &archive);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // Allocate memory for the success flag and its length
    let result_len = std::mem::size_of::<u8>();
    let total_len = std::mem::size_of::<u32>() + result_len;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and success flag into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}

/// Reads archive information from the specified file and returns a pointer to the data.
#[no_mangle]
pub extern "C" fn smn_read_archive_info(path: *const i8) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Attempt to read the archive info
    let result = read_archive_info(path_str);
    let archive_info = match result {
        Ok(info) => info,
        Err(_) => return ptr::null(),
    };

    // Convert the archive info to bytes
    let archive_bytes = archive_info.header_to_bytes();
    let len = archive_bytes.len() as u32;

    // Allocate memory for the length and archive bytes
    let total_len = std::mem::size_of::<u32>() + len as usize;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and archive bytes to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(archive_bytes.as_ptr(), len as usize);
    }

    ptr as *const u8
}

/// Reads a lightweight version of the archive and returns a pointer to the data.
#[no_mangle]
pub extern "C" fn smn_read_lite_archive(path: *const i8) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Read the LiteArchive and handle errors
    let lite_archive = match read_lite_archive(path_str) {
        Ok(archive) => archive,
        Err(_) => return ptr::null(),
    };

    // Convert the LiteArchive to bytes
    let archive_bytes = lite_archive.to_bytes();
    let len = archive_bytes.len() as u32;

    // Allocate memory for the length and archive bytes
    let total_len = std::mem::size_of::<u32>() + archive_bytes.len();
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and archive bytes to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(archive_bytes.as_ptr(), archive_bytes.len());
    }

    ptr as *const u8
}

/// Writes form data to a file and returns a success flag.
#[no_mangle]
pub extern "C" fn smn_write_form(path: *const i8, form_data: *const u8, form_size: usize) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Convert form data to a byte slice
    let form_slice = unsafe { std::slice::from_raw_parts(form_data, form_size) };

    // Parse the form and handle errors
    let read_form = match FormBase::read_from_byte_buffer(form_slice) {
        Ok(f) => f.0,
        Err(_) => return ptr::null(),
    };

    // Write the form to the file and check success
    let result = write_form(path_str, &*read_form);
    let was_successful = match result {
        Ok(_) => 1u8,
        Err(_) => 0u8,
    };

    // Prepare a response with the success flag
    let response_len = 5;
    let ptr = unsafe { libc::malloc(response_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and success flag to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = 1u32;
        *ptr.add(4) = was_successful;
    }

    ptr as *const u8
}

/// Deletes a form by its form ID from a specified file and returns a success flag.
#[no_mangle]
pub extern "C" fn smn_delete_form(path: *const i8, form_id: u16) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let form_id = FormID::from(form_id);

    // Attempt to delete the form and capture success or failure
    let result = delete_form(path_str, form_id);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // Allocate memory for the success flag
    let result_len = std::mem::size_of::<u8>();
    let total_len = std::mem::size_of::<u32>() + result_len;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and success flag into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}

/// Checks if a form exists in the specified file by its form ID.
#[no_mangle]
pub extern "C" fn smn_get_form_exists(path: *const i8, form_id: u16) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let form_id = FormID::from(form_id);

    // Check if the form exists and capture the result
    let form_exists = match get_form_exists(path_str, form_id) {
        Ok(exists) => exists,
        Err(_) => false,
    };

    // Allocate memory for the result
    let result_len = std::mem::size_of::<u8>();
    let total_len = std::mem::size_of::<u32>() + result_len;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and result into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = form_exists as u8;
    }

    ptr as *const u8
}

/// Reads a form by its form ID from a file and returns a pointer to the serialized form data.
#[no_mangle]
pub extern "C" fn smn_read_form(path: *const i8, form_id: u16) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    let form_id = FormID::from(form_id);

    // Attempt to read the form and handle errors
    let form = match read_form(path_str, form_id) {
        Ok(f) => f,
        Err(_) => return ptr::null(),
    };

    // Convert the form to bytes
    let form_bytes = form.to_bytes();
    let len = form_bytes.len() as u32;

    // Allocate memory for the length and form bytes
    let total_len = std::mem::size_of::<u32>() + len as usize;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and form bytes to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(form_bytes.as_ptr(), len as usize);
    }

    ptr as *const u8
}

/// Reads multiple forms by their IDs from a file and returns a pointer to the serialized form data.
#[no_mangle]
pub extern "C" fn smn_read_forms(path: *const i8, form_ids: *const u8) -> *const u8 {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Read the number of form IDs from the first 2 bytes
    let form_count = unsafe { *(form_ids as *const u16) } as usize;
    let form_id_bytes = unsafe { slice::from_raw_parts(form_ids.add(2), form_count * 2) };

    // Convert form ID bytes to a vector of FormIDs
    let mut form_ids_vec = Vec::with_capacity(form_count);
    for i in 0..form_count {
        let form_id = u16::from_be_bytes([form_id_bytes[i * 2], form_id_bytes[i * 2 + 1]]);
        form_ids_vec.push(FormID::from(form_id));
    }

    // Read the forms and handle errors
    let forms = match read_forms(path_str, form_ids_vec) {
        Ok(f) => f,
        Err(_) => return ptr::null(),
    };

    // Convert all forms to bytes and concatenate them
    let mut form_bytes = Vec::new();
    for form in forms {
        let form_data = form.to_bytes();
        form_bytes.extend_from_slice(&form_data);
    }

    let len = form_bytes.len() as u32;

    // Allocate memory for the length and form bytes
    let total_len = std::mem::size_of::<u32>() + len as usize;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write length and form bytes to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(form_bytes.as_ptr(), len as usize);
    }

    ptr as *const u8
}

/// Frees a pointer allocated by the FFI.
#[no_mangle]
pub extern "C" fn free_ptr(ptr: *mut c_void) {
    unsafe {
        if !ptr.is_null() {
            libc::free(ptr);
        }
    }
}
