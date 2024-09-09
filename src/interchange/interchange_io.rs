use std::{ffi::{c_void, CStr}, ptr};

use crate::archive_tools::{io::{delete_form, get_form_exists, read_archive_info, read_form, read_lite_archive, write_archive_info, write_archive_skeleton, write_form}, structs::{Archive, FormBase, ArchiveID, FormID, StrLrg, Version}};


#[no_mangle]
pub extern "C" fn smn_write_archive_skeleton(path: *const i8, archive_id: u8, version_major: u8, version_minor: u8, description: *const i8) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Convert the description from a raw pointer to a string
    let desc_c_str = unsafe { CStr::from_ptr(description) };
    let description_str = desc_c_str.to_str().unwrap_or("Invalid UTF-8");


    // Write the archive skeleton to the specified path
    let archive = Archive::new(
        ArchiveID::from(archive_id),
        Version::from((version_major, version_minor)),
        StrLrg::from(description_str),
    );

    let result = write_archive_skeleton(path_str, &archive);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // The size of the bool result in bytes
    let result_len = std::mem::size_of::<u8>();

    // Total size: 4 bytes for the length + 1 byte for the bool
    let total_len = std::mem::size_of::<u32>() + result_len;

    // Allocate memory for the length and the bool
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length (1 byte for bool) into the first 4 bytes of the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        
        // Copy the bool value into the memory after the length
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}



#[no_mangle]
pub extern "C" fn smn_write_archive_info(path: *const i8, archive_id: u8, version_major: u8, version_minor: u8, description: *const i8) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Convert the description from a raw pointer to a string
    let desc_c_str = unsafe { CStr::from_ptr(description) };
    let description_str = desc_c_str.to_str().unwrap_or("Invalid UTF-8");


    // Construct the new archive
    let archive = Archive::new(
        ArchiveID::from(archive_id),
        Version::from((version_major, version_minor)),
        StrLrg::from(description_str),
    );

    // Write the archive info to the specified path
    let result = write_archive_info(path_str, &archive);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // The size of the bool result in bytes
    let result_len = std::mem::size_of::<u8>();

    // Total size: 4 bytes for the length + 1 byte for the bool
    let total_len = std::mem::size_of::<u32>() + result_len;

    // Allocate memory for the length and the bool
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length (1 byte for bool) into the first 4 bytes of the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        
        // Copy the bool value into the memory after the length
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}


#[no_mangle]
pub extern "C" fn smn_read_archive_info(path: *const u8) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");


    // Attempt to read the archive info from the specified path
    let result = read_archive_info(path_str);
    let archive_info = match result {
        Ok(info) => info,
        Err(_) => return ptr::null(),
    };

    // Convert the archive_info to bytes using the header_to_bytes method
    let archive_bytes = archive_info.header_to_bytes();
    let len = archive_bytes.len() as u32;

    // Allocate memory for the length and the archive bytes
    let total_len = std::mem::size_of::<u32>() + len as usize;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;

        // Copy the archive bytes into the allocated memory after the length
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(archive_bytes.as_ptr(), len as usize);
    }

    // Return the pointer to the caller
    ptr as *const u8
}


#[no_mangle]
pub extern "C" fn smn_read_lite_archive(path: *const u8) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");


    // Read the LiteArchive from the file
    let lite_archive = match read_lite_archive(path_str) {
        Ok(archive) => archive,
        Err(_) => return ptr::null(),
    };

    // Convert the LiteArchive to bytes
    let archive_bytes = lite_archive.to_bytes();
    let len = archive_bytes.len() as u32;

    // Allocate memory for the length and the archive bytes
    let total_len = std::mem::size_of::<u32>() + archive_bytes.len();
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length (4 bytes) into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;

        // Copy the archive bytes into the memory after the length
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(archive_bytes.as_ptr(), archive_bytes.len());
    }

    // Return the pointer to the allocated memory
    ptr as *const u8
}



#[no_mangle]
pub extern "C" fn smn_write_form(path: *const u8, form_data: *const u8, form_size: usize) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    // Convert the form data from a raw pointer to a slice
    let form_slice = unsafe { std::slice::from_raw_parts(form_data, form_size) };


    // Parse the form using FormBase's read_from_byte_buffer method
    let read_form = match FormBase::read_from_byte_buffer(form_slice) {
        Ok(f) => f.0, // Extract the concrete form type (e.g., FormString)
        Err(_) => return ptr::null(),
    };

    // Dereference the Box to pass a &dyn FormTrait
    let result = write_form(path_str, &*read_form);
    let was_successful = match result {
        Ok(_) => 1u8, // Success is indicated by 1
        Err(_) => 0u8, // Failure is indicated by 0
    };

    // Prepare a response with 4 length bytes followed by the success flag
    let response_len = 5; // 4 bytes for length + 1 byte for success flag
    let ptr = unsafe { libc::malloc(response_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Write the length and the success flag to the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = 1u32; // The length is 1 byte for the success flag
        *ptr.add(4) = was_successful; // Set the success flag
    }

    // Return the pointer to the caller
    ptr as *const u8
}



#[no_mangle]
pub extern "C" fn smn_delete_form(path: *const u8, form_id: u16) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let form_id = FormID::from(form_id);

    let result = delete_form(path_str, form_id);
    let was_successful = match result {
        Ok(_) => true,
        Err(_) => false,
    };

    // The size of the bool result in bytes
    let result_len = std::mem::size_of::<u8>();

    // Total size: 4 bytes for the length + 1 byte for the bool
    let total_len = std::mem::size_of::<u32>() + result_len;

    // Allocate memory for the length and the bool
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length (1 byte for bool) into the first 4 bytes of the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        
        // Copy the bool value into the memory after the length
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = was_successful as u8;
    }

    ptr as *const u8
}


#[no_mangle]
pub extern "C" fn smn_get_form_exists(path: *const u8, form_id: u16) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");
    let form_id = FormID::from(form_id);

    // Directly assign the result to form_exists using the match expression
    let form_exists = match get_form_exists(path_str, form_id) {
        Ok(exists) => exists,
        Err(_) => false,
    };

    // The size of the bool result in bytes
    let result_len = std::mem::size_of::<u8>();

    // Total size: 4 bytes for the length + 1 byte for the bool
    let total_len = std::mem::size_of::<u32>() + result_len;

    // Allocate memory for the length and the bool
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length (1 byte for bool) into the first 4 bytes of the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = result_len as u32;
        
        // Copy the bool value into the memory after the length
        let bool_ptr = ptr.add(std::mem::size_of::<u32>());
        *bool_ptr = form_exists as u8;
    }

    ptr as *const u8
}



#[no_mangle]
pub extern "C" fn smn_read_form(path: *const u8, form_id: u16) -> *const u8 {
    // Convert the path from a raw pointer to a string
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    let path_str = c_str.to_str().unwrap_or("Invalid UTF-8");

    let form_id = FormID::from(form_id);

    let form = match read_form(path_str, form_id) {
        Ok(f) => f,
        Err(_) => return ptr::null(),
    };

    let form_bytes = form.to_bytes();

    let len = form_bytes.len() as u32;

    // Allocate memory for the length and the form bytes
    let total_len = std::mem::size_of::<u32>() + len as usize;
    let ptr = unsafe { libc::malloc(total_len) as *mut u8 };
    if ptr.is_null() {
        return ptr::null();
    }

    // Copy the length into the allocated memory
    unsafe {
        let len_ptr = ptr as *mut u32;
        *len_ptr = len;
        // Copy the form bytes into the allocated memory after the length
        let data_ptr = ptr.add(std::mem::size_of::<u32>());
        data_ptr.copy_from_nonoverlapping(form_bytes.as_ptr(), len as usize);
    }

    ptr as *const u8
}


#[no_mangle]
pub extern "C" fn free_form(ptr: *mut c_void) {
    unsafe {
        if !ptr.is_null() {
            libc::free(ptr);
        }
    }
}
